import { invoke } from "@tauri-apps/api/core"
import type { Account, Transaction } from "@/types"

// ── Account API ──────────────────────────────────────────────────────────

export interface CreateAccountPayload {
  parentId: number | null
  accountType: string
  code: string | null
  name: string
  description: string | null
  currencyCode?: string
  placeholder?: boolean
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
  placeholder?: boolean
  isActive?: boolean
  sortOrder?: number
}

export async function listAccounts(): Promise<Account[]> {
  return await invoke("list_accounts")
}

export async function createAccount(payload: CreateAccountPayload): Promise<Account> {
  return await invoke("create_account", {
    payload: {
      parent_id: payload.parentId,
      account_type: payload.accountType,
      code: payload.code,
      name: payload.name,
      description: payload.description,
      currency_code: payload.currencyCode,
      placeholder: payload.placeholder,
      sort_order: payload.sortOrder,
    },
  })
}

export async function updateAccount(payload: UpdateAccountPayload): Promise<Account> {
  return await invoke("update_account", {
    payload: {
      id: payload.id,
      parent_id: payload.parentId,
      account_type: payload.accountType,
      code: payload.code,
      name: payload.name,
      description: payload.description,
      currency_code: payload.currencyCode,
      placeholder: payload.placeholder,
      is_active: payload.isActive,
      sort_order: payload.sortOrder,
    },
  })
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

export async function postTransaction(payload: CreateTransactionPayload): Promise<Transaction> {
  return await invoke("post_transaction", {
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
}

export async function listTransactions(
  query: ListTransactionsQuery = {},
): Promise<PaginatedTransactions> {
  return await invoke("list_transactions", {
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
