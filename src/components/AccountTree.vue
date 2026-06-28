<script setup lang="ts">
import { ref, computed } from "vue"
import { ChevronRight, ChevronDown, Minus } from "@lucide/vue"
import { Badge, Button } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"

defineOptions({ name: "AccountTree" })

const props = defineProps<{
  nodes: AccountNode[]
  depth?: number
}>()

const emit = defineEmits<{
  select: [node: AccountNode]
}>()

const expanded = ref<Set<number>>(new Set())

function toggle(id: number) {
  const s = new Set(expanded.value)
  if (s.has(id)) s.delete(id); else s.add(id)
  expanded.value = s
}

function isOpen(id: number) {
  return expanded.value.has(id)
}

function hasChildren(node: AccountNode) {
  return node.children && node.children.length > 0
}

const flatRows = computed(() => {
  const result: { node: AccountNode; depth: number }[] = []
  function walk(nodes: AccountNode[], depth: number) {
    for (const node of nodes) {
      result.push({ node, depth })
      if (hasChildren(node) && isOpen(node.id)) {
        walk(node.children!, depth + 1)
      }
    }
  }
  walk(props.nodes, props.depth ?? 0)
  return result
})
</script>

<template>
  <div class="w-full" role="tree">
    <div
      v-for="item in flatRows"
      :key="item.node.id"
      role="treeitem"
      :aria-expanded="hasChildren(item.node) ? isOpen(item.node.id) : undefined"
      class="flex items-center gap-2 py-2 px-2 hover:bg-muted/50 rounded-sm cursor-pointer group"
      :class="{ 'opacity-60 italic': item.node.isPlaceholder }"
      :style="{ paddingLeft: `${item.depth * 20 + 8}px` }"
      @click="hasChildren(item.node) && toggle(item.node.id)"
    >
      <button
        class="h-4 w-4 flex items-center justify-center shrink-0 text-muted-foreground focus:outline-none"
        @click.stop="hasChildren(item.node) && toggle(item.node.id)"
        :tabindex="hasChildren(item.node) ? 0 : -1"
      >
        <ChevronRight v-if="hasChildren(item.node) && !isOpen(item.node.id)" class="h-4 w-4" />
        <ChevronDown v-else-if="hasChildren(item.node) && isOpen(item.node.id)" class="h-4 w-4" />
        <Minus v-else class="h-3 w-3 text-muted-foreground/40" />
      </button>

      <span v-if="item.node.code" class="text-xs text-muted-foreground font-mono tabular-nums w-16 shrink-0">
        {{ item.node.code }}
      </span>

      <span class="flex-1 text-sm font-medium truncate min-w-0">{{ item.node.name }}</span>

      <Badge variant="secondary" class="text-xs shrink-0">{{ ACCOUNT_TYPE_LABELS[item.node.accountType] }}</Badge>

      <span
        :class="[
          'text-sm font-mono tabular-nums w-28 shrink-0 text-right',
          isNegative(item.node.balance) ? 'text-rose-500' : '',
        ]"
      >{{ formatMoney(item.node.balance) }}</span>

      <Button
        variant="ghost"
        size="sm"
        class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0"
        @click.stop="emit('select', item.node)"
      >View</Button>
    </div>
  </div>
</template>
