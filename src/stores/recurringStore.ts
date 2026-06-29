import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { RecurringTransaction, RecurringFrequency } from "@/types"
import * as api from "@/services/api"

export const useRecurringStore = defineStore("recurring", () => {
  const transactions = ref<RecurringTransaction[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const upcoming = computed(() => {
    const today = new Date().toISOString().split("T")[0]
    return transactions.value
      .filter((r) => r.isActive && r.nextDate <= today)
      .sort((a, b) => a.nextDate.localeCompare(b.nextDate))
  })

  const activeTransactions = computed(() =>
    transactions.value.filter((r) => r.isActive),
  )

  const pastTransactions = computed(() =>
    transactions.value.filter((r) => !r.isActive),
  )

  async function fetchAll() {
    loading.value = true
    error.value = null
    try {
      transactions.value = await api.listRecurringTransactions()
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to fetch recurring transactions"
    } finally {
      loading.value = false
    }
  }

  async function create(payload: api.CreateRecurringPayload): Promise<RecurringTransaction> {
    error.value = null
    try {
      const txn = await api.createRecurringTransaction(payload)
      transactions.value.push(txn)
      return txn
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to create recurring transaction"
      throw error.value
    }
  }

  async function update(payload: api.UpdateRecurringPayload): Promise<RecurringTransaction> {
    error.value = null
    try {
      const txn = await api.updateRecurringTransaction(payload)
      const idx = transactions.value.findIndex((t) => t.id === payload.id)
      if (idx >= 0) transactions.value[idx] = txn
      return txn
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to update recurring transaction"
      throw error.value
    }
  }

  async function remove(id: number) {
    error.value = null
    try {
      await api.deleteRecurringTransaction(id)
      transactions.value = transactions.value.filter((t) => t.id !== id)
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to delete recurring transaction"
      throw error.value
    }
  }

  async function execute(id: number): Promise<RecurringTransaction> {
    error.value = null
    try {
      const updated = await api.executeRecurringTransaction(id)
      const idx = transactions.value.findIndex((t) => t.id === id)
      if (idx >= 0) transactions.value[idx] = updated
      return updated
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to execute recurring transaction"
      throw error.value
    }
  }

  function getById(id: number): RecurringTransaction | undefined {
    return transactions.value.find((t) => t.id === id)
  }

  return {
    transactions,
    loading,
    error,
    upcoming,
    activeTransactions,
    pastTransactions,
    fetchAll,
    create,
    update,
    remove,
    execute,
    getById,
  }
})
