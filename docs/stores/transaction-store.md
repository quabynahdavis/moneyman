# Transaction Store (`useTransactionStore`)

Manages transactions, splits, and recurring schedules.

## State

- `transactions: Transaction[]`
- `recurringTransactions: RecurringTransaction[]`
- `loading`, `error`
- Pagination: `currentPage`, `pageSize` (default 50)
- Sorting: `sortField` (post_date/description/state/num), `sortDirection`
- Filters: `filterText`, `filterAccountId`, `filterState`

## Computed

| Property | Description |
|---|---|
| `totalTransactions` | Count |
| `paginatedTransactions` | Current page slice |
| `totalPages` | Page count |
| `upcomingRecurring` | Active recurring items due today or earlier |

## Methods

| Method | Description |
|---|---|
| `fetchTransactions()` | API-backed paginated fetch with current filters/sort |
| `postNewTransaction(payload)` | Validates balance, posts via API, refreshes accounts |
| `voidExistingTransaction(id)` | Soft-delete via API, refreshes accounts |
| `setSort(field)` | Toggle sort direction |
| `setPage(page)` | Navigate pages |

All monetary values use **integer cents**. Validation uses strict integer equality (`===`), not floating-point epsilon.
