import { createRouter, createWebHistory } from "vue-router"

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", redirect: "/dashboard" },
    {
      path: "/dashboard",
      name: "dashboard",
      component: () => import("@/views/Dashboard.vue"),
      meta: { title: "Dashboard" },
    },
    {
      path: "/accounts",
      name: "accounts",
      component: () => import("@/views/Accounts.vue"),
      meta: { title: "Chart of Accounts" },
    },
    {
      path: "/accounts/new",
      name: "account-new",
      component: () => import("@/views/AccountNew.vue"),
      meta: { title: "New Account" },
    },
    {
      path: "/ledger",
      name: "ledger",
      component: () => import("@/views/Ledger.vue"),
      meta: { title: "General Ledger" },
    },
    {
      path: "/ledger/:accountId",
      name: "account-ledger",
      component: () => import("@/views/Ledger.vue"),
      meta: { title: "Account Register" },
    },
    {
      path: "/transactions/new",
      name: "transaction-new",
      component: () => import("@/views/TransactionNew.vue"),
      meta: { title: "New Transaction" },
    },
    {
      path: "/reconciliation",
      name: "reconciliation",
      component: () => import("@/views/Reconciliation.vue"),
      meta: { title: "Reconciliation" },
    },
    {
      path: "/reports",
      name: "reports",
      component: () => import("@/views/Reports.vue"),
      meta: { title: "Reports" },
    },
    {
      path: "/scheduled",
      name: "scheduled",
      component: () => import("@/views/ScheduledTransactions.vue"),
      meta: { title: "Scheduled Transactions" },
    },
    {
      path: "/invoices",
      name: "invoices",
      component: () => import("@/views/Invoices.vue"),
      meta: { title: "Invoices & Bills" },
    },
    {
      path: "/contacts",
      name: "contacts",
      component: () => import("@/views/Contacts.vue"),
      meta: { title: "Contacts" },
    },
    {
      path: "/budgets",
      name: "budgets",
      component: () => import("@/views/Budgets.vue"),
      meta: { title: "Budgets" },
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/Settings.vue"),
      meta: { title: "Settings" },
    },
  ],
})

export default router
