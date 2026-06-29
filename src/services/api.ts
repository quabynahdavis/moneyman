import { invoke } from "@tauri-apps/api/core"
import type { Account, AccountNode, Split, Transaction, ReconciliationData, ImportPreviewItem, CommitImportResult, ImportProfile, RecurringTransaction, RecurringSplit } from "@/types"

// ── Account API ──────────────────────────────────────────────────────────

export interface CreateAccountPayload {
  parentId: number | null
  accountType: string
  code: string | null
  name: string
  description: string | null
  currencyCode?: string
  isPlaceholder?: boolean
  sortOrder?: number
}

export interface UpdateAccountPayload {
  id: number
  parentId?: number | null
  accountType?: string
  code?: string | null
  name?: string
  description?: string | null
  currencyCode?: string
  isPlaceholder?: boolean
  isActive?: boolean
  sortOrder?: number
}

function toCamelAccount(raw: any): Account {
  return {
    id: raw.id,
    parentId: raw.parent_id,
    accountType: raw.account_type,
    code: raw.code,
    name: raw.name,
    description: raw.description,
    currencyCode: raw.currency_code,
    isPlaceholder: raw.is_placeholder,
    isActive: raw.is_active,
    sortOrder: raw.sort_order,
    balance: raw.balance,
    createdAt: raw.created_at,
    updatedAt: raw.updated_at,
  }
}

export async function listAccounts(): Promise<Account[]> {
  const raw: any[] = await invoke("list_accounts")
  return raw.map(toCamelAccount)
}

function toCamelAccountNode(raw: any): AccountNode {
  return {
    id: raw.id,
    parentId: raw.parent_id,
    accountType: raw.account_type,
    code: raw.code,
    name: raw.name,
    description: raw.description,
    currencyCode: raw.currency_code,
    isPlaceholder: raw.is_placeholder,
    isActive: raw.is_active,
    sortOrder: raw.sort_order,
    balance: raw.balance,
    children: raw.children?.map(toCamelAccountNode),
  }
}

export async function getAccountTree(): Promise<AccountNode[]> {
  const raw: any[] = await invoke("get_account_tree")
  return raw.map(toCamelAccountNode)
}

export async function createAccount(payload: CreateAccountPayload): Promise<Account> {
  const raw: any = await invoke("create_account", {
    payload: {
      parent_id: payload.parentId,
      account_type: payload.accountType,
      code: payload.code,
      name: payload.name,
      description: payload.description,
      currency_code: payload.currencyCode,
      is_placeholder: payload.isPlaceholder,
      sort_order: payload.sortOrder,
    },
  })
  return toCamelAccount(raw)
}

export async function updateAccount(payload: UpdateAccountPayload): Promise<Account> {
  const raw: any = await invoke("update_account", {
    payload: {
      id: payload.id,
      parent_id: payload.parentId,
      account_type: payload.accountType,
      code: payload.code,
      name: payload.name,
      description: payload.description,
      currency_code: payload.currencyCode,
      is_placeholder: payload.isPlaceholder,
      is_active: payload.isActive,
      sort_order: payload.sortOrder,
    },
  })
  return toCamelAccount(raw)
}

export async function deleteAccount(id: number): Promise<void> {
  await invoke("delete_account", { id })
}

// ── Transaction API (v3: integer cents) ──────────────────────────────────

export interface CreateSplitPayload {
  accountId: number
  debit: number
  credit: number
  memo?: string | null
  quantity?: string | null
  action?: string | null
}

export interface CreateTransactionPayload {
  currencyCode?: string
  description?: string | null
  notes?: string | null
  num?: string | null
  postDate?: string
  state?: string
  splits: CreateSplitPayload[]
}

export interface ListTransactionsQuery {
  page?: number
  pageSize?: number
  sortField?: string
  sortDirection?: string
  filterText?: string
  filterAccountId?: number | null
  filterState?: string | null
}

export interface PaginatedTransactions {
  transactions: Transaction[]
  total: number
  page: number
  pageSize: number
}

function toCamelSplit(raw: any): Split {
  return {
    id: raw.id,
    transactionId: raw.transaction_id,
    accountId: raw.account_id,
    accountName: raw.account_name,
    accountType: raw.account_type,
    debit: raw.debit,
    credit: raw.credit,
    memo: raw.memo ?? null,
    reconcileState: raw.reconcile_state ?? "n",
    quantity: raw.quantity ?? null,
    action: raw.action ?? null,
  }
}

function toCamelTransaction(raw: any): Transaction {
  return {
    id: raw.id,
    currencyCode: raw.currency_code,
    description: raw.description ?? "",
    notes: raw.notes ?? null,
    num: raw.num ?? null,
    postDate: raw.post_date,
    state: raw.state,
    splits: (raw.splits ?? []).map(toCamelSplit),
    createdAt: raw.created_at,
    updatedAt: raw.updated_at,
  }
}

export async function postTransaction(payload: CreateTransactionPayload): Promise<Transaction> {
  const raw: any = await invoke("post_transaction", {
    payload: {
      currency_code: payload.currencyCode,
      description: payload.description,
      notes: payload.notes,
      num: payload.num,
      post_date: payload.postDate,
      state: payload.state,
      splits: payload.splits.map((s) => ({
        account_id: s.accountId,
        debit: s.debit,
        credit: s.credit,
        memo: s.memo,
        quantity: s.quantity,
        action: s.action,
      })),
    },
  })
  return toCamelTransaction(raw)
}

export async function listTransactions(
  query: ListTransactionsQuery = {},
): Promise<PaginatedTransactions> {
  const raw: any = await invoke("list_transactions", {
    query: {
      page: query.page,
      page_size: query.pageSize,
      sort_field: query.sortField,
      sort_direction: query.sortDirection,
      filter_text: query.filterText,
      filter_account_id: query.filterAccountId,
      filter_state: query.filterState,
    },
  })
  return {
    transactions: (raw.transactions ?? []).map(toCamelTransaction),
    total: raw.total,
    page: raw.page,
    pageSize: raw.page_size,
  }
}

export async function voidTransaction(id: number): Promise<void> {
  return await invoke("void_transaction", { id })
}

// ── Dashboard API ────────────────────────────────────────────────────────

export interface DashboardSummary {
  totalAssets: number
  totalLiabilities: number
  totalIncome: number
  totalExpenses: number
  totalCash: number
}

export async function getDashboardSummary(): Promise<DashboardSummary> {
  const raw: any = await invoke("get_dashboard_summary")
  return {
    totalAssets: raw.total_assets,
    totalLiabilities: raw.total_liabilities,
    totalIncome: raw.total_income,
    totalExpenses: raw.total_expenses,
    totalCash: raw.total_cash,
  }
}

// ── Reconciliation API ─────────────────────────────────────────────────────

export interface StartReconciliationPayload {
  accountId: number
  statementDate: string
  endingBalance: number
  startingBalance: number
}

function toCamelReconcileSplit(raw: any): any {
  return {
    id: raw.id,
    transactionId: raw.transaction_id,
    accountId: raw.account_id,
    accountName: raw.account_name,
    accountType: raw.account_type,
    postDate: raw.post_date,
    description: raw.description,
    num: raw.num,
    debit: raw.debit,
    credit: raw.credit,
    memo: raw.memo,
    reconcileState: raw.reconcile_state,
  }
}

function toCamelReconciliationData(raw: any): ReconciliationData {
  return {
    session: {
      id: raw.session.id,
      accountId: raw.session.account_id,
      statementDate: raw.session.statement_date,
      endingBalance: raw.session.ending_balance,
      startingBalance: raw.session.starting_balance,
      state: raw.session.state,
      createdAt: raw.session.created_at,
      completedAt: raw.session.completed_at,
    },
    splits: (raw.splits ?? []).map(toCamelReconcileSplit),
    clearedTotal: raw.cleared_total,
    difference: raw.difference,
  }
}

export async function startReconciliation(
  payload: StartReconciliationPayload,
): Promise<ReconciliationData> {
  const raw: any = await invoke("start_reconciliation", {
    payload: {
      account_id: payload.accountId,
      statement_date: payload.statementDate,
      ending_balance: payload.endingBalance,
      starting_balance: payload.startingBalance,
    },
  })
  return toCamelReconciliationData(raw)
}

export async function getReconciliationData(
  sessionId: number,
): Promise<ReconciliationData> {
  const raw: any = await invoke("get_reconciliation_data", {
    sessionId,
  })
  return toCamelReconciliationData(raw)
}

export async function toggleSplitReconcileState(
  splitId: number,
): Promise<string> {
  return await invoke("toggle_split_reconcile_state", { splitId })
}

export async function finalizeReconciliation(
  sessionId: number,
): Promise<ReconciliationData> {
  const raw: any = await invoke("finalize_reconciliation", { sessionId })
  return toCamelReconciliationData(raw)
}

export async function checkReconciledSplit(splitId: number): Promise<boolean> {
  return await invoke("check_reconciled_split", { splitId })
}

// ── Import API ─────────────────────────────────────────────────────────────

function toCamelImportPreviewItem(raw: any): ImportPreviewItem {
  return {
    rowIndex: raw.row_index,
    date: raw.date,
    description: raw.description,
    amountCents: raw.amount_cents,
    debit: raw.debit,
    credit: raw.credit,
    memo: raw.memo,
    num: raw.num,
    isDuplicate: raw.is_duplicate,
    matchedTransactionId: raw.matched_transaction_id,
  }
}

function toCamelImportProfile(raw: any): ImportProfile {
  return {
    id: raw.id,
    name: raw.name,
    fileFormat: raw.file_format,
    columnMapping: raw.column_mapping,
    defaultAccountId: raw.default_account_id,
    createdAt: raw.created_at,
    updatedAt: raw.updated_at,
  }
}

export async function listImportProfiles(): Promise<ImportProfile[]> {
  const raw: any[] = await invoke("list_import_profiles")
  return raw.map(toCamelImportProfile)
}

export async function saveImportProfile(payload: {
  name: string
  fileFormat: string
  columnMapping: string
  defaultAccountId?: number | null
}): Promise<ImportProfile> {
  const raw: any = await invoke("save_import_profile", {
    payload: {
      name: payload.name,
      file_format: payload.fileFormat,
      column_mapping: payload.columnMapping,
      default_account_id: payload.defaultAccountId ?? null,
    },
  })
  return toCamelImportProfile(raw)
}

export async function deleteImportProfile(id: number): Promise<void> {
  return await invoke("delete_import_profile", { id })
}

export async function previewCsvImport(
  rawContent: string,
  headerMap: string,
): Promise<ImportPreviewItem[]> {
  const raw: any[] = await invoke("preview_csv_import", {
    rawContent,
    headerMap,
  })
  return raw.map(toCamelImportPreviewItem)
}

export async function commitImport(payload: {
  accountId: number
  items: any[]
}): Promise<CommitImportResult> {
  const raw: any = await invoke("commit_import", {
    payload: {
      account_id: payload.accountId,
      items: payload.items.map((i: any) => ({
        row_index: i.rowIndex,
        date: i.date,
        description: i.description,
        amount_cents: i.amountCents,
        debit: i.debit,
        credit: i.credit,
        memo: i.memo,
        num: i.num,
        is_duplicate: i.isDuplicate,
        matched_transaction_id: i.matchedTransactionId,
      })),
    },
  })
  return { imported: raw.imported, skipped: raw.skipped }
}

// ── Recurring Transactions API ──────────────────────────────────────────

function toCamelRecurringSplit(raw: any): RecurringSplit {
  return {
    id: raw.id,
    recurringId: raw.recurring_id,
    accountId: raw.account_id,
    debit: raw.debit,
    credit: raw.credit,
    memo: raw.memo ?? null,
  }
}

function toCamelRecurringTransaction(raw: any): RecurringTransaction {
  return {
    id: raw.id,
    frequency: raw.frequency,
    intervalCount: raw.interval_count,
    nextDate: raw.next_date,
    endDate: raw.end_date ?? null,
    autoExecute: raw.auto_execute,
    lastGenerated: raw.last_generated ?? null,
    isActive: raw.is_active,
    description: raw.description ?? "",
    currencyCode: raw.currency_code ?? "USD",
    notes: raw.notes ?? null,
    num: raw.num ?? null,
    splits: (raw.splits ?? []).map(toCamelRecurringSplit),
    createdAt: raw.created_at,
    updatedAt: raw.updated_at,
  }
}

export async function listRecurringTransactions(): Promise<RecurringTransaction[]> {
  const raw: any[] = await invoke("list_recurring_transactions")
  return raw.map(toCamelRecurringTransaction)
}

export interface CreateRecurringPayload {
  frequency: string
  intervalCount?: number
  nextDate: string
  endDate?: string | null
  autoExecute?: boolean
  isActive?: boolean
  description: string
  currencyCode?: string
  notes?: string | null
  num?: string | null
  splits: { accountId: number; debit: number; credit: number; memo?: string | null }[]
}

export async function createRecurringTransaction(
  payload: CreateRecurringPayload,
): Promise<RecurringTransaction> {
  const raw: any = await invoke("create_recurring_transaction", {
    payload: {
      frequency: payload.frequency,
      interval_count: payload.intervalCount,
      next_date: payload.nextDate,
      end_date: payload.endDate ?? null,
      auto_execute: payload.autoExecute,
      is_active: payload.isActive,
      description: payload.description,
      currency_code: payload.currencyCode,
      notes: payload.notes,
      num: payload.num,
      splits: payload.splits.map((s) => ({
        account_id: s.accountId,
        debit: s.debit,
        credit: s.credit,
        memo: s.memo ?? null,
      })),
    },
  })
  return toCamelRecurringTransaction(raw)
}

export interface UpdateRecurringPayload {
  id: number
  frequency?: string
  intervalCount?: number
  nextDate?: string
  endDate?: string | null
  autoExecute?: boolean
  isActive?: boolean
  description?: string
  currencyCode?: string
  notes?: string | null
  num?: string | null
  splits?: { accountId: number; debit: number; credit: number; memo?: string | null }[]
}

export async function updateRecurringTransaction(
  payload: UpdateRecurringPayload,
): Promise<RecurringTransaction> {
  const raw: any = await invoke("update_recurring_transaction", {
    payload: {
      id: payload.id,
      frequency: payload.frequency,
      interval_count: payload.intervalCount,
      next_date: payload.nextDate,
      end_date: payload.endDate,
      auto_execute: payload.autoExecute,
      is_active: payload.isActive,
      description: payload.description,
      currency_code: payload.currencyCode,
      notes: payload.notes,
      num: payload.num,
      splits: payload.splits?.map((s) => ({
        account_id: s.accountId,
        debit: s.debit,
        credit: s.credit,
        memo: s.memo ?? null,
      })),
    },
  })
  return toCamelRecurringTransaction(raw)
}

export async function deleteRecurringTransaction(id: number): Promise<void> {
  return await invoke("delete_recurring_transaction", { id })
}

export async function executeRecurringTransaction(
  id: number,
): Promise<RecurringTransaction> {
  const raw: any = await invoke("execute_recurring_transaction", { id })
  return toCamelRecurringTransaction(raw)
}
