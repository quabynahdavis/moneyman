# Reconciliation

Routes: `/reconciliation`, `/reconciliation/:accountId`

## Purpose

Match ledger entries against real-world bank statements (bank reconciliation).

## Implementation

Reconciliation is fully implemented with a Rust backend and Vue frontend.

### Backend (Rust)

- `src-tauri/src/commands/reconciliation.rs` — Commands: `start_reconciliation`, `get_reconciliation_data`, `toggle_split_reconcile_state`, `finalize_reconciliation`, `check_reconciled_split`
- `src-tauri/src/models/reconciliation.rs` — `ReconciliationSession`, `ReconcileSplit`, `ReconciliationData` structs
- Schema v4: `reconciliation_sessions` and `import_profiles` tables
- Guard: reconciled splits cannot be toggled back to cleared
- `finalize_reconciliation` requires `difference == 0`

### Frontend

- `src/stores/reconciliationStore.ts` — Pinia store managing session lifecycle, split toggle (with local cleared total recompute), finalize guard
- `src/views/Reconciliation.vue` — Split-pane layout:
  - **Start form**: Account picker, statement date, starting/ending balance
  - **Widget row**: Statement Balance, Cleared Total, Difference (color-coded)
  - **Transaction list**: Searchable table of unreconciled splits, click to toggle n↔c state
  - **Finalize button**: Enabled only when `difference == 0`

### Difference Equation

```
difference = clearedTotal - statementBalance
```

- `difference == 0` → balanced (finalize enabled)
- `difference > 0` → amber (over-cleared)
- `difference < 0` → red (under-cleared)

### Reconcile States

| Code | Meaning |
|------|---------|
| `n` | New / unreconciled |
| `c` | Cleared (matched to statement) |
| `r` | Reconciled (finalized, read-only) |
