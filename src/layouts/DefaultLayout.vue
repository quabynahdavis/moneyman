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
  Menu,
  Sun,
  Moon,
  Search,
  ArrowLeft,
  ArrowRight,
} from "@lucide/vue"
import { computed } from "vue"
import { useRouter, useRoute } from "vue-router"

const ui = useUiStore()
const router = useRouter()
const route = useRoute()

const canGoBack = computed(() => window.history.length > 1)

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
]

const sidebarWidth = computed(() => (ui.sidebarCollapsed ? "w-16" : "w-56"))

function toggleTheme() {
  ui.setTheme(ui.isDark ? "light" : "dark")
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
          @click="router.back()"
        >
          <ArrowLeft class="h-4 w-4" />
        </button>
        <button
          class="rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground"
          @click="router.forward()"
        >
          <ArrowRight class="h-4 w-4" />
        </button>
        <h1 class="text-lg font-semibold ml-1">
          {{ route.meta.title || "Moneyman" }}
        </h1>
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
</template>
