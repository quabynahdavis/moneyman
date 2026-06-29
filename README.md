# Moneyman

> Desktop-native, open-source, double-entry accounting — a modern GnuCash clone powered by **Tauri v2**, **Vue 3**, **shadcn-vue**, and **SQLite**.

---

## Features

- **Double-Entry Accounting Engine** — Every transaction is validated for balance (debits = credits) using strict integer cents.
- **Split Transactions** — Model complex receipts and invoices across multiple accounts, assets, and categories.
- **Chart of Accounts** — Hierarchical tree with recursive balance rollups (Assets, Liabilities, Equity, Income, Expenses).
- **General Ledger** — Sortable, filterable, paginated register with expandable split rows and context menus.
- **Account Picker** — Modal with tabbed hierarchy, drill-down navigation, and global search.
- **Calendar Date Picker** — Self-contained calendar widget for transaction date entry.
- **Reconciliation Engine** — Match ledger entries against bank statements with live difference tracking.
- **Import Automation** — 4-step CSV import wizard (upload → map columns → preview → commit) with duplicate detection.
- **Multi-Currency** — Track accounts in distinct fiat currencies with historical exchange rates and trading accounts.
- **Investments** — Stock and mutual fund accounts with cost-basis tracking and price quote syncing.
- **Invoicing** — Full A/R and A/P lifecycle: create, send, track, and reconcile invoices and bills.
- **Budgeting** — Monthly/quarterly/annual spending targets with actual-vs-budget tracking.
- **Scheduled Transactions** — Recurring rules for bills and income with auto-execute or prompt-on-startup.
- **Reports** — Balance Sheet, Profit & Loss, Cash Flow, Portfolio Valuation.
- **Dark Mode** — Light/Dark/System theme selector via sidebar popover with persistent preference.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop Shell | [Tauri v2](https://v2.tauri.app) (Rust) |
| Frontend | [Vue 3](https://vuejs.org) (Composition API, `<script setup>`, TypeScript) |
| UI Kit | [shadcn-vue](https://www.shadcn-vue.com) + [Tailwind CSS](https://tailwindcss.com) |
| State | [Pinia](https://pinia.vuejs.org) |
| Routing | [vue-router](https://router.vuejs.org) |
| Table | [@tanstack/vue-table](https://tanstack.com/table/v8) |
| Icons | [Lucide](https://lucide.dev) |
| Monetary Math | Integer cents (`i64`/`number`) |
| Dialogs | [Reka UI](https://reka-ui.com) / [vaul-vue](https://vaul.emilkowal.ski) |
| Persistence | SQLite (via Tauri's `tauri-plugin-sql` or `rusqlite`) |
| Database Schema | See [database-schema.sql](./database-schema.sql) |

---

## Getting Started

```bash
# Install dependencies
pnpm install

# Run in dev mode (Vite dev server)
pnpm tauri dev

# Build for production
pnpm tauri build
```

The Vite dev server runs on `http://localhost:1420`. Tauri wraps it in a native window.

---

## Project Structure

```
src/
├── assets/css/main.css          # Tailwind base + CSS variables
├── components/
│   ├── ui/                      # shadcn-vue primitives (Accordion, Button, Card, Calendar, Command, Dialog, Input, Label, Popover, Table, Tabs, Select)
│   │   └── index.ts             # barrel exports
│   ├── AccountPicker.vue        # Modal account selector with tabs + drill-down
│   ├── AccountTree.vue          # Recursive hierarchy with accordion + context menus
│   ├── Combobox.vue             # Searchable select for enums
│   ├── ConfirmDialog.vue        # Toast-based confirmation prompt
│   └── LedgerTable.vue          # TanStack table with expandable splits
├── composables/
│   ├── useAccountingEngine.ts   # Double-entry validation, tree builder
│   └── useConfirm.ts            # `vue-sonner` toast-based confirm dialog
├── layouts/
│   └── DefaultLayout.vue        # Collapsible sidebar + nav stack + theme popover
├── lib/utils.ts                 # cn() helper (clsx + tailwind-merge)
├── router/index.ts              # 16 routes (lazy-loaded)
├── stores/
│   ├── accountStore.ts          # Chart of Accounts state + treeWithRollup
│   ├── importStore.ts           # Import wizard state machine
│   ├── ledgerStore.ts           # Ledger account context labels
│   ├── reconciliationStore.ts   # Reconciliation session + difference equation
│   ├── settingsStore.ts         # App preferences (localStorage)
│   ├── transactionStore.ts      # Transactions, filters, pagination
│   └── uiStore.ts               # Theme, sidebar, modals
├── types/
│   ├── account.ts               # Account, AccountNode, AccountType
│   ├── currency.ts              # Currency, ExchangeRate
│   ├── import.ts                # ImportProfile, ImportPreviewItem, ColumnMapping
│   ├── invoice.ts               # Invoice, InvoiceLine, Contact
│   ├── reconciliation.ts        # ReconciliationSession, ReconcileSplit
│   ├── transaction.ts           # Transaction, Split, RecurringTransaction
│   └── index.ts                 # Re-exports
├── utils/decimal.ts             # decimal.js helpers (formatMoney, toCents, etc.)
├── views/
│   ├── AccountDetail.vue
│   ├── AccountNew.vue           # Dual create/update via editId query param
│   ├── Accounts.vue             # Hierarchical account tree
│   ├── Budgets.vue
│   ├── Contacts.vue
│   ├── Dashboard.vue
│   ├── ImportWizard.vue         # 4-step CSV import (upload→map→preview→commit)
│   ├── Invoices.vue
│   ├── Ledger.vue               # General ledger / register
│   ├── Reconciliation.vue       # Split-pane bank reconciliation
│   ├── Reports.vue
│   ├── ScheduledTransactions.vue
│   ├── Settings.vue
│   └── TransactionNew.vue       # Split transaction form with date picker + account picker
├── App.vue
└── main.ts                      # Entry point (Pinia + Router)
```

---

## Documentation

Detailed documentation for every subsystem is in the [`docs/`](./docs/) folder:

| Document | Covers |
|---|---|---|
| [Architecture](./docs/architecture/) | Data flow, Tauri ↔ Vue IPC, design decisions |
| [Database Schema](./docs/database/) | All tables, relationships, indexes, views |
| [UI Components](./docs/ui-components/) | shadcn-vue components, usage, props |
| [Accounting Engine](./docs/accounting-engine/) | Double-entry math, split validation, balance rollups |
| [Transactions](./docs/transactions/) | Transaction structure, split model, states, validation, data flow |
| [Stores](./docs/stores/) | Pinia stores API, state shape, computed values |
| [Types](./docs/types/) | TypeScript interfaces, type guards, utilities |
| [Ledger Table](./docs/ledger-table/) | TanStack table configuration, column definitions, expandable rows |
| [Views](./docs/views/) | Each view's purpose, state dependencies, interactions |
| [Router & Layout](./docs/router-layout/) | Route definitions, sidebar navigation, dark mode |
| [Decimal Utilities](./docs/decimal-utils/) | Precision math helpers, formatMoney, comparisons |
| [Tauri Integration](./docs/tauri-integration/) | IPC commands, SQLite backend, native features |

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

---

## License

MIT
