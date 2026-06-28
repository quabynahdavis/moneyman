# Transaction Store (`useTransactionStore`)

Manages transactions, splits, and recurring schedules.

## State

- `transactions: Transaction[]`
- `recurringTransactions: RecurringTransaction[]`
- `loading`, `error`
- Pagination: `currentPage`, `pageSize` (default 50)
- Sorting: `sortField` (date/payee/amount/state), `sortDirection`
- Filters: `filterText`, `filterAccountId`, `filterState`

## Computed

| Property | Description |
|---|---|
| `totalTransactions` | Count |
| `filteredTransactions` | Sorted + filtered list |
| `paginatedTransactions` | Current page slice |
| `totalPages` | Page count |
| `upcomingRecurring` | Active recurring items due today or earlier |

## Methods

| Method | Description |
|---|---|
| `computeTxnTotal(txn)` | Absolute total of all splits |
| `addTransaction(txn)` | Validates balance, prepends |
| `updateTransaction(id, updates)` | Validates if splits changed |
| `removeTransaction(id)` | Remove by ID |
| `setTransactions(data)` | Bulk replace |
| `setSort(field)` | Toggle sort direction |
| `setPage(page)` | Navigate pages |
| `addRecurring(rt) / removeRecurring(id)` | Manage schedules |
