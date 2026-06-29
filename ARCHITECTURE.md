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
3. **Balance Validation** — The store ensures debits === credits (integer cents) before sending.
4. **Tauri IPC** — The validated payload is sent to the Rust core via `invoke('post_transaction', { payload })`.
5. **Rust Validation** — Rust re-validates (belt-and-suspenders) using strict integer math.
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
│   ├── ui/                        # shadcn-vue primitives (Button, Card, Table, etc.)
│   │   ├── Accordion.vue
│   │   ├── Badge.vue
│   │   ├── Button.vue
│   │   ├── Card.vue / Card*.vue
│   │   ├── Dialog.vue
│   │   ├── Input.vue / Label.vue
│   │   ├── Select.vue / SelectItem.vue
│   │   ├── Table.vue / Table*.vue
│   │   ├── calendar/              # Reka UI Calendar wrapper
│   │   ├── command/               # Command palette primitives
│   │   ├── popover/               # Popover primitives
│   │   ├── tabs/                  # Tabs primitives
│   │   └── index.ts               # Barrel exports
│   ├── AccountPicker.vue          # Modal account selector with tabs + drill-down
│   ├── AccountTree.vue            # Recursive hierarchy with accordion
│   ├── Combobox.vue               # Searchable select for enums
│   ├── ConfirmDialog.vue          # Toast-based confirmation prompt
│   └── LedgerTable.vue            # TanStack Vue Table split-ledger
├── composables/
│   ├── useAccountingEngine.ts     # Core double-entry math & validation
│   ├── useConfirm.ts              # Toast-based confirm dialog
│   ├── useCurrencyConverter.ts    # Multi-currency conversion engine
│   └── useRecurringTransactions.ts # Scheduler for recurring txns
├── layouts/
│   └── DefaultLayout.vue          # Collapsible sidebar + header + nav stack
├── lib/
│   └── utils.ts                   # cn() helper (clsx + tailwind-merge)
├── router/
│   └── index.ts                   # Vue Router (16 routes)
├── stores/
│   ├── accountStore.ts            # Chart of Accounts state + treeWithRollup
│   ├── importStore.ts             # Import wizard state machine
│   ├── ledgerStore.ts             # Ledger account context labels
│   ├── reconciliationStore.ts     # Reconciliation session + difference calc
│   ├── settingsStore.ts           # App preferences
│   ├── transactionStore.ts        # Transactions, filters, pagination
│   └── uiStore.ts                 # Theme, sidebar, modals
├── types/
│   ├── account.ts                 # Account, AccountNode, AccountType
│   ├── currency.ts                # Currency, ExchangeRate
│   ├── import.ts                  # ImportProfile, ImportPreviewItem, ColumnMapping
│   ├── invoice.ts                 # Invoice, InvoiceLine, Contact
│   ├── reconciliation.ts          # ReconciliationSession, ReconcileSplit
│   ├── transaction.ts             # Transaction, Split, RecurringTransaction
│   └── index.ts                   # Re-exports
├── utils/
│   └── decimal.ts                 # decimal.js helpers (toCents, fromCents, format)
├── views/
│   ├── AccountDetail.vue          # Account detail view
│   ├── AccountNew.vue             # Create/edit account form
│   ├── Accounts.vue               # Chart of Accounts hierarchy with edit/delete
│   ├── Budgets.vue                # Budget management
│   ├── Contacts.vue               # Contact management
│   ├── Dashboard.vue              # Net worth, budget, cash flow overview
│   ├── ImportWizard.vue           # 4-step CSV import wizard
│   ├── Invoices.vue               # Invoice/bill tracking
│   ├── Ledger.vue                 # Checkbook register / general ledger
│   ├── Reconciliation.vue        # Bank reconciliation split-pane
│   ├── Reports.vue                # Balance Sheet, P&L, etc.
│   ├── ScheduledTransactions.vue  # Recurring transaction rules
│   ├── Settings.vue               # App preferences
│   └── TransactionNew.vue         # Create split transaction
├── App.vue                        # Root component
└── main.ts                        # Entry point (Pinia + Router)
```

---

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Integer cents** for money | Eliminates floating-point risk; `i64` in Rust, `number` on frontend; simple arithmetic |
| **rusqlite** on backend | Embeddable, zero-config, battle-tested for local-first desktop apps |
| **Adjacency list** for accounts | Simple tree queries via recursive CTEs; easy inserts/moves |
| **Split transactions** | One `transaction` row → N `splits` rows; each split has `debit`/`credit` in cents |
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
