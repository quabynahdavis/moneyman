import { defineStore } from "pinia"
import { ref, computed, watch } from "vue"

export type Theme = "light" | "dark" | "system"
export type SidebarState = "expanded" | "collapsed"

export const useUiStore = defineStore("ui", () => {
  const theme = ref<Theme>(
    (localStorage.getItem("moneyman-theme") as Theme) || "system",
  )
  const sidebar = ref<SidebarState>(
    (localStorage.getItem("moneyman-sidebar") as SidebarState) || "expanded",
  )
  const activeModal = ref<string | null>(null)
  const modalData = ref<Record<string, unknown>>({})

  const isDark = computed(() => {
    if (theme.value === "dark") return true
    if (theme.value === "light") return false
    return window.matchMedia("(prefers-color-scheme: dark)").matches
  })

  const sidebarCollapsed = computed(() => sidebar.value === "collapsed")

  function setTheme(t: Theme) {
    theme.value = t
    localStorage.setItem("moneyman-theme", t)
  }

  function toggleSidebar() {
    sidebar.value = sidebar.value === "expanded" ? "collapsed" : "expanded"
    localStorage.setItem("moneyman-sidebar", sidebar.value)
  }

  function setSidebar(state: SidebarState) {
    sidebar.value = state
    localStorage.setItem("moneyman-sidebar", state)
  }

  function openModal(name: string, data: Record<string, unknown> = {}) {
    activeModal.value = name
    modalData.value = data
  }

  function closeModal() {
    activeModal.value = null
    modalData.value = {}
  }

  watch(isDark, (val) => {
    document.documentElement.classList.toggle("dark", val)
  }, { immediate: true })

  return {
    theme,
    sidebar,
    activeModal,
    modalData,
    isDark,
    sidebarCollapsed,
    setTheme,
    toggleSidebar,
    setSidebar,
    openModal,
    closeModal,
  }
})
