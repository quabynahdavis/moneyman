<script setup lang="ts">
import { ref, computed } from "vue"
import { Card, CardContent, CardHeader, CardTitle, Badge, Button } from "@/components/ui"
import { useAccountStore } from "@/stores/accountStore"
import { formatMoney, isNegative } from "@/utils/decimal"
import { useRouter } from "vue-router"
import { Plus, ChevronRight, ChevronDown, Circle } from "@lucide/vue"
import type { AccountTreeNode } from "@/types"
import { ACCOUNT_TYPE_LABELS } from "@/types"

const accountStore = useAccountStore()
const router = useRouter()
const expanded = ref<Set<number>>(new Set())

function toggle(id: number) {
  const s = new Set(expanded.value)
  if (s.has(id)) s.delete(id)
  else s.add(id)
  expanded.value = s
}

function isOpen(id: number) {
  return expanded.value.has(id)
}

const flatTree = computed(() => {
  const result: { node: AccountTreeNode; depth: number; visible: boolean }[] = []
  function walk(nodes: AccountTreeNode[], depth: number, parentVisible: boolean) {
    for (const n of nodes) {
      const visible = parentVisible
      result.push({ node: n, depth, visible })
      if (n.children.length > 0 && isOpen(n.id)) {
        walk(n.children, depth + 1, true)
      }
    }
  }
  walk(accountStore.accountTree, 0, true)
  return result
})
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Chart of Accounts</h2>
        <p class="text-sm text-muted-foreground">Manage your account hierarchy</p>
      </div>
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        New Account
      </Button>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Account Tree</CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div class="divide-y">
          <div
            v-for="item in flatTree"
            :key="item.node.id"
            v-show="item.visible"
            class="flex items-center gap-2 py-2 px-2 hover:bg-muted/50 rounded-sm cursor-pointer group"
            :style="{ paddingLeft: `${item.depth * 20 + 8}px` }"
            @click="item.node.children.length > 0 && toggle(item.node.id)"
          >
            <component
              :is="item.node.children.length > 0
                ? (isOpen(item.node.id) ? ChevronDown : ChevronRight)
                : Circle"
              class="h-4 w-4 text-muted-foreground shrink-0"
            />
            <span class="flex-1 text-sm font-medium">{{ item.node.name }}</span>
            <Badge variant="secondary" class="text-xs">
              {{ ACCOUNT_TYPE_LABELS[item.node.accountType] }}
            </Badge>
            <span
              :class="[
                'text-sm font-mono tabular-nums w-28 text-right',
                isNegative(item.node.balance) ? 'text-rose-500' : '',
              ]"
            >
              {{ formatMoney(item.node.balance) }}
            </span>
            <Button
              variant="ghost"
              size="sm"
              class="h-7 px-2 opacity-0 group-hover:opacity-100"
              @click.stop="router.push({ name: 'account-ledger', params: { accountId: item.node.id } })"
            >
              View
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
