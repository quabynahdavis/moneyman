<script setup lang="ts">
import { ref } from "vue"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/Card.vue"
import { Button } from "@/components/ui/Button.vue"
import { BarChart3, Table2, Download } from "@lucide/vue"

const activeReport = ref<string | null>(null)

const reports = [
  {
    id: "balance-sheet",
    title: "Balance Sheet",
    description: "Snapshot of assets, liabilities, and equity",
    icon: Table2,
  },
  {
    id: "profit-loss",
    title: "Profit & Loss",
    description: "Income and expenses over a period",
    icon: Table2,
  },
  {
    id: "cash-flow",
    title: "Cash Flow",
    description: "Movement of cash in and out",
    icon: BarChart3,
  },
  {
    id: "portfolio",
    title: "Portfolio Valuation",
    description: "Investment account performance",
    icon: BarChart3,
  },
]
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Reports</h2>
        <p class="text-sm text-muted-foreground">
          Financial statements and analytical reports
        </p>
      </div>
    </div>

    <div class="grid gap-4 md:grid-cols-2">
      <Card
        v-for="report in reports"
        :key="report.id"
        class="cursor-pointer transition-colors hover:bg-accent/50"
        @click="activeReport = report.id"
      >
        <CardHeader class="flex flex-row items-start gap-4">
          <component :is="report.icon" class="h-8 w-8 text-muted-foreground" />
          <div>
            <CardTitle class="text-base">{{ report.title }}</CardTitle>
            <p class="text-sm text-muted-foreground">{{ report.description }}</p>
          </div>
        </CardHeader>
      </Card>
    </div>

    <Card v-if="activeReport">
      <CardHeader class="flex flex-row items-center justify-between">
        <CardTitle>
          {{ reports.find((r) => r.id === activeReport)?.title }}
        </CardTitle>
        <Button variant="outline" size="sm">
          <Download class="h-4 w-4 mr-2" />
          Export
        </Button>
      </CardHeader>
      <CardContent>
        <p class="text-sm text-muted-foreground">
          Report content will be generated here based on ledger data.
        </p>
      </CardContent>
    </Card>
  </div>
</template>
