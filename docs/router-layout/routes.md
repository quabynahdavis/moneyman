# Routes

| Path | Name | View | Title |
|---|---|---|---|
| `/` | — | Redirect to `/dashboard` | — |
| `/dashboard` | `dashboard` | Dashboard | "Dashboard" |
| `/accounts` | `accounts` | Accounts | "Chart of Accounts" |
| `/accounts/new` | `account-new` | AccountNew | "New Account" |
| `/accounts/:accountId` | `account-detail` | AccountDetail | "Account Detail" |
| `/ledger` | `ledger` | Ledger | "General Ledger" |
| `/ledger/:accountId` | `account-ledger` | Ledger | "Account Register" |
| `/transactions/new` | `transaction-new` | TransactionNew | "New Transaction" |
| `/reconciliation` | `reconciliation` | Reconciliation | "Reconciliation" |
| `/reports` | `reports` | Reports | "Reports" |
| `/scheduled` | `scheduled` | ScheduledTransactions | "Scheduled Transactions" |
| `/invoices` | `invoices` | Invoices | "Invoices & Bills" |
| `/contacts` | `contacts` | Contacts | "Contacts" |
| `/budgets` | `budgets` | Budgets | "Budgets" |
| `/settings` | `settings` | Settings | "Settings" |

All routes use lazy loading (`() => import(...)`) for code splitting. Static routes (`/accounts/new`) are declared before dynamic segments (`/accounts/:accountId`) to avoid param capture.
