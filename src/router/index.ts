import { createRouter, createWebHistory } from "vue-router"

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/dashboard",
    },
    {
      path: "/dashboard",
      name: "dashboard",
      component: () => import("@/views/Dashboard.vue"),
      meta: { title: "Dashboard", icon: "LayoutDashboard" },
    },
    {
      path: "/accounts",
      name: "accounts",
      component: () => import("@/views/Accounts.vue"),
      meta: { title: "Chart of Accounts", icon: "BookType" },
    },
    {
      path: "/ledger",
      name: "ledger",
      component: () => import("@/views/Ledger.vue"),
      meta: { title: "General Ledger", icon: "NotebookText" },
    },
    {
      path: "/ledger/:accountId",
      name: "account-ledger",
      component: () => import("@/views/Ledger.vue"),
      meta: { title: "Account Register", icon: "NotebookText" },
    },
    {
      path: "/reconciliation",
      name: "reconciliation",
      component: () => import("@/views/Reconciliation.vue"),
      meta: { title: "Reconciliation", icon: "CheckCircle" },
    },
    {
      path: "/reports",
      name: "reports",
      component: () => import("@/views/Reports.vue"),
      meta: { title: "Reports", icon: "BarChart3" },
    },
    {
      path: "/scheduled",
      name: "scheduled",
      component: () => import("@/views/ScheduledTransactions.vue"),
      meta: { title: "Scheduled Transactions", icon: "CalendarClock" },
    },
    {
      path: "/invoices",
      name: "invoices",
      component: () => import("@/views/Invoices.vue"),
      meta: { title: "Invoices & Bills", icon: "FileText" },
    },
    {
      path: "/contacts",
      name: "contacts",
      component: () => import("@/views/Contacts.vue"),
      meta: { title: "Contacts", icon: "Users" },
    },
    {
      path: "/budgets",
      name: "budgets",
      component: () => import("@/views/Budgets.vue"),
      meta: { title: "Budgets", icon: "PiggyBank" },
    },
  ],
})

export default router
