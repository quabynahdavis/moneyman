# Moneyman

> Desktop-native, open-source, double-entry accounting — a modern GnuCash clone powered by **Tauri v2**, **Vue 3**, **shadcn-vue**, and **SQLite**.

---

## Features

- **Double-Entry Accounting Engine** — Every transaction is validated for balance (debits = credits) using `decimal.js` arbitrary-precision math.
- **Split Transactions** — Model complex receipts and invoices across multiple accounts, assets, and categories.
- **Chart of Accounts** — Hierarchical tree with recursive balance rollups (Assets, Liabilities, Equity, Income, Expenses).
- **General Ledger** — Sortable, filterable, paginated register with expandable split rows (TanStack Table).
- **Multi-Currency** — Track accounts in distinct fiat currencies with historical exchange rates and trading accounts.
- **Investments** — Stock and mutual fund accounts with cost-basis tracking and price quote syncing.
- **Invoicing** — Full A/R and A/P lifecycle: create, send, track, and reconcile invoices and bills.
- **Budgeting** — Monthly/quarterly/annual spending targets with actual-vs-budget tracking.
- **Reconciliation** — Match ledger entries against bank statements with live balance comparison.
- **Scheduled Transactions** — Recurring rules for bills and income with auto-execute or prompt-on-startup.
- **Reports** — Balance Sheet, Profit & Loss, Cash Flow, Portfolio Valuation.
- **Dark Mode** — Full light/dark/system theme support with persistent preference.

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
| Precision Math | [decimal.js](https://github.com/MikeMcl/decimal.js) |
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
│   ├── ui/                      # shadcn-vue primitives (17 components)
│   │   ├── Button.vue
│   │   ├── Card.vue / Card*.vue
│   │   ├── Table.vue / Table*.vue
│   │   ├── Badge.vue
│   │   ├── Dialog.vue
│   │   ├── Input.vue / Label.vue
│   │   ├── Select.vue / SelectItem.vue
│   │   └── index.ts             # barrel exports
│   └── SplitLedgerTable.vue     # TanStack table with expandable splits
├── composables/
│   └── useAccountingEngine.ts   # Double-entry validation, tree builder
├── layouts/
│   └── DefaultLayout.vue        # Sidebar + header + <router-view>
├── lib/utils.ts                 # cn() helper (clsx + tailwind-merge)
├── router/index.ts              # 9 routes
├── stores/
│   ├── accountStore.ts          # Chart of Accounts state
│   ├── transactionStore.ts      # Transactions, filters, pagination
│   ├── uiStore.ts               # Theme, sidebar, modals
│   └── settingsStore.ts         # App preferences
├── types/
│   ├── account.ts               # Account, AccountType, AccountTreeNode
│   ├── transaction.ts           # Transaction, Split, RecurringTransaction
│   ├── currency.ts              # Currency, ExchangeRate
│   ├── invoice.ts               # Invoice, InvoiceLine, Contact
│   └── index.ts                 # Re-exports
├── utils/decimal.ts             # decimal.js helpers (formatMoney, add, sub, etc.)
├── views/
│   ├── Dashboard.vue
│   ├── Accounts.vue             # Hierarchical account tree
│   ├── Ledger.vue               # General ledger / register
│   ├── Reconciliation.vue
│   ├── Reports.vue
│   ├── ScheduledTransactions.vue
│   ├── Invoices.vue
│   ├── Contacts.vue
│   └── Budgets.vue
├── App.vue
└── main.ts                      # Entry point (Pinia + Router)
```

---

## Documentation

Detailed documentation for every subsystem is in the [`docs/`](./docs/) folder:

| Document | Covers |
|---|---|
| [Architecture](./docs/architecture/) | Data flow, Tauri ↔ Vue IPC, design decisions |
| [Database Schema](./docs/database/) | All tables, relationships, indexes, views |
| [UI Components](./docs/ui-components/) | shadcn-vue components, usage, props |
| [Accounting Engine](./docs/accounting-engine/) | Double-entry math, split validation, balance rollups |
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
