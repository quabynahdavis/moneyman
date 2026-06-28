# Account Store (`useAccountStore`)

Manages the Chart of Accounts — flat list and hierarchical tree.

## State

| Field | Type | Description |
|---|---|---|
| `accounts` | `Account[]` | Flat list of all accounts |
| `tree` | `AccountNode[]` | Hierarchical tree from Rust `get_account_tree` |
| `loading` | `boolean` | Loading flag |
| `error` | `string \| null` | Error message |

## Computed

| Property | Description |
|---|---|
| `treeWithRollup` | Recursive rollup: each parent's balance = its own balance + sum of all children's balances |
| `activeAccounts` | Only accounts where `isActive === true` |
| `accountMap` | `Map<number, Account>` for O(1) lookups |

## Methods

| Method | Description |
|---|---|
| `getAccountById(id)` | Lookup by ID |
| `getChildren(parentId)` | Direct children from flat list |
| `getDescendantIds(accountId)` | Recursive descendant IDs |
| `getLeafAccounts(parentId?)` | Accounts with no children |
| `getAccountsByType(type)` | Filter by account type |
| `addAccount(account)` | Append single |
| `updateAccountInStore(id, updates)` | Partial update |
| `removeAccount(id)` | Remove by ID |
| `fetchAccounts()` | Fetch flat list via `list_accounts` IPC |
| `fetchAccountTree()` | Fetch tree via `get_account_tree` IPC; returns `AccountNode[]` with children |
| `createNewAccount(payload)` | Create via IPC + refetch tree |
| `updateExistingAccount(payload)` | Update via IPC + refetch tree |

## IPC Calls

- `list_accounts` — Flat accounts with balance CTE
- `get_account_tree` — Nested tree built server-side from adjacency list
- `create_account` / `update_account` — Standard CRUD
