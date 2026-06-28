# Architecture Overview

## Data Flow

```
Vue 3 Views
    ↕ Pinia Stores
    ↕ Composables (useAccountingEngine)
    ↕ Tauri IPC (invoke)
    ↕ Rust Commands
    ↕ Accounting Engine (Rust)
    ↕ SQLite
```

See [`ARCHITECTURE.md`](../../ARCHITECTURE.md) at the project root for a full data-flow diagram.

## Key Principles

- **Local-first**: All data lives in a local SQLite file. No cloud dependency.
- **Belt-and-suspenders validation**: Client-side `decimal.js` checks balance before sending; Rust re-validates before persisting.
- **Precision everywhere**: Never use `Number` for financial math. All monetary values are `string`-encoded decimal numbers.
- **Own your UI**: shadcn-vue components are copied into the repo. Full control over every pixel.

## Tauri ↔ Vue IPC

The frontend communicates with the Rust backend exclusively through Tauri's `invoke` function:

```ts
import { invoke } from "@tauri-apps/api/core"

const result = await invoke("post_transaction", { data: txnPayload })
```

All Rust commands are defined in `src-tauri/src/` with `#[tauri::command]`.
