# Dashboard

Route: `/dashboard` (default redirect from `/`)

## Purpose

Provides a financial overview with summary cards and recent activity.

## Components

- **Net Worth** card — Total assets minus liabilities
- **Income (This Month)** card — Green, shows budget comparison
- **Expenses (This Month)** card — Red, shows budget comparison
- **Total Cash** card — Sum of all cash/bank accounts
- **Recent Transactions** card — Last few entries (placeholder)
- **Upcoming Scheduled** card — Next recurring items (placeholder)

## State Dependencies

- `uiStore` — theme/sidebar (via layout)
- `transactionStore` — for recent transactions (future)
- `accountStore` — for account balances (future)
