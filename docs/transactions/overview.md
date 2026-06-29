# Transactions

## Core Concept

Moneyman uses standard **double-entry accounting**. Every financial event is recorded as a **transaction** containing two or more **splits** (journal entry lines). The fundamental rule:

**For every transaction: Σ debits = Σ credits**

Each split assigns a monetary amount to a specific account. Debits increase asset/expense accounts and decrease liability/income/equity accounts. Credits do the opposite.

```
Transaction: "Groceries - $50.00"
┌──────────────────────────────────────────────────────────┐
│ Description: "Weekly grocery run"                        │
│ Post Date: 2026-06-29                                    │
│ State: UNRECONCILED                                      │
├──────────────────────────────────────────────────────────┤
│ Splits:                                                  │
│   Split 1: Checking Account    Credit: $50.00 (outflow)  │
│   Split 2: Food & Dining       Debit:  $50.00 (expense)  │
├──────────────────────────────────────────────────────────┤
│ Total Debits:  $50.00  Total Credits: $50.00  ✓ BALANCED │
└──────────────────────────────────────────────────────────┘
```

---

## Database Schema

### `transactions` table

| Column | Type | Description |
|---|---|---|
| `id` | INTEGER (PK) | Auto-increment ID |
| `currency_code` | TEXT | Transaction currency (default `'USD'`) |
| `description` | TEXT | Payee / description (required, non-null) |
| `notes` | TEXT | Optional extended notes |
| `num` | TEXT | Check or reference number |
| `post_date` | TEXT | Transaction date in `YYYY-MM-DD` format |
| `state` | TEXT | Lifecycle state: `UNRECONCILED`, `CLEARED`, `RECONCILED`, or `VOID` |
| `created_at` | TEXT | Auto-set insertion timestamp |
| `updated_at` | TEXT | Auto-set update timestamp |

### `splits` table

| Column | Type | Description |
|---|---|---|
| `id` | INTEGER (PK) | Auto-increment ID |
| `transaction_id` | INTEGER (FK→transactions) | Parent transaction (CASCADE on delete) |
| `account_id` | INTEGER (FK→accounts) | Account to post to (RESTRICT on delete) |
| `memo` | TEXT | Optional split-level memo |
| `debit` | INTEGER | Debit amount in cents (≥ 0) |
| `credit` | INTEGER | Credit amount in cents (≥ 0) |
| `reconcile_state` | TEXT | `'n'` = new/unreconciled, `'c'` = cleared, `'r'` = reconciled |
| `quantity` | TEXT | Number of shares (for stock accounts) |
| `action` | TEXT | `'Buy'`, `'Sell'`, `'Dividend'`, `'Fee'` (for investment accounts) |
| `created_at` | TEXT | Auto-set insertion timestamp |

### Relationships

```
transactions  1 ──── * splits
accounts     1 ──── * splits
```

A transaction always has ≥ 2 splits. A split belongs to exactly one transaction and one account.

---

## Integer Cents

**All monetary values are stored as integer cents** — never as floating-point decimals.

| Layer | Type | Example |
|---|---|---|
| Rust (SQLite) | `INTEGER` / `i64` | `5000` = $50.00 |
| Frontend (TypeScript) | `number` | `5000` = $50.00 |
| Display | Formatted string | `"$50.00"` via `formatCents()` |

```typescript
// utils/decimal.ts
export function toCents(amount: string | number): number {
  return new Decimal(amount).times(100).toNumber()
  // "50.00" → 5000
}

export function fromCents(cents: number): string {
  return new Decimal(cents).div(100).toFixed(2)
  // 5000 → "50.00"
}

export function formatCents(cents: number, currencySymbol = "$"): string {
  return formatMoney(fromCents(cents), currencySymbol)
  // 5000 → "$50.00"
}
```

This eliminates floating-point rounding errors (`0.1 + 0.2 !== 0.3`) entirely. The Rust backend performs all balance checks with strict `i64` integer arithmetic.

---

## TypeScript Types

```typescript
// Frontend (camelCase — mapped from Rust snake_case via api.ts)

type TransactionState = "UNRECONCILED" | "CLEARED" | "RECONCILED" | "VOID"
type ReconcileState = "n" | "c" | "r"

interface Transaction {
  id?: number
  currencyCode: string       // "USD"
  description: string
  notes: string | null
  num: string | null
  postDate: string           // "2026-06-29"
  state: TransactionState
  splits: Split[]
  createdAt?: string
  updatedAt?: string
}

interface Split {
  id?: number
  transactionId?: number
  accountId: number
  accountName?: string       // Joined from accounts table
  accountType?: string       // Joined from accounts table
  debit: number              // Integer cents
  credit: number             // Integer cents
  memo: string | null
  reconcileState: ReconcileState
  quantity: string | null
  action: string | null
}
```

```rust
// Rust backend (snake_case — matches JSON field names)

struct Transaction {
    id: Option<i64>,
    currency_code: String,
    description: String,
    notes: Option<String>,
    num: Option<String>,
    post_date: String,
    state: String,
    splits: Vec<Split>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

struct Split {
    id: Option<i64>,
    transaction_id: Option<i64>,
    account_id: i64,
    account_name: Option<String>,
    account_type: Option<String>,
    debit: i64,
    credit: i64,
    memo: Option<String>,
    reconcile_state: String,
    quantity: Option<String>,
    action: Option<String>,
}
```

The `api.ts` service layer translates between the two naming conventions using `toCamelTransaction()` and `toCamelSplit()`.

---

## States & Lifecycle

### Transaction State

```
UNRECONCILED ──→ CLEARED ──→ RECONCILED
       │
       └──→ VOID
```

| State | Meaning |
|---|---|
| `UNRECONCILED` | Newly created, not yet matched to a bank statement |
| `CLEARED` | Matched to a bank statement line item |
| `RECONCILED` | Finalized in a reconciliation session (read-only) |
| `VOID` | Reversed/removed from financial reporting |

A void transaction remains in the database but is excluded from balance calculations and ledger views.

### Split Reconcile State

| Code | Meaning |
|---|---|
| `n` | New — split has not been touched by reconciliation |
| `c` | Cleared — split matches a line on the bank statement |
| `r` | Reconciled — the reconciliation session was finalized |

Reconciled splits (`r`) are **read-only** and cannot be toggled back. This is enforced by a guard clause in `toggle_split_reconcile_state`.

---

## Validation Rules

Validation runs both on the **frontend** (immediate feedback) and **backend** (belt-and-suspenders).

1. **At least 2 splits** — A transaction must affect at least two accounts.
2. **Debits = Credits** — Sum of all debit amounts must equal sum of all credit amounts (strict integer equality, not floating).
3. **No split has both** — Each split must have `debit > 0 XOR credit > 0`. Both cannot be non-zero.
4. **No placeholder accounts** — Splits cannot reference accounts marked `is_placeholder = 1` (e.g., parent categories like "Assets" or "Expenses").

```rust
// Rust validation in post_transaction
if payload.splits.len() < 2 {
    return Err("Transaction must have at least 2 splits".to_string());
}

let total_debits: i64 = payload.splits.iter().map(|s| s.debit).sum();
let total_credits: i64 = payload.splits.iter().map(|s| s.credit).sum();
if total_debits != total_credits {
    return Err("Transaction out of balance".to_string());
}

for s in &payload.splits {
    if s.debit > 0 && s.credit > 0 {
        return Err("Split cannot have both debit and credit".to_string());
    }
    if is_placeholder {
        return Err("Cannot post to placeholder account".to_string());
    }
}
```

```typescript
// Frontend validation in useAccountingEngine
function validateBalanced(splits: Split[]): void {
  if (splits.length < 2)
    throw new Error("Transaction must have at least 2 splits")
  if (totalDebits !== totalCredits)
    throw new Error("Transaction out of balance")
  if (split.debit > 0 && split.credit > 0)
    throw new Error("Split cannot have both debit and credit amounts")
}
```

---

## Balance Computation

### Per-Account Balance

An account's balance is the net sum of all its split amounts across non-void transactions:

```
balance = Σ(debit) − Σ(credit)
// (across all non-void splits referencing this account)
```

For asset/expense accounts (normal debit balance):
- A positive balance means the account has a net debit (e.g., Checking has money)
- A negative balance means the account has a net credit (overdrawn)

For liability/income/equity accounts (normal credit balance):
- A positive balance means the account has a net credit (e.g., Credit Card owes money)
- A negative balance means the account has a net debit

### Account Hierarchy Rollup

Parent accounts (placeholders like "Assets" or "Expenses") compute their balance via a **recursive CTE** that aggregates all descendant leaf accounts:

```sql
-- Simplified from schema.rs v_account_balances
WITH RECURSIVE account_tree AS (
    SELECT id, parent_id FROM accounts WHERE is_active = 1
),
leaf_balances AS (
    SELECT a.id AS account_id,
           COALESCE(SUM(s.debit - s.credit), 0) AS balance
    FROM accounts a
    LEFT JOIN splits s ON s.account_id = a.id
    LEFT JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
    GROUP BY a.id
)
SELECT a.id, COALESCE(SUM(r.balance), 0) AS balance
FROM accounts a
LEFT JOIN account_tree t ON t.id IN (
    WITH RECURSIVE desc_ids AS (
        SELECT id FROM accounts WHERE parent_id = a.id
        UNION ALL
        SELECT acc.id FROM accounts acc JOIN desc_ids ON acc.parent_id = desc_ids.id
    )
    SELECT id FROM desc_ids
    UNION SELECT a.id
)
LEFT JOIN leaf_balances r ON r.account_id = t.id
GROUP BY a.id
```

### Running Balance (Ledger)

The ledger computes a running balance by sorting transactions by `post_date` and applying each transaction's net effect:

```
running += (debit − credit) × direction

Where direction = +1 for normal-debit accounts (asset/expense)
                 −1 for normal-credit accounts (liability/income/equity)
```

```typescript
// LedgerTable.vue
let running = startingBalance
for (const row of sortedRows) {
  const change = row.debit - row.credit
  running += normalBalance === "debit" ? change : -change
  row.balance = running
}
```

---

## Examples

### Simple 2-Split Transaction (Checking → Groceries)

Transfer $50.00 from Checking Account to Food & Dining:

```
Transaction: "Weekly grocery run"
  Split 1: Checking Account   credit = 5000   (money leaves the account)
  Split 2: Food & Dining      debit  = 5000   (expense recorded)
  ----------------------------------------------------
  Total Debits:  5000  Total Credits:  5000  ✓ Balanced
```

Effect on balances:
- Checking Account: balance decreases by $50.00
- Food & Dining: balance increases by $50.00

### Multi-Split Transaction (Split a Bill)

Split a $100.00 restaurant bill across two expense categories:

```
Transaction: "Business lunch"
  Split 1: Checking Account      credit = 10000  (money leaves)
  Split 2: Food & Dining         debit  =  6000  (60%)
  Split 3: Entertainment         debit  =  4000  (40%)
  ----------------------------------------------------
  Total Debits:  10000  Total Credits:  10000  ✓ Balanced
```

### Income + Asset (Paycheck)

Record a $2,000.00 paycheck deposited into Checking:

```
Transaction: "Acme Corp salary"
  Split 1: Checking Account      debit  = 200000  (money arrives)
  Split 2: Employment Income     credit = 200000  (income earned)
  ----------------------------------------------------
  Total Debits:  200000  Total Credits:  200000  ✓ Balanced
```

### Loan Draw (Liability)

Borrow $10,000.00 deposited into Checking:

```
Transaction: "Bank loan disbursement"
  Split 1: Checking Account      debit  = 1000000  (money arrives)
  Split 2: Loan Payable          credit = 1000000  (liability incurred)
  ----------------------------------------------------
  Total Debits:  1000000  Total Credits:  1000000  ✓ Balanced
```

---

## Data Flow

```
┌──────────────────────────────────────────────────────────────┐
│ Vue Frontend                                                  │
│                                                               │
│  TransactionNew.vue                                           │
│   │ User fills form (date, description, splits with           │
│   │ amounts + account selection)                               │
│   │                                                              │
│   ▼                                                              │
│  save()                                                         │
│   │ Checks isBalanced (computed: debits===credits,             │
│   │ all accounts selected, ≥2 splits)                          │
│   │                                                              │
│   ▼                                                              │
│  transactionStore.postNewTransaction(payload)                   │
│   │ validateBalanced() — client-side belt check                 │
│   │                                                              │
│   ▼                                                              │
│  api.postTransaction(payload)                                   │
│   │ invoke("post_transaction", { payload: {...} })              │
│   │ Maps camelCase → snake_case                                 │
│   │                                                              │
├──┼──────────────────────────────────────────────────────────────┤
│   │ Tauri IPC (JSON over webview bridge)                        │
├──┼──────────────────────────────────────────────────────────────┤
│   ▼                                                              │
│ Rust Backend                                                    │
│                                                               │
│  post_transaction(db, payload)                                 │
│   │ Validates: ≥2 splits, debits===credits, no zero-only       │
│   │ splits, no placeholder accounts, no split with both        │
│   │                                                              │
│   ▼                                                              │
│  INSERT INTO transactions (...)                                 │
│  INSERT INTO splits (...)  (× N)                                │
│                                                               │
│   ▼                                                              │
│  Returns Transaction with splits (serde_json → Tauri → invoke) │
│                                                               │
├──┼──────────────────────────────────────────────────────────────┤
│   │ Response back to frontend                                   │
├──┼──────────────────────────────────────────────────────────────┤
│   ▼                                                              │
│ Vue Frontend                                                    │
│                                                               │
│  transactionStore                                               │
│   │ transactions.unshift(txn) — prepend to list                 │
│   │ totalTransactions++                                         │
│   │ accountStore.fetchAccounts() — refresh balances             │
│   │                                                              │
│   ▼                                                              │
│  router.push({ name: "ledger" }) — navigate to ledger           │
└──────────────────────────────────────────────────────────────┘
```

---

## API

### `post_transaction`

Create a new transaction with splits.

```typescript
// Frontend payload (camelCase)
interface CreateTransactionPayload {
  currencyCode?: string    // default "USD"
  description?: string | null
  notes?: string | null
  num?: string | null
  postDate?: string        // default today
  state?: string           // default "UNRECONCILED"
  splits: {
    accountId: number
    debit: number          // integer cents
    credit: number         // integer cents
    memo?: string | null
  }[]
}

// Returns: Transaction (with generated id + splits)
```

```rust
// Rust command
#[tauri::command]
fn post_transaction(
    db: State<Database>,
    payload: CreateTransactionPayload,
) -> Result<Transaction, String>
```

### `list_transactions`

Fetch transactions with pagination, sorting, and filtering.

```typescript
interface ListTransactionsQuery {
  page?: number            // default 1
  pageSize?: number        // default 50, max 500
  sortField?: string       // "post_date" | "description" | "state" | "num"
  sortDirection?: string   // "asc" | "desc"
  filterText?: string      // searches description + num
  filterAccountId?: number | null
  filterState?: string | null
}

// Returns: { transactions: Transaction[], total: number, page: number, pageSize: number }
```

### `void_transaction`

Mark a transaction as VOID (excluded from balances/ledger).

```typescript
function voidTransaction(id: number): Promise<void>
```

Sets `state = 'VOID'` and `updated_at = datetime('now')`. The transaction and its splits remain in the database for audit trail purposes.

---

## Related Files

| Layer | File | Purpose |
|---|---|---|
| Schema | `src-tauri/src/db/schema.rs` | `transactions` + `splits` table DDL, v3 migration |
| Rust model | `src-tauri/src/models/transaction.rs` | Rust structs: `Transaction`, `Split`, `CreateTransactionPayload` |
| Rust commands | `src-tauri/src/commands/transactions.rs` | `post_transaction`, `list_transactions`, `void_transaction` |
| Frontend types | `src/types/transaction.ts` | TypeScript interfaces + helpers |
| Frontend API | `src/services/api.ts` | `postTransaction`, `listTransactions`, `voidTransaction` with snake_case mapping |
| Pinia store | `src/stores/transactionStore.ts` | State: transactions list, filters, pagination. Actions: `postNewTransaction`, `fetchTransactions`, `voidExistingTransaction` |
| View (create) | `src/views/TransactionNew.vue` | Form with date picker, account picker, split rows, auto-balance |
| View (list) | `src/views/Ledger.vue` | Ledger/register view using LedgerTable |
| Component (table) | `src/components/LedgerTable.vue` | TanStack table with expandable splits, running balance, context menus |
| Validation | `src/composables/useAccountingEngine.ts` | `validateBalanced`, `createSimpleSplits`, `computeRunningBalance` |
| Normal balances | `src/types/account.ts` | `getAccountNormalBalance()` — debit vs credit per account type |
