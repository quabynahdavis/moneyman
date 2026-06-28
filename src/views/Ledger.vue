<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { useRoute } from "vue-router"
import { Card, CardContent, Button } from "@/components/ui"
import Dialog from "@/components/ui/Dialog.vue"
import { useTransactionStore } from "@/stores/transactionStore"
import { useAccountStore } from "@/stores/accountStore"
import SplitLedgerTable from "@/components/SplitLedgerTable.vue"
import TransactionFormDialog from "@/components/TransactionFormDialog.vue"
import { Plus, RefreshCw } from "@lucide/vue"

const route = useRoute()
const txnStore = useTransactionStore()
const accountStore = useAccountStore()
const showCreateDialog = ref(false)

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

onMounted(() => {
  if (accountId.value) {
    txnStore.filterAccountId = accountId.value
  }
  txnStore.fetchTransactions()
  accountStore.fetchAccounts()
})

function onTransactionSaved() {
  showCreateDialog.value = false
  txnStore.fetchTransactions()
}
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
      <div class="flex gap-2">
        <Button variant="outline" size="sm" @click="txnStore.fetchTransactions()">
          <RefreshCw class="h-4 w-4 mr-1" />
          Refresh
        </Button>
        <Button @click="showCreateDialog = true">
          <Plus class="h-4 w-4 mr-2" />
          New Transaction
        </Button>
      </div>
    </div>

    <Card>
      <CardContent class="p-4">
        <div v-if="txnStore.loading" class="text-center text-sm text-muted-foreground py-8">
          Loading transactions...
        </div>
        <div v-else-if="txnStore.error" class="text-center text-sm text-destructive py-8">
          {{ txnStore.error }}
        </div>
        <div v-else>
          <SplitLedgerTable
            :transactions="txnStore.paginatedTransactions"
            :show-account-column="!accountId"
          />
        </div>
      </CardContent>
    </Card>

    <Dialog v-model:open="showCreateDialog">
      <template #default>
        <TransactionFormDialog
          @saved="onTransactionSaved"
          @closed="showCreateDialog = false"
        />
      </template>
    </Dialog>
  </div>
</template>
