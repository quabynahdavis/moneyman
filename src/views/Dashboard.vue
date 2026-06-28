<script setup lang="ts">
import { ref, onMounted } from "vue"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui"
import { ArrowUp, Wallet, TrendingUp, TrendingDown, PiggyBank } from "@lucide/vue"
import { formatMoney, isNegative } from "@/utils/decimal"
import * as api from "@/services/api"

const summary = ref<api.DashboardSummary | null>(null)
const loading = ref(true)

onMounted(async () => {
  try {
    summary.value = await api.getDashboardSummary()
  } catch {
    // fallback to defaults
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="space-y-6">
    <div v-if="loading" class="text-center text-sm text-muted-foreground py-8">
      Loading dashboard...
    </div>
    <template v-else>
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">Net Worth</CardTitle>
            <Wallet class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold">
              {{ summary ? formatMoney(summary.netWorth) : "$0.00" }}
            </div>
            <p class="text-xs text-muted-foreground mt-1">
              Assets: {{ summary ? formatMoney(summary.totalAssets) : "$0" }}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">Income (This Month)</CardTitle>
            <TrendingUp class="h-4 w-4 text-emerald-500" />
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold text-emerald-500">
              {{ summary ? formatMoney(summary.totalIncome) : "$0.00" }}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">Expenses (This Month)</CardTitle>
            <TrendingDown class="h-4 w-4 text-rose-500" />
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold text-rose-500">
              {{ summary ? formatMoney(summary.totalExpenses) : "$0.00" }}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">Total Cash</CardTitle>
            <PiggyBank class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold">
              {{ summary ? formatMoney(summary.totalCash) : "$0.00" }}
            </div>
          </CardContent>
        </Card>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Recent Transactions</CardTitle>
          </CardHeader>
          <CardContent>
            <p class="text-sm text-muted-foreground">
              View recent activity in the Ledger.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Quick Actions</CardTitle>
          </CardHeader>
          <CardContent>
            <p class="text-sm text-muted-foreground">
              Start by adding accounts and posting transactions from the sidebar.
            </p>
          </CardContent>
        </Card>
      </div>
    </template>
  </div>
</template>
