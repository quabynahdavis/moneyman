# Ledger

Routes: `/ledger` (all), `/ledger/:accountId` (filtered)

## Purpose

The general ledger / checkbook register — the primary transaction browsing interface.

## Features

- **Full-account view** — `/ledger` shows all transactions
- **Account-filtered view** — `/ledger/123` shows only transactions touching account 123
- **New Transaction button** — (placeholder for future form dialog)
- **SplitLedgerTable** — Sortable, filterable, expandable table

## State Dependencies

- `transactionStore.paginatedTransactions` — Current page data
- `accountStore.getAccountById()` — For display name in filtered view
- `route.params.accountId` — Active account filter

## Dynamic Title

The header reads "Register: Account Name" when viewing a single account, or "General Ledger" for the unfiltered view.
