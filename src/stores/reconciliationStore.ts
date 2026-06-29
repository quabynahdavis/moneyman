import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { ReconciliationSession, ReconcileSplit, ReconciliationData } from "@/types"
import * as api from "@/services/api"

export const useReconciliationStore = defineStore("reconciliation", () => {
  const session = ref<ReconciliationSession | null>(null)
  const splits = ref<ReconcileSplit[]>([])
  const clearedTotal = ref(0)
  const difference = ref(0)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const unreconciledSplits = computed(() =>
    splits.value.filter((s) => s.reconcileState !== "r"),
  )

  const clearedSplits = computed(() =>
    splits.value.filter((s) => s.reconcileState === "c"),
  )

  const statementBalance = computed(() => session.value?.endingBalance ?? 0)

  // Difference = cleared_total - ending_balance (reactive)
  const currentDifference = computed(() => difference.value)

  const isBalanced = computed(() => difference.value === 0)

  const isActive = computed(() => session.value?.state === "open")

  async function startSession(payload: api.StartReconciliationPayload) {
    loading.value = true
    error.value = null
    try {
      const data = await api.startReconciliation(payload)
      applyData(data)
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to start reconciliation"
      throw error.value
    } finally {
      loading.value = false
    }
  }

  async function loadSession(sessionId: number) {
    loading.value = true
    error.value = null
    try {
      const data = await api.getReconciliationData(sessionId)
      applyData(data)
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to load reconciliation"
      throw error.value
    } finally {
      loading.value = false
    }
  }

  async function toggleSplit(splitId: number) {
    try {
      const newState = await api.toggleSplitReconcileState(splitId)
      const idx = splits.value.findIndex((s) => s.id === splitId)
      if (idx !== -1) {
        splits.value[idx] = { ...splits.value[idx], reconcileState: newState as "n" | "c" }
      }
      recomputeTotals()
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to toggle split"
      throw error.value
    }
  }

  async function finalize() {
    if (!session.value?.id) return
    try {
      const data = await api.finalizeReconciliation(session.value.id)
      applyData(data)
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to finalize"
      throw error.value
    }
  }

  function applyData(data: ReconciliationData) {
    session.value = data.session
    splits.value = data.splits
    clearedTotal.value = data.clearedTotal
    difference.value = data.difference
  }

  function recomputeTotals() {
    const total = clearedSplits.value.reduce((sum, s) => sum + s.debit - s.credit, 0)
    clearedTotal.value = total
    difference.value = total - statementBalance.value
  }

  function reset() {
    session.value = null
    splits.value = []
    clearedTotal.value = 0
    difference.value = 0
    error.value = null
  }

  return {
    session,
    splits,
    clearedTotal,
    difference,
    loading,
    error,
    unreconciledSplits,
    clearedSplits,
    statementBalance,
    currentDifference,
    isBalanced,
    isActive,
    startSession,
    loadSession,
    toggleSplit,
    finalize,
    reset,
  }
})
