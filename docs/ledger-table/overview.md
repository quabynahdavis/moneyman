# Ledger Table

The `LedgerTable` component renders a context-aware transaction register with running balance, contextual column headers, and right-click context menus.

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `transactions` | `Transaction[]` | Required | Data source |
| `accountId` | `number \| null` | Required | Filters splits to this account; `null` for global ledger |
| `accountType` | `string \| null` | `null` | Determines column header labels and normal balance direction |

## Features

- **Contextual Headers** — Column labels for debit/credit swap based on account type (see `ACCOUNT_CONTEXT_LABELS` in `ledgerStore.ts`).
- **Transfer Column** — For 2-split transactions, shows the opposite account name; for multi-split, shows "— Split Transaction —".
- **Running Balance** — Cumulative sum of (debit − credit), adjusted for normal balance direction, computed client-side in cents.
- **Expandable Splits** — Click any row to expand/collapse a nested detail table showing individual split lines.
- **Right-Click Context Menu** — Right-click any row for Edit / Void actions.
- **Ellipsis Dropdown** — Hover any row to reveal a ⋮ button with the same actions.
- **Right-Aligned Numbers** — All monetary columns use `tabular-nums` for vertical alignment of decimal points.

## Events

| Event | Payload | Description |
|---|---|---|
| `void-transaction` | `txnId: number` | Emitted when user clicks Void |
| `edit-transaction` | `txnId: number` | Emitted when user clicks Edit |

## State Indicators

| Badge | Meaning |
|---|---|
| `R` (green) | Reconciled — matched bank statement |
| `C` (blue) | Cleared — pending confirmation |
| `V` (red) | Void — cancelled transaction |
| (none) | Unreconciled — not yet processed |
