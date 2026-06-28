# Moneyman — System Architecture

## Overview

Moneyman is a desktop-native double-entry accounting application built on **Tauri v2** (Rust backend) with a **Vue 3** frontend styled by **Tailwind CSS** and **shadcn-vue**. It emulates GnuCash's local-first philosophy while providing a modern, keyboard-navigable, dark-mode-ready UI.

---

## High-Level Data Flow

```
┌──────────────────────────────────────────────────────────┐
│                    Vue 3 Frontend                         │
│  ┌──────────┐  ┌────────────┐  ┌──────────────────────┐  │
│  │  Views   │──│  Pinia     │──│  Composables          │  │
│  │ (Router) │  │  Stores    │  │  (useAccountingEngine) │  │
│  └────┬─────┘  └─────┬──────┘  └──────────┬───────────┘  │
│       │              │                     │              │
│       └──────────────┼─────────────────────┘              │
│                      │                                    │
│              ┌───────▼────────┐                           │
│              │  Tauri invoke  │  (IPC bridge)             │
│              └───────┬────────┘                           │
├──────────────────────┼────────────────────────────────────┤
│                 Tauri Rust Core                           │
│  ┌───────────────────▼──────────────────────────────┐     │
│  │              Tauri Commands                       │     │
│  │  (accounts::create, transactions::post, etc.)     │     │
│  └───────────────────┬──────────────────────────────┘     │
│                      │                                    │
│  ┌───────────────────▼──────────────────────────────┐     │
│  │           Accounting Engine (Rust)                 │     │
│  │  • Double-entry validation                         │     │
│  │  • Split transaction balancing                     │     │
│  │  • Account tree rollup calculations                │     │
│  │  • Fixed-point / Decimal arithmetic                │     │
│  │  • Currency conversion & Trading accounts          │     │
│  └───────────────────┬──────────────────────────────┘     │
│                      │                                    │
│  ┌───────────────────▼──────────────────────────────┐     │
│  │         SQLite (via rusqlite)                     │     │
│  │  • Hierarchical accounts (adjacency list)         │     │
│  │  • Multi-split transactions                       │     │
│  │  • Invoices, contacts, budgets                    │     │
│  └──────────────────────────────────────────────────┘     │
└────────────────────────────────────────────────────────────┘
```

### Data Flow Steps

1. **User Interaction** — User fills a transaction form in a Vue view.
2. **Pinia Store** — The store calls a composable method (e.g., `useAccountingEngine().postTransaction()`).
3. **Precision Validation** — The composable runs client-side double-entry math using `decimal.js` to ensure debits === credits.
4. **Tauri IPC** — The validated payload is sent to the Rust core via `invoke('post_transaction', { data })`.
5. **Rust Validation** — Rust re-validates (belt-and-suspenders) using its own `rust_decimal` crate.
6. **SQLite Persist** — The validated transaction is written to SQLite. Account balances are updated.
7. **Rollup Propagation** — A recursive CTE recalculates parent account totals up the account tree.
8. **Response** — The updated account tree + new transaction are returned to the frontend.
9. **Store Update** — Pinia replaces its reactive state with the returned data. Vue reactivity updates the UI.

---

## Directory Structure

```
src/
├── assets/
│   └── css/
│       └── main.css              # Tailwind base + shadcn-vue CSS variables
├── components/
│   └── ui/                        # shadcn-vue primitives (Button, Card, Table, etc.)
│       ├── Button.vue
│       ├── Card.vue
│       ├── Table.vue / TableHeader.vue / ...
│       ├── Dialog.vue
│       ├── Input.vue
│       ├── Label.vue
│       ├── Badge.vue
│       ├── Select.vue
│       └── SelectItem.vue
├── composables/
│   ├── useAccountingEngine.ts     # Core double-entry math & validation
│   ├── useCurrencyConverter.ts    # Multi-currency conversion engine
│   └── useRecurringTransactions.ts # Scheduler for recurring txns
├── layouts/
│   └── DefaultLayout.vue          # Sidebar + topbar + <router-view />
├── lib/
│   └── utils.ts                   # cn() helper
├── router/
│   └── index.ts                   # Vue Router configuration
├── stores/
│   ├── accountStore.ts            # Chart of Accounts state
│   ├── transactionStore.ts        # Transaction + split state
│   ├── uiStore.ts                 # Dark mode, sidebar, modals
│   └── settingsStore.ts           # App preferences
├── types/
│   ├── account.ts                 # Account, AccountType, AccountTree
│   ├── transaction.ts             # Transaction, Split, RecurringRule
│   ├── currency.ts                # Currency, ExchangeRate
│   ├── invoice.ts                 # Invoice, InvoiceLine, Contact
│   └── index.ts                   # Re-exports
├── utils/
│   └── decimal.ts                 # decimal.js helper (toCents, fromCents, format)
├── views/
│   ├── Dashboard.vue              # Net worth, budget, cash flow overview
│   ├── Accounts.vue               # Chart of Accounts hierarchy
│   ├── Ledger.vue                 # Checkbook register / general ledger
│   ├── Reconciliation.vue         # Bank reconciliation view
│   └── Reports.vue                # Balance Sheet, P&L, etc.
├── App.vue                        # Root component
└── main.ts                        # Entry point
```

---

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **decimal.js** on frontend | Pure JS, no WASM dependency, arbitrary precision, proven in financial apps |
| **rusqlite** on backend | Embeddable, zero-config, battle-tested for local-first desktop apps |
| **Adjacency list** for accounts | Simple tree queries via recursive CTEs; easy inserts/moves |
| **Split transactions** | One `transaction` row → N `splits` rows; each split has `amount` (positive for debit, negative for credit) |
| **Balance rollups via CTE** | Recursive SQL computes parent totals on every mutation; avoids stale caches |
| **Tauri v2** | Lightweight (< 10MB binary), secure (CSP), native performance |
| **Pinia** | Official Vue 3 state management; devtools support |
| **shadcn-vue** | Copy-paste components you own; full control over styling |

---

## Accounting Model

```
Account Types (GnuCash-compatible):
  ROOT
  ├── ASSET
  │   ├── BANK
  │   ├── CASH
  │   ├── INVESTMENT
  │   │   ├── STOCK
  │   │   └── MUTUAL_FUND
  │   └── RECEIVABLE
  ├── LIABILITY
  │   ├── CREDIT_CARD
  │   └── PAYABLE
  ├── EQUITY
  ├── INCOME
  └── EXPENSE

Every transaction must satisfy: Σ debits = Σ credits
```

---

## Security & Local-First

- All data lives in a local SQLite file (`~/.moneyman/data.db`).
- No network calls except optional price quote syncing (user-opt-in).
- Tauri's CSP restricts the renderer to local assets only.
- File attachments (receipts) stored in app data dir; accessed via Tauri's native dialog.
