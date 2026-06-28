# Routes

| Path | Name | View | Title |
|---|---|---|---|
| `/` | — | Redirect to `/dashboard` | — |
| `/dashboard` | `dashboard` | Dashboard | "Dashboard" |
| `/accounts` | `accounts` | Accounts | "Chart of Accounts" |
| `/ledger` | `ledger` | Ledger | "General Ledger" |
| `/ledger/:accountId` | `account-ledger` | Ledger | "Account Register" |
| `/reconciliation` | `reconciliation` | Reconciliation | "Reconciliation" |
| `/reports` | `reports` | Reports | "Reports" |
| `/scheduled` | `scheduled` | ScheduledTransactions | "Scheduled Transactions" |
| `/invoices` | `invoices` | Invoices | "Invoices & Bills" |
| `/contacts` | `contacts` | Contacts | "Contacts" |
| `/budgets` | `budgets` | Budgets | "Budgets" |

All routes use lazy loading (`() => import(...)`) for code splitting.
