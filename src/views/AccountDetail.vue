<script setup lang="ts">
import { computed, onMounted } from "vue"
import { useRoute, useRouter } from "vue-router"
import { Card, CardContent, CardHeader, CardTitle, Badge, Button } from "@/components/ui"
import { useAccountStore } from "@/stores/accountStore"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import { ArrowLeft } from "@lucide/vue"
import type { AccountNode } from "@/types"

const route = useRoute()
const router = useRouter()
const accountStore = useAccountStore()

const accountId = computed(() => Number(route.params.accountId))

function findNode(nodes: AccountNode[], id: number): AccountNode | null {
  for (const n of nodes) {
    if (n.id === id) return n
    if (n.children) {
      const found = findNode(n.children, id)
      if (found) return found
    }
  }
  return null
}

const account = computed(() => findNode(accountStore.treeWithRollup, accountId.value))

const children = computed(() => account.value?.children ?? [])

onMounted(() => {
  if (accountStore.accounts.length === 0) {
    accountStore.fetchAccounts()
  }
})

function selectAccount(child: AccountNode) {
  if (child.children && child.children.length > 0) {
    router.push({ name: "account-detail", params: { accountId: child.id } })
  } else {
    router.push({ name: "account-ledger", params: { accountId: child.id } })
  }
}
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center gap-2">
      <Button variant="ghost" size="sm" @click="router.push({ name: 'accounts' })">
        <ArrowLeft class="h-4 w-4" />
      </Button>
      <div v-if="account">
        <h2 class="text-lg font-semibold flex items-center gap-2">
          {{ account.name }}
          <Badge variant="outline" class="text-xs">{{ ACCOUNT_TYPE_LABELS[account.accountType] }}</Badge>
        </h2>
        <p class="text-sm text-muted-foreground">{{ account.code ? `Code: ${account.code}` : "" }} &mdash; {{ children.length }} sub-account{{ children.length === 1 ? "" : "s" }}</p>
      </div>
      <div v-else>
        <h2 class="text-lg font-semibold">Account not found</h2>
      </div>
    </div>

    <Card>
      <CardHeader>
        <CardTitle class="text-base">Sub-Accounts</CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div v-if="children.length === 0" class="p-8 text-center text-sm text-muted-foreground">
          No sub-accounts under this account.
        </div>
        <div v-else class="divide-y">
          <div
            v-for="child in children"
            :key="child.id"
            class="flex items-center gap-2 px-4 py-3 hover:bg-muted/50 group cursor-pointer"
            @click="selectAccount(child)"
          >
            <span v-if="child.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ child.code }}</span>
            <span class="flex-1 font-medium text-sm truncate">{{ child.name }}</span>
            <Badge variant="secondary" class="text-xs shrink-0">{{ ACCOUNT_TYPE_LABELS[child.accountType] }}</Badge>
            <span
              class="font-mono tabular-nums text-right w-28 shrink-0 text-sm"
              :class="isNegative(child.balance) ? 'text-rose-500' : ''"
            >{{ formatMoney(child.balance) }}</span>
            <Button
              variant="ghost"
              size="sm"
              class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0 font-medium"
              @click.stop="selectAccount(child)"
            >
              {{ child.children && child.children.length > 0 ? "View" : "Ledger" }}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
