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

// ── Transaction API ──────────────────────────────────────────────────────

export interface CreateSplitPayload {
  accountId: number
  debitAmount: string
  creditAmount: string
  memo?: string | null
  quantity?: string | null
  action?: string | null
}

export interface CreateTransactionPayload {
  currencyCode?: string
  description?: string | null
  notes?: string | null
  payee?: string | null
  number?: string | null
  date?: string
  datePosted?: string
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
    debitAmount: raw.debit_amount,
    creditAmount: raw.credit_amount,
    memo: raw.memo ?? null,
    quantity: raw.quantity ?? null,
    action: raw.action ?? null,
    reconciledDate: raw.reconciled_date ?? null,
  }
}

function toCamelTransaction(raw: any): Transaction {
  return {
    id: raw.id,
    currencyCode: raw.currency_code,
    description: raw.description ?? null,
    notes: raw.notes ?? null,
    payee: raw.payee ?? null,
    number: raw.number ?? null,
    date: raw.date,
    datePosted: raw.date_posted,
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
      payee: payload.payee,
      number: payload.number,
      date: payload.date,
      date_posted: payload.datePosted,
      state: payload.state,
      splits: payload.splits.map((s) => ({
        account_id: s.accountId,
        debit_amount: s.debitAmount,
        credit_amount: s.creditAmount,
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
  netWorth: string
  totalAssets: string
  totalLiabilities: string
  totalIncome: string
  totalExpenses: string
  totalCash: string
}

export async function getDashboardSummary(): Promise<DashboardSummary> {
  const raw: any = await invoke("get_dashboard_summary")
  return {
    netWorth: raw.net_worth,
    totalAssets: raw.total_assets,
    totalLiabilities: raw.total_liabilities,
    totalIncome: raw.total_income,
    totalExpenses: raw.total_expenses,
    totalCash: raw.total_cash,
  }
}
