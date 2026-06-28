import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { Transaction, RecurringTransaction } from "@/types"
import { useAccountingEngine } from "@/composables/useAccountingEngine"

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

  const { validateBalanced } = useAccountingEngine()

  const totalTransactions = computed(() => transactions.value.length)

  const filteredTransactions = computed(() => {
    let result = [...transactions.value]

    if (filterText.value) {
      const q = filterText.value.toLowerCase()
      result = result.filter(
        (t) =>
          (t.description && t.description.toLowerCase().includes(q)) ||
          (t.payee && t.payee.toLowerCase().includes(q)) ||
          (t.number && t.number.toLowerCase().includes(q)),
      )
    }

    if (filterAccountId.value !== null) {
      result = result.filter((t) =>
        t.splits.some((s) => s.accountId === filterAccountId.value),
      )
    }

    if (filterState.value) {
      result = result.filter((t) => t.state === filterState.value)
    }

    result.sort((a, b) => {
      let cmp = 0
      if (sortField.value === "date") {
        cmp = a.date.localeCompare(b.date)
      } else if (sortField.value === "payee") {
        cmp = (a.payee || "").localeCompare(b.payee || "")
      } else if (sortField.value === "amount") {
        cmp = computeTxnTotal(a).localeCompare(computeTxnTotal(b))
      } else if (sortField.value === "state") {
        cmp = a.state.localeCompare(b.state)
      }
      return sortDirection.value === "desc" ? -cmp : cmp
    })

    return result
  })

  const paginatedTransactions = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    return filteredTransactions.value.slice(start, start + pageSize.value)
  })

  const totalPages = computed(() =>
    Math.ceil(filteredTransactions.value.length / pageSize.value),
  )

  function computeTxnTotal(txn: Transaction): string {
    return txn.splits
      .reduce((acc, s) => {
        return acc.plus(s.debitAmount || "0").minus(s.creditAmount || "0")
      }, new Decimal(0))
      .abs()
      .toString()
  }

  function addTransaction(txn: Transaction) {
    validateBalanced(txn.splits)
    transactions.value.unshift(txn)
  }

  function updateTransaction(id: number, updates: Partial<Transaction>) {
    const idx = transactions.value.findIndex((t) => t.id === id)
    if (idx !== -1) {
      const updated = { ...transactions.value[idx], ...updates }
      if (updates.splits) {
        validateBalanced(updates.splits)
      }
      transactions.value[idx] = updated
    }
  }

  function removeTransaction(id: number) {
    transactions.value = transactions.value.filter((t) => t.id !== id)
  }

  function setTransactions(data: Transaction[]) {
    transactions.value = data
  }

  function setSort(field: string) {
    if (sortField.value === field) {
      sortDirection.value = sortDirection.value === "asc" ? "desc" : "asc"
    } else {
      sortField.value = field
      sortDirection.value = "desc"
    }
  }

  function setPage(page: number) {
    currentPage.value = page
  }

  function addRecurring(rt: RecurringTransaction) {
    recurringTransactions.value.push(rt)
  }

  function removeRecurring(id: number) {
    recurringTransactions.value = recurringTransactions.value.filter(
      (r) => r.id !== id,
    )
  }

  const upcomingRecurring = computed(() => {
    const today = new Date().toISOString().split("T")[0]
    return recurringTransactions.value
      .filter((r) => r.isActive && r.nextDate <= today)
      .sort((a, b) => a.nextDate.localeCompare(b.nextDate))
  })

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
    filteredTransactions,
    paginatedTransactions,
    totalPages,
    upcomingRecurring,
    computeTxnTotal,
    addTransaction,
    updateTransaction,
    removeTransaction,
    setTransactions,
    setSort,
    setPage,
    addRecurring,
    removeRecurring,
  }
})

import Decimal from "decimal.js"
