<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/Card.vue"
import { Button } from "@/components/ui/Button.vue"
import { Plus } from "@lucide/vue"
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Contacts</h2>
        <p class="text-sm text-muted-foreground">Clients, vendors, and partners</p>
      </div>
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        New Contact
      </Button>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>All Contacts</CardTitle>
        <CardDescription>0 contacts registered</CardDescription>
      </CardHeader>
      <CardContent>
        <p class="text-sm text-muted-foreground py-8 text-center">
          No contacts yet. Add clients and vendors to start invoicing.
        </p>
      </CardContent>
    </Card>
  </div>
</template>
