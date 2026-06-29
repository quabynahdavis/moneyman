<script setup lang="ts">
import { useUiStore } from "@/stores/uiStore"
import {
  LayoutDashboard,
  BookType,
  NotebookText,
  CheckCircle,
  BarChart3,
  CalendarClock,
  FileText,
  Users,
  PiggyBank,
  Settings,
  Menu,
  Sun,
  Moon,
  Search,
  ArrowLeft,
  ArrowRight,
  RefreshCw,
} from "@lucide/vue"
import { ref, computed, watch } from "vue"
import { useRouter, useRoute } from "vue-router"
import { Toaster } from "vue-sonner"
import ConfirmDialog from "@/components/ConfirmDialog.vue"

const ui = useUiStore()
const router = useRouter()
const route = useRoute()

// Custom navigation stack (not browser history)
const navStack = ref<string[]>([])
const navIndex = ref(-1)
const isProgrammaticNav = ref(false)

watch(
  () => route.fullPath,
  (to, from) => {
    if (isProgrammaticNav.value) {
      isProgrammaticNav.value = false
      return
    }
    if (from && to !== from) {
      if (navIndex.value < navStack.value.length - 1) {
        navStack.value = navStack.value.slice(0, navIndex.value + 1)
      }
      navStack.value.push(to)
      navIndex.value = navStack.value.length - 1
    }
  },
  { immediate: true },
)

const canGoBack = computed(() => navIndex.value > 0)
const canGoForward = computed(() => navIndex.value < navStack.value.length - 1)

function goBack() {
  if (canGoBack.value) {
    navIndex.value--
    isProgrammaticNav.value = true
    router.replace(navStack.value[navIndex.value])
  }
}

function goForward() {
  if (canGoForward.value) {
    navIndex.value++
    isProgrammaticNav.value = true
    router.replace(navStack.value[navIndex.value])
  }
}

const navItems = [
  { name: "dashboard", label: "Dashboard", icon: LayoutDashboard },
  { name: "accounts", label: "Accounts", icon: BookType },
  { name: "ledger", label: "Ledger", icon: NotebookText },
  { name: "reconciliation", label: "Reconcile", icon: CheckCircle },
  { name: "scheduled", label: "Scheduled", icon: CalendarClock },
  { name: "invoices", label: "Invoices", icon: FileText },
  { name: "contacts", label: "Contacts", icon: Users },
  { name: "budgets", label: "Budgets", icon: PiggyBank },
  { name: "reports", label: "Reports", icon: BarChart3 },
  { name: "settings", label: "Settings", icon: Settings },
]

const sidebarWidth = computed(() => (ui.sidebarCollapsed ? "w-16" : "w-56"))

function toggleTheme() {
  ui.setTheme(ui.isDark ? "light" : "dark")
}

// Route-specific refresh handlers
function refreshCurrentPage() {
  const name = route.name as string
  if (name === "accounts") {
    import("@/stores/accountStore").then((m) => m.useAccountStore().fetchAccounts())
  } else if (name === "dashboard") {
    // Dashboard refreshes on mount, just re-mount by toggling key
    window.location.reload()
  } else if (name === "ledger" || name === "account-ledger") {
    import("@/stores/transactionStore").then((m) => m.useTransactionStore().fetchTransactions())
  }
}
</script>

<template>
  <div class="flex h-screen overflow-hidden bg-background">
    <aside
      :class="[
        sidebarWidth,
        'flex flex-col border-r bg-card transition-all duration-200 ease-in-out',
      ]"
    >
      <div class="flex h-14 items-center border-b px-4">
        <div
          v-if="!ui.sidebarCollapsed"
          class="flex items-center gap-2 font-semibold text-lg"
        >
          <div class="h-6 w-6 rounded bg-primary" />
          <span class="truncate">Moneyman</span>
        </div>
        <button
          class="ml-auto rounded-md p-1.5 hover:bg-accent"
          @click="ui.toggleSidebar()"
        >
          <Menu class="h-5 w-5" />
        </button>
      </div>

      <nav class="flex-1 space-y-1 p-2 overflow-y-auto">
        <button
          v-for="item in navItems"
          :key="item.name"
          @click="router.push({ name: item.name })"
          :class="[
            'flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
            route.name === item.name
              ? 'bg-accent text-accent-foreground'
              : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground',
          ]"
        >
          <component :is="item.icon" class="h-4 w-4 shrink-0" />
          <span v-if="!ui.sidebarCollapsed" class="truncate">
            {{ item.label }}
          </span>
        </button>
      </nav>

      <div class="border-t p-2">
        <button
          @click="toggleTheme"
          class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:bg-accent"
        >
          <component :is="ui.isDark ? Sun : Moon" class="h-4 w-4 shrink-0" />
          <span v-if="!ui.sidebarCollapsed">
            {{ ui.isDark ? "Light" : "Dark" }} Mode
          </span>
        </button>
      </div>
    </aside>

    <main class="flex flex-1 flex-col overflow-hidden">
      <header class="flex h-14 items-center border-b bg-card px-4 gap-2">
        <button
          class="rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground disabled:opacity-30"
          :disabled="!canGoBack"
          @click="goBack"
        >
          <ArrowLeft class="h-4 w-4" />
        </button>
        <button
          class="rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground disabled:opacity-30"
          :disabled="!canGoForward"
          @click="goForward"
        >
          <ArrowRight class="h-4 w-4" />
        </button>
        <h1 class="text-lg font-semibold ml-1">
          {{ route.meta.title || "Moneyman" }}
        </h1>
        <button
          class="rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground ml-0.5"
          @click="refreshCurrentPage"
          title="Refresh"
        >
          <RefreshCw class="h-4 w-4" />
        </button>
        <div class="ml-auto flex items-center gap-2">
          <div class="relative">
            <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
            <input
              class="h-9 w-48 rounded-md border bg-background pl-8 pr-3 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              placeholder="Search..."
            />
          </div>
        </div>
      </header>

      <div class="flex-1 overflow-auto p-6">
        <router-view />
      </div>
    </main>
  </div>
  <Toaster
    position="bottom-right"
    :rich-colors="true"
    :close-button="true"
    :duration="4000"
  />
  <ConfirmDialog />
</template>
