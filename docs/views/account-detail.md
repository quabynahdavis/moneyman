# Account Detail

Route: `/accounts/:accountId`

## Purpose

Drilldown view showing the immediate sub-accounts of a selected parent account, with their names, types, codes, and rolled-up balances.

## Features

- **Account Info Header** — Name, type badge, code, child count
- **Children Table** — Code, name, type, right-aligned balance, "Ledger" button per row
- **Back Navigation** — Arrow-left button returns to full Chart of Accounts
- **Empty State** — "No sub-accounts under this account." when leaf node is viewed

## State Dependencies

- `accountStore.treeWithRollup` — Finds the target node via recursive `findNode()`
- `accountStore.tree` — Triggers `fetchAccountTree()` if empty on mount

## Implementation

The view uses a local `findNode()` function that walks `treeWithRollup` to locate the account by route param `accountId`. Children are then rendered as a simple list (not an accordion). Each child row's "Ledger" button navigates to `/ledger/:childId`.
