# Transaction Types

## `Transaction`

| Field | Type | Description |
|---|---|---|
| `id` | `number` (optional) | Primary key |
| `currencyCode` | `string` | Transaction currency |
| `description` | `string` | Payee / description (required, non-null) |
| `notes` | `string \| null` | Extended notes |
| `num` | `string \| null` | Check/reference number |
| `postDate` | `string` | Transaction date (ISO) |
| `state` | `TransactionState` | UNRECONCILED / CLEARED / RECONCILED / VOID |
| `splits` | `Split[]` | Journal entry legs |

## `Split`

| Field | Type | Description |
|---|---|---|
| `id` | `number` (optional) | Primary key |
| `transactionId` | `number` (optional) | Parent transaction |
| `accountId` | `number` | Account to post to |
| `debit` | `number` | Integer cents (not decimal) |
| `credit` | `number` | Integer cents (not decimal) |
| `memo` | `string \| null` | Split-level memo |
| `reconcileState` | `'n' \| 'c' \| 'r'` | Unreconciled / Cleared / Reconciled |
| `quantity` | `string \| null` | Shares (stock accounts) |
| `action` | `string \| null` | Buy/Sell/Dividend/Fee |

## Helpers

- `isTransactionBalanced(splits)` — Returns `true` if total debit cents equal total credit cents (strict integer equality)
- `computeTransactionTotal(splits)` — Returns net cents as number
- `getNetAmount(splits, accountId)` — Returns net cents for a specific account within a set of splits

All monetary values use **integer cents**. Display formatting divides by 100 — see `formatCents()` in `decimal.ts`.
