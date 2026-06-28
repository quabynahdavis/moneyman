# Split Ledger Table

The `SplitLedgerTable` component renders a sortable, filterable, paginated register using `@tanstack/vue-table`.

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `transactions` | `Transaction[]` | Required | Data source |
| `showAccountColumn` | `boolean` | `false` | Show account name column |

## Features

- **Sorting** — Click column headers to sort ascending/descending (date, payee, amount, state)
- **Global Filter** — Text input filters across description, payee, and number
- **Expandable Splits** — Click the date cell to expand/collapse individual split rows
- **Column Visibility** — Account column is conditionally rendered via `showAccountColumn`

## Row Types

### Transaction Row
Shows date, number, payee, description, memo, debit, credit, and state badge. The date cell contains the expand/collapse chevron.

### Split Row
Shows individual journal entry legs with account name, memo, debit, and credit amounts. Rendered at half-height (text-xs) with muted foreground.

## State Indicators

| Badge | Meaning |
|---|---|
| `RECONCILED` (green) | Matched bank statement |
| `CLEARED` (yellow) | Pending confirmation |
| `VOID` (red) | Cancelled |
| `UNRECONCILED` (gray) | Not yet processed |
