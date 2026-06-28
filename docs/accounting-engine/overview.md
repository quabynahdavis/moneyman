# Accounting Engine

The accounting engine is implemented as a Vue composable: `useAccountingEngine()`.

## Core Functions

### `validateBalanced(splits)`
Throws if total debits ≠ total credits, or if any split has both non-zero debit and credit.

### `createSimpleSplits(debitAccountId, creditAccountId, amount, memo?)`
Returns a pair of `[debitSplit, creditSplit]` for a simple two-split transfer.

### `accountNetChange(accountId, splits)`
Computes the net (debit - credit) for a specific account across a set of splits.

### `computeRunningBalance(startingBalance, splits, normalBalance)`
Returns an array of `{ splitId, runningBalance }` objects.

### `buildAccountTree(accounts)`
Converts a flat `Account[]` into a nested `AccountTreeNode[]` with `children` and `depth`.

### `flattenAccountTree(tree)`
Depth-first flatten of the tree back to a flat list.

## Validation Rules

1. Every transaction must have at least 2 splits
2. A split cannot have both `debitAmount > 0` and `creditAmount > 0`
3. Sum of all `debitAmount` must equal sum of all `creditAmount`
