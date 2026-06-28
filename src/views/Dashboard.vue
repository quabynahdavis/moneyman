<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/Card.vue"
import { ArrowUp, Wallet, TrendingUp, TrendingDown, PiggyBank } from "@lucide/vue"

const sampleNetWorth = "$24,582.32"
const sampleIncome = "$6,240.00"
const sampleExpenses = "$4,183.21"
const sampleCash = "$12,450.89"
</script>

<template>
  <div class="space-y-6">
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Net Worth</CardTitle>
          <Wallet class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ sampleNetWorth }}</div>
          <p class="text-xs text-muted-foreground flex items-center gap-1 mt-1">
            <ArrowUp class="h-3 w-3 text-emerald-500" />
            <span class="text-emerald-500">+$1,240.32</span> from last month
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Income (This Month)</CardTitle>
          <TrendingUp class="h-4 w-4 text-emerald-500" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-emerald-500">{{ sampleIncome }}</div>
          <p class="text-xs text-muted-foreground mt-1">vs $5,800.00 budgeted</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Expenses (This Month)</CardTitle>
          <TrendingDown class="h-4 w-4 text-rose-500" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-rose-500">{{ sampleExpenses }}</div>
          <p class="text-xs text-muted-foreground mt-1">vs $4,500.00 budgeted</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Total Cash</CardTitle>
          <PiggyBank class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ sampleCash }}</div>
          <p class="text-xs text-muted-foreground mt-1">Across 4 accounts</p>
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
            No recent transactions to display. Add one from the Ledger.
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Upcoming Scheduled</CardTitle>
        </CardHeader>
        <CardContent>
          <p class="text-sm text-muted-foreground">
            No upcoming scheduled transactions.
          </p>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
