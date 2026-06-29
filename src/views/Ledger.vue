<script setup lang="ts">
import { computed, onMounted, watch } from "vue"
import { useRoute, useRouter } from "vue-router"
import { Card, CardContent, Button } from "@/components/ui"
import { useTransactionStore } from "@/stores/transactionStore"
import { useAccountStore } from "@/stores/accountStore"
import { useLedgerStore } from "@/stores/ledgerStore"
import LedgerTable from "@/components/LedgerTable.vue"
import { Plus, RefreshCw } from "@lucide/vue"
import { toast } from "vue-sonner"
import { useConfirm } from "@/composables/useConfirm"

const route = useRoute()
const router = useRouter()
const txnStore = useTransactionStore()
const accountStore = useAccountStore()
const ledgerStore = useLedgerStore()
const { confirm } = useConfirm()

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
    if (account.value) {
      ledgerStore.setAccountContext(account.value.accountType)
    }
  } else {
    txnStore.filterAccountId = null
    ledgerStore.setAccountContext("ASSET")
  }
  txnStore.fetchTransactions()
  accountStore.fetchAccounts()
})

watch(account, (acc) => {
  if (acc) ledgerStore.setAccountContext(acc.accountType)
})

async function onVoidTransaction(txnId: number) {
  const ok = await confirm({
    title: "Void Transaction",
    message: "Are you sure you want to void this transaction?",
    confirmLabel: "Void",
    variant: "destructive",
  })
  if (!ok) return
  try {
    await txnStore.voidExistingTransaction(txnId)
    toast.success("Transaction voided")
  } catch (e: any) {
    toast.error(typeof e === "string" ? e : "Failed to void transaction")
  }
}

function onEditTransaction(txnId: number) {
  toast.info(`Edit transaction #${txnId} - coming soon`)
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
          <RefreshCw class="h-4 w-4 mr-1" /> Refresh
        </Button>
        <Button @click="router.push({ name: 'transaction-new' })">
          <Plus class="h-4 w-4 mr-2" /> New Transaction
        </Button>
      </div>
    </div>

    <Card>
      <CardContent class="p-4">
        <div v-if="txnStore.loading" class="text-center text-sm text-muted-foreground py-8">Loading transactions...</div>
        <div v-else-if="txnStore.error" class="text-center text-sm text-destructive py-8">{{ txnStore.error }}</div>
        <div v-else>
          <LedgerTable
            :transactions="txnStore.paginatedTransactions"
            :account-id="accountId"
            :account-type="account?.accountType ?? null"
            @void-transaction="onVoidTransaction"
            @edit-transaction="onEditTransaction"
          />
        </div>
      </CardContent>
    </Card>
  </div>
</template>
