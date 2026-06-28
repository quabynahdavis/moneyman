# Account Store (`useAccountStore`)

Manages the Chart of Accounts.

## State

- `accounts: Account[]` — Flat list of all accounts
- `loading: boolean`
- `error: string | null`

## Computed

| Property | Description |
|---|---|
| `accountTree` | Nested tree structure from `buildAccountTree()` |
| `activeAccounts` | Only accounts where `isActive === true` |
| `accountMap` | `Map<number, Account>` for O(1) lookups |

## Methods

| Method | Description |
|---|---|
| `getAccountById(id)` | Lookup by ID |
| `getChildren(parentId)` | Direct children |
| `getDescendantIds(accountId)` | Recursive descendant IDs |
| `getLeafAccounts(parentId?)` | Accounts with no children |
| `getAccountsByType(type)` | Filter by account type |
| `setAccounts(data)` | Bulk replace |
| `addAccount(account)` | Append single |
| `updateAccount(id, updates)` | Partial update |
| `removeAccount(id)` | Remove by ID |
