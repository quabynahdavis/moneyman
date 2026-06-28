# Accounts

Route: `/accounts`

## Purpose

Displays the Chart of Accounts as an interactive hierarchical tree using `AccountTree.vue`, with live balance rollups from the Rust backend.

## Features

- **Accordion Toggle** — Click the chevron to fold/unfold a parent and all its descendants; clicking the row itself also toggles
- **Placeholder Dimming** — Accounts with `isPlaceholder === true` render at `opacity-60 italic`
- **Sub-Account Drilldown** — Hover "View" button navigates parents to `/accounts/:id` (immediate children table) and leaves to `/ledger/:id`
- **Right-Aligned Balances** — Monospace `tabular-nums`, negative values in `text-rose-500`
- **Type Badges** — Each account shows its GnuCash type as a `Badge`
- **Refresh / New Account** — Header toolbar buttons

## State Dependencies

- `accountStore.treeWithRollup` — Recursive tree with rollup balances
- `accountStore.loading` / `accountStore.error`

## Implementation

The tree is rendered via a depth-tracked `flatRows` computed. A single `expanded: Set<number>` ref controls which nodes are visible. Clicking the row calls `toggle(node)`, which uses `collectDescendantIds` to fold/unfold the entire subtree in one set operation.
