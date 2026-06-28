<script setup lang="ts">
import { computed } from "vue"
import { useRoute } from "vue-router"
import { Card, CardContent } from "@/components/ui/Card.vue"
import { Button } from "@/components/ui/Button.vue"
import { useTransactionStore } from "@/stores/transactionStore"
import { useAccountStore } from "@/stores/accountStore"
import SplitLedgerTable from "@/components/SplitLedgerTable.vue"
import { Plus } from "@lucide/vue"

const route = useRoute()
const txnStore = useTransactionStore()
const accountStore = useAccountStore()

const accountId = computed(() => {
  const id = route.params.accountId
  return id ? Number(id) : null
})

const account = computed(() =>
  accountId.value ? accountStore.getAccountById(accountId.value) : null,
)

const title = computed(() =>
  account.value ? `Register: ${account.value.name}` : "General Ledger",
)
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">{{ title }}</h2>
        <p class="text-sm text-muted-foreground">
          {{ accountId ? "Filtered to selected account" : "All transactions" }}
        </p>
      </div>
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        New Transaction
      </Button>
    </div>

    <Card>
      <CardContent class="p-4">
        <SplitLedgerTable
          :transactions="txnStore.paginatedTransactions"
          :show-account-column="true"
        />
      </CardContent>
    </Card>
  </div>
</template>
