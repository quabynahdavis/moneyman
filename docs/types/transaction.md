# Transaction Types

## `Transaction`

| Field | Type | Description |
|---|---|---|
| `id` | `number` (optional) | Primary key |
| `currencyCode` | `string` | Transaction currency |
| `description` | `string \| null` | Memo/description |
| `notes` | `string \| null` | Extended notes |
| `payee` | `string \| null` | Payee name |
| `number` | `string \| null` | Check/reference number |
| `date` | `string` | Transaction date (ISO) |
| `datePosted` | `string` | Posted date |
| `state` | `TransactionState` | UNRECONCILED / CLEARED / RECONCILED / VOID |
| `splits` | `Split[]` | Journal entry legs |

## `Split`

| Field | Type | Description |
|---|---|---|
| `id` | `number` (optional) | Primary key |
| `transactionId` | `number` (optional) | Parent transaction |
| `accountId` | `number` | Account to post to |
| `debitAmount` | `string` | Decimal string |
| `creditAmount` | `string` | Decimal string |
| `memo` | `string \| null` | Split-level memo |
| `quantity` | `string \| null` | Shares (stock accounts) |
| `action` | `string \| null` | Buy/Sell/Dividend/Fee |

## Helpers

- `isTransactionBalanced(splits)` — Returns `true` if total debits equal total credits
- `computeTransactionTotal(splits)` — Returns net amount as string
