import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { Transaction, RecurringTransaction } from "@/types"
import { useAccountingEngine } from "@/composables/useAccountingEngine"
import * as api from "@/services/api"
import Decimal from "decimal.js"

export const useTransactionStore = defineStore("transactions", () => {
  const transactions = ref<Transaction[]>([])
  const recurringTransactions = ref<RecurringTransaction[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const currentPage = ref(1)
  const pageSize = ref(50)
  const sortField = ref<string>("date")
  const sortDirection = ref<"asc" | "desc">("desc")
  const filterText = ref("")
  const filterAccountId = ref<number | null>(null)
  const filterState = ref<string | null>(null)
  const totalTransactions = ref(0)

  const { validateBalanced } = useAccountingEngine()

  const totalPages = computed(() =>
    Math.ceil(totalTransactions.value / pageSize.value),
  )

  const paginatedTransactions = computed(() => transactions.value)

  const upcomingRecurring = computed(() => {
    const today = new Date().toISOString().split("T")[0]
    return recurringTransactions.value
      .filter((r) => r.isActive && r.nextDate <= today)
      .sort((a, b) => a.nextDate.localeCompare(b.nextDate))
  })

  function computeTxnTotal(txn: Transaction): string {
    return txn.splits
      .reduce((acc, s) => {
        return acc.plus(s.debitAmount || "0").minus(s.creditAmount || "0")
      }, new Decimal(0))
      .abs()
      .toString()
  }

  // ── API-backed actions ──

  async function fetchTransactions() {
    loading.value = true
    error.value = null
    try {
      const result = await api.listTransactions({
        page: currentPage.value,
        pageSize: pageSize.value,
        sortField: sortField.value,
        sortDirection: sortDirection.value,
        filterText: filterText.value,
        filterAccountId: filterAccountId.value,
        filterState: filterState.value,
      })
      transactions.value = result.transactions
      totalTransactions.value = result.total
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to fetch transactions"
    } finally {
      loading.value = false
    }
  }

  async function postNewTransaction(payload: api.CreateTransactionPayload) {
    validateBalanced(
      payload.splits.map((s) => ({
        accountId: s.accountId,
        debitAmount: s.debitAmount,
        creditAmount: s.creditAmount,
        memo: s.memo || null,
        quantity: s.quantity || null,
        action: s.action || null,
        reconciledDate: null,
      })),
    )
    error.value = null
    try {
      const txn = await api.postTransaction(payload)
      transactions.value.unshift(txn)
      totalTransactions.value++
      return txn
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to post transaction"
      throw error.value
    }
  }

  async function voidExistingTransaction(id: number) {
    error.value = null
    try {
      await api.voidTransaction(id)
      transactions.value = transactions.value.filter((t) => t.id !== id)
      totalTransactions.value = Math.max(0, totalTransactions.value - 1)
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to void transaction"
      throw error.value
    }
  }

  function setSort(field: string) {
    if (sortField.value === field) {
      sortDirection.value = sortDirection.value === "asc" ? "desc" : "asc"
    } else {
      sortField.value = field
      sortDirection.value = "desc"
    }
    fetchTransactions()
  }

  function setPage(page: number) {
    currentPage.value = page
    fetchTransactions()
  }

  return {
    transactions,
    recurringTransactions,
    loading,
    error,
    currentPage,
    pageSize,
    sortField,
    sortDirection,
    filterText,
    filterAccountId,
    filterState,
    totalTransactions,
    paginatedTransactions,
    totalPages,
    upcomingRecurring,
    computeTxnTotal,
    fetchTransactions,
    postNewTransaction,
    voidExistingTransaction,
    setSort,
    setPage,
  }
})
