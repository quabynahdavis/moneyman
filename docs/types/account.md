# Account Types

## `AccountType`

```ts
type AccountType =
  | "ROOT" | "ASSET" | "BANK" | "CASH"
  | "INVESTMENT" | "STOCK" | "MUTUAL_FUND"
  | "RECEIVABLE" | "LIABILITY" | "CREDIT_CARD"
  | "PAYABLE" | "EQUITY" | "INCOME" | "EXPENSE"
```

## `Account`

| Field | Type | Description |
|---|---|---|
| `id` | `number` | Primary key |
| `parentId` | `number \| null` | Parent account ID |
| `accountType` | `AccountType` | Type enum |
| `code` | `string \| null` | Account number (e.g. "1000") |
| `name` | `string` | Account name |
| `description` | `string \| null` | Optional description |
| `currencyCode` | `string` | ISO 4217 currency |
| `placeholder` | `boolean` | Parent-only (no txns) |
| `isActive` | `boolean` | Soft delete |
| `sortOrder` | `number` | Display order |
| `balance` | `string` | Computed decimal balance |

## `AccountTreeNode`

Extends `Account` with:
- `children: AccountTreeNode[]`
- `depth: number`

## Helpers

- `ACCOUNT_TYPE_LABELS` — Map of type → human-readable name
- `ACCOUNT_TYPE_TAXONOMY` — Ordered array of all types
- `isAccountParentType(t)` — Returns true for ROOT/ASSET/LIABILITY/EQUITY/INCOME/EXPENSE
- `getAccountNormalBalance(t)` — Returns `"debit"` or `"credit"`
