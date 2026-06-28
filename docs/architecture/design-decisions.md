# Design Decisions

## Why Adjacency List for Account Hierarchy?

- Simple recursive CTE queries for balance rollups
- Easy to insert, move, or delete nodes
- No nested-set complexity for a desktop app

## Why decimal.js instead of native BigInt?

- `decimal.js` supports arbitrary precision out of the box
- Handles division and multiplication without manual scaling
- Well-maintained, tree-shakeable ESM build

## Why Split Transactions?

One transaction row maps to N split rows. Each split is a single leg of a double-entry journal entry. This allows:

- A single paycheck split across Income (salary), Tax (liability), and Deductions (expense)
- A single credit card payment split across multiple purchases
- Stock purchases split between Asset (shares) and Expense (commission)

## Why Not WASM for the Engine?

- Tauri's Rust backend is already native and fast
- Keeping validation in Rust means we get free thread-safety
- WASM would add build complexity for no measurable gain in a desktop app
