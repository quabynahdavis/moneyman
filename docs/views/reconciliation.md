# Reconciliation

Route: `/reconciliation`

## Purpose

Match ledger entries against real-world bank statements.

## Planned Workflow

1. User selects an account to reconcile
2. Enters the statement balance and date
3. The view shows all uncleared transactions with checkboxes
4. As each item is marked "cleared," a running difference is shown
5. When the difference reaches $0.00, the reconciliation is complete

## Current State

Placeholder view with empty state illustration. The reconciliation store and SQL tables (`reconciliations`, `reconciliation_entries`) are already defined in the schema.
