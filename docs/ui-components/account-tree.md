# AccountTree

Recursive tree-table component for the Chart of Accounts.

## Props

| Prop | Type | Description |
|---|---|---|
| `nodes` | `AccountNode[]` | Top-level nodes (with optional nested children) |

## Emits

| Event | Payload | Description |
|---|---|---|
| `select` | `node: AccountNode` | Fired when "View" button is clicked |

## Behavior

- **Accordion Toggle** — Clicking the row or the chevron folds/unfolds the node and all its descendants
- **Placeholder Dimming** — Nodes with `isPlaceholder === true` get `opacity-60 italic`
- **Right-Aligned Balances** — Monospace `tabular-nums` with `text-right`
- **Hover "View"** — A `Button` with `opacity-0 group-hover:opacity-100` appears on row hover
- **Tree Connectors** — Indentation via `paddingLeft` based on `depth`, no ASCII tree lines

## Implementation

The tree is flattened into `flatRows` via a recursive `walk()` that respects the `expanded: Set<number>` ref. The `toggle()` function calls `collectDescendantIds()` to fold/unfold the entire subtree at once. Since it is a single component (not recursive), the `expanded` set is shared across all levels.
