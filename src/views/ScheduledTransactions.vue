<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/Card.vue"
import { Button } from "@/components/ui/Button.vue"
import { Badge } from "@/components/ui/Badge.vue"
import { CalendarClock, Plus } from "@lucide/vue"

const scheduled = [
  { id: 1, name: "Monthly Rent", amount: "$1,500.00", frequency: "Monthly", nextDate: "2026-07-01", auto: true },
  { id: 2, name: "Netflix Subscription", amount: "$15.99", frequency: "Monthly", nextDate: "2026-07-15", auto: true },
]
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Scheduled Transactions</h2>
        <p class="text-sm text-muted-foreground">Recurring and upcoming transactions</p>
      </div>
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        New Schedule
      </Button>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Upcoming</CardTitle>
        <CardDescription>Transactions scheduled for the near future</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="divide-y">
          <div
            v-for="item in scheduled"
            :key="item.id"
            class="flex items-center gap-4 px-6 py-4"
          >
            <CalendarClock class="h-5 w-5 text-muted-foreground" />
            <div class="flex-1">
              <p class="text-sm font-medium">{{ item.name }}</p>
              <p class="text-xs text-muted-foreground">
                {{ item.frequency }} — next: {{ item.nextDate }}
              </p>
            </div>
            <div class="text-sm font-mono tabular-nums">{{ item.amount }}</div>
            <Badge :variant="item.auto ? 'success' : 'warning'">
              {{ item.auto ? "Auto" : "Manual" }}
            </Badge>
          </div>
          <div v-if="scheduled.length === 0" class="px-6 py-8 text-center text-sm text-muted-foreground">
            No scheduled transactions. Create one to automate recurring bills or income.
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
