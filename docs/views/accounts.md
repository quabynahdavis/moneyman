# Accounts

Route: `/accounts`

## Purpose

Displays the Chart of Accounts as an interactive hierarchical tree with live balance rollups.

## Features

- **Expand/Collapse** — Click parent accounts to show/hide children
- **Account Type Badge** — Color-coded type indicator
- **Balance Display** — Right-aligned monospace figures (red for negative)
- **View Button** — Hover to navigate to account-specific register
- **New Account Button** — (placeholder for future dialog)

## State Dependencies

- `accountStore.accounts` — Flat list
- `accountStore.accountTree` — Computed nested tree

## Implementation

The tree is flattened into a `flatTree` computed for rendering. Depth is tracked for indentation. The `expanded` ref holds a `Set<number>` of expanded node IDs.
