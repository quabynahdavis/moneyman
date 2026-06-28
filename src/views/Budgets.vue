<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle, CardDescription, Button } from "@/components/ui"
import { Plus } from "@lucide/vue"
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Budgets</h2>
        <p class="text-sm text-muted-foreground">Plan and track spending targets</p>
      </div>
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        New Budget
      </Button>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Active Budgets</CardTitle>
        <CardDescription>No budgets defined</CardDescription>
      </CardHeader>
      <CardContent>
        <p class="text-sm text-muted-foreground py-8 text-center">
          Create a budget to set spending targets for your expense categories.
        </p>
      </CardContent>
    </Card>
  </div>
</template>
