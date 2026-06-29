<script setup lang="ts">
import { onMounted } from "vue"
import { Card, CardContent, CardHeader, CardTitle, Button } from "@/components/ui"
import AccountTree from "@/components/AccountTree.vue"
import { useAccountStore } from "@/stores/accountStore"
import { useRouter } from "vue-router"
import { Plus, RefreshCw } from "@lucide/vue"
import type { AccountNode } from "@/types"
import { toast } from "vue-sonner"
import { useConfirm } from "@/composables/useConfirm"

const accountStore = useAccountStore()
const router = useRouter()
const { confirm } = useConfirm()

onMounted(() => accountStore.fetchAccounts())

function onSelect(node: AccountNode) {
  if (node.children && node.children.length > 0) {
    router.push({ name: "account-detail", params: { accountId: node.id } })
  } else {
    router.push({ name: "account-ledger", params: { accountId: node.id } })
  }
}

function onEdit(node: AccountNode) {
  router.push({ name: "account-new", query: { editId: node.id } })
}

async function onDelete(node: AccountNode) {
  const ok = await confirm({
    title: "Delete Account",
    message: `Delete account "${node.name}"? This cannot be undone.`,
    confirmLabel: "Delete",
    variant: "destructive",
  })
  if (!ok) return
  try {
    await accountStore.deleteExistingAccount(node.id)
    toast.success("Account deleted")
  } catch (e: any) {
    toast.error(typeof e === "string" ? e : "Failed to delete account")
  }
}
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Chart of Accounts</h2>
        <p class="text-sm text-muted-foreground">Manage your account hierarchy</p>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" size="sm" @click="accountStore.fetchAccounts()">
          <RefreshCw class="h-4 w-4 mr-1" /> Refresh
        </Button>
        <Button @click="router.push({ name: 'account-new' })">
          <Plus class="h-4 w-4 mr-2" /> New Account
        </Button>
      </div>
    </div>

    <Card>
      <CardHeader><CardTitle>Account Tree</CardTitle></CardHeader>
      <CardContent class="p-0">
        <div v-if="accountStore.loading" class="p-8 text-center text-sm text-muted-foreground">Loading accounts...</div>
        <div v-else-if="accountStore.error" class="p-8 text-center text-sm text-destructive">{{ accountStore.error }}</div>
        <AccountTree
          v-else
          :accounts="accountStore.treeWithRollup"
          @select-account="onSelect"
          @edit-account="onEdit"
          @delete-account="onDelete"
        />
      </CardContent>
    </Card>
  </div>
</template>
