# Column Definitions

The table uses a static template (not TanStack Table) for full control over layout and event handling.

| Column | Accessor | Sortable | Formatting |
|---|---|---|---|
| (chevron) | — | No | Expand/collapse toggle |
| Date | `postDate` | No | Raw ISO string, monospace |
| Num | `num` | No | Raw value or empty |
| Description | `description` | No | Bold text |
| Transfer | `transferAccount` | No | Opposite account or "— Split Transaction —" |
| Debit / Deposit / etc. | `debit` | No | `formatCents()` — hidden if 0 |
| Credit / Withdrawal / etc. | `credit` | No | `formatCents()` — hidden if 0 |
| Balance | `balance` | No | `formatCents()`, running cumulative |
| R | `state` | No | Badge letter (R / C / V) |
| (⋮) | — | No | Ellipsis dropdown with Edit / Void |

## Context-Aware Headers

Debit and Credit column headers swap text based on account type:

| Account Types | Debit Column | Credit Column |
|---|---|---|
| ASSET, BANK, CASH | Deposit | Withdrawal |
| LIABILITY, CREDIT_CARD, PAYABLE | Payment | Charge |
| EXPENSE | Expense | Rebate |
| INCOME | Decrease | Income |

## Expanded Split Detail

When a row is expanded, a nested sub-table appears showing every underlying split with:

- Account name (bold if it matches the current `accountId`)
- Memo
- Debit (cents formatted)
- Credit (cents formatted)

## Formatting

All monetary values use `tabular-nums` CSS class for vertical alignment of decimal points. The `formatCents()` utility divides integer cents by 100 for display (e.g., `1050` → `$10.50`).
