import { invoke } from "@tauri-apps/api/core"
import type { Account, AccountNode, Split, Transaction } from "@/types"

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
