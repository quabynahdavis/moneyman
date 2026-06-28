import { defineStore } from "pinia"
import { ref } from "vue"

export interface AppSettings {
  baseCurrency: string
  defaultAccountId: number | null
  reconcileDefaultAccountId: number | null
  autoSaveInterval: number
  showScheduledReminders: boolean
  priceQuoteProvider: "manual" | "yahoo" | "alpha_vantage"
  priceQuoteApiKey: string
  dateFormat: "YYYY-MM-DD" | "MM/DD/YYYY" | "DD/MM/YYYY"
  numberFormat: "1,234.56" | "1.234,56" | "1 234,56"
  showInactiveAccounts: boolean
  enableMultiCurrency: boolean
  enableInvestments: boolean
  enableInvoicing: boolean
}

const DEFAULT_SETTINGS: AppSettings = {
  baseCurrency: "USD",
  defaultAccountId: null,
  reconcileDefaultAccountId: null,
  autoSaveInterval: 60,
  showScheduledReminders: true,
  priceQuoteProvider: "manual",
  priceQuoteApiKey: "",
  dateFormat: "YYYY-MM-DD",
  numberFormat: "1,234.56",
  showInactiveAccounts: false,
  enableMultiCurrency: true,
  enableInvestments: true,
  enableInvoicing: true,
}

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS })

  const stored = localStorage.getItem("moneyman-settings")
  if (stored) {
    try {
      settings.value = { ...DEFAULT_SETTINGS, ...JSON.parse(stored) }
    } catch {
      // ignore
    }
  }

  function updateSetting<K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K],
  ) {
    settings.value[key] = value
    persist()
  }

  function updateSettings(partial: Partial<AppSettings>) {
    Object.assign(settings.value, partial)
    persist()
  }

  function resetSettings() {
    settings.value = { ...DEFAULT_SETTINGS }
    persist()
  }

  function persist() {
    localStorage.setItem("moneyman-settings", JSON.stringify(settings.value))
  }

  return {
    settings,
    updateSetting,
    updateSettings,
    resetSettings,
  }
})
