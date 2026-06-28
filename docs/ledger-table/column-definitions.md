# Column Definitions

The table uses simple `{ accessorKey, header }` objects (no `columnHelper` to avoid typing issues with Vue TanStack adapter).

| Column | Accessor Key | Sortable | Formatting |
|---|---|---|---|
| Date | `date` | Yes | Raw string (ISO) |
| Num | `number` | Yes | Raw value or empty string |
| Payee | `payee` | Yes | Raw value or empty string |
| Description | `description` | No | Raw value or empty string |
| Account | `accountName` | No | Only shown when `showAccountColumn` is true |
| Memo | `memo` | No | Raw value or empty string |
| Debit | `debit` | Yes | `formatMoney()` — hidden if "0" |
| Credit | `credit` | Yes | `formatMoney()` — hidden if "0" |
| State | `state` | Yes | Badge component (h() render) |

## Row Expansion

The table uses `getSubRows` to nest split rows under their parent transaction. The `expanded` state is a ref keyed by row ID. Toggling is handled by a click handler on the date cell.
