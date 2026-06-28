<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle, CardDescription, Button } from "@/components/ui"
import { FileText, Plus } from "@lucide/vue"
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Invoices & Bills</h2>
        <p class="text-sm text-muted-foreground">Manage receivables and payables</p>
      </div>
      <div class="flex gap-2">
        <Button variant="outline">
          <FileText class="h-4 w-4 mr-2" />
          New Bill
        </Button>
        <Button>
          <Plus class="h-4 w-4 mr-2" />
          New Invoice
        </Button>
      </div>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Outstanding</CardTitle>
        <CardDescription>Invoices awaiting payment</CardDescription>
      </CardHeader>
      <CardContent>
        <p class="text-sm text-muted-foreground py-8 text-center">
          No outstanding invoices or bills. Create one to get started.
        </p>
      </CardContent>
    </Card>
  </div>
</template>
