# Ledger Store (`useLedgerStore`)

Provides context-aware label mappings and running balance computation for the `LedgerTable` component.

## State

| Property | Type | Description |
|---|---|---|
| `accountContext` | `AccountContext` | Current context derived from account type (ASSET / BANK / CASH / LIABILITY / CREDIT_CARD / PAYABLE / EXPENSE / INCOME) |
| `contextLabels` | `{ debit: string, credit: string }` | Computed header labels based on context |

## Context Labels (`ACCOUNT_CONTEXT_LABELS`)

| Context | Debit Label | Credit Label |
|---|---|---|
| ASSET, BANK, CASH | Deposit | Withdrawal |
| LIABILITY, CREDIT_CARD, PAYABLE | Payment | Charge |
| EXPENSE | Expense | Rebate |
| INCOME | Decrease | Income |

## Methods

| Method | Description |
|---|---|
| `setAccountContext(accountType)` | Update context based on account type string |
| `buildLedgerRows(transactions, forAccountId, normalBalance)` | Computes ledger rows with running balance, transfer column, context-aware sorting |

## Running Balance Logic

1. Sorts transactions ascending by `postDate`
2. Accumulates `running += normalBalance === 'debit' ? (debit - credit) : -(debit - credit)`
3. Reverses to display most-recent-first
4. All arithmetic uses integer cents
