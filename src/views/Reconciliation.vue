<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle, CardDescription, Button } from "@/components/ui"
import { Wallet, ArrowRight } from "@lucide/vue"
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Reconciliation</h2>
        <p class="text-sm text-muted-foreground">
          Match ledger entries against bank statements
        </p>
      </div>
      <Button>
        <Wallet class="h-4 w-4 mr-2" />
        New Reconciliation
      </Button>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Active Reconciliations</CardTitle>
        <CardDescription>No active reconciliations</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="flex flex-col items-center justify-center py-8 text-muted-foreground">
          <ArrowRight class="h-12 w-12 mb-4" />
          <p class="text-sm">
            Select an account and start a reconciliation to match your ledger against
            a bank statement.
          </p>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
