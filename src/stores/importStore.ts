import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { WizardStep, ImportPreviewItem, ImportProfile, ColumnMapping } from "@/types"
import * as api from "@/services/api"

export const useImportStore = defineStore("import", () => {
  const currentStep = ref<WizardStep>("upload")
  const rawContent = ref("")
  const fileName = ref("")
  const fileFormat = ref<"CSV" | "OFX">("CSV")

  // Column mapping: internal field -> column index
  const columnMapping = ref<ColumnMapping>({})

  // Preview items after parsing
  const previewItems = ref<ImportPreviewItem[]>([])
  const selectedAccountId = ref<number | null>(null)

  // Import profiles
  const profiles = ref<ImportProfile[]>([])
  const selectedProfileId = ref<number | null>(null)

  // Results
  const commitResult = ref<{ imported: number; skipped: number } | null>(null)

  const loading = ref(false)
  const error = ref<string | null>(null)

  const canProceed = computed(() => {
    switch (currentStep.value) {
      case "upload":
        return rawContent.value.length > 0
      case "mapping":
        return columnMapping.value.date !== undefined
      case "preview":
        return previewItems.value.length > 0 && selectedAccountId.value !== null
      case "commit":
        return true
    }
  })

  const totalAmount = computed(() =>
    previewItems.value.reduce((sum, i) => sum + i.amountCents, 0),
  )

  async function loadProfiles() {
    try {
      profiles.value = await api.listImportProfiles()
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message
    }
  }

  async function saveProfile(name: string) {
    try {
      const profile = await api.saveImportProfile({
        name,
        fileFormat: fileFormat.value,
        columnMapping: JSON.stringify(columnMapping.value),
        defaultAccountId: selectedAccountId.value,
      })
      profiles.value.push(profile)
      return profile
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message
    }
  }

  function setFile(raw: string, name: string) {
    rawContent.value = raw
    fileName.value = name
    if (name.toLowerCase().endsWith(".ofx") || name.toLowerCase().endsWith(".qfx")) {
      fileFormat.value = "OFX"
    } else {
      fileFormat.value = "CSV"
    }
    currentStep.value = "mapping"
  }

  async function runPreview() {
    loading.value = true
    error.value = null
    try {
      const items = await api.previewCsvImport(
        rawContent.value,
        JSON.stringify(columnMapping.value),
      )
      previewItems.value = items
      currentStep.value = "preview"
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to parse file"
    } finally {
      loading.value = false
    }
  }

  async function commit() {
    if (!selectedAccountId.value) return
    loading.value = true
    error.value = null
    try {
      const result = await api.commitImport({
        accountId: selectedAccountId.value,
        items: previewItems.value.map((i) => ({
          ...i,
          isDuplicate: false,
          matchedTransactionId: undefined,
        })),
      })
      commitResult.value = result
      currentStep.value = "commit"
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to import"
    } finally {
      loading.value = false
    }
  }

  function goToStep(step: WizardStep) {
    currentStep.value = step
  }

  function reset() {
    currentStep.value = "upload"
    rawContent.value = ""
    fileName.value = ""
    columnMapping.value = {}
    previewItems.value = []
    selectedAccountId.value = null
    commitResult.value = null
    error.value = null
  }

  return {
    currentStep,
    rawContent,
    fileName,
    fileFormat,
    columnMapping,
    previewItems,
    selectedAccountId,
    profiles,
    selectedProfileId,
    commitResult,
    loading,
    error,
    canProceed,
    totalAmount,
    loadProfiles,
    saveProfile,
    setFile,
    runPreview,
    commit,
    goToStep,
    reset,
  }
})
