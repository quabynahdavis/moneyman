<script setup lang="ts">
import { ref, computed } from "vue"
import { ChevronRight } from "@lucide/vue"
import { Badge, Button } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"

const props = defineProps<{
  nodes: AccountNode[]
}>()

const emit = defineEmits<{
  select: [node: AccountNode]
}>()

const expanded = ref<Set<number>>(new Set())

function collectDescendantIds(node: AccountNode): number[] {
  const ids: number[] = [node.id]
  if (node.children) {
    for (const child of node.children) {
      ids.push(...collectDescendantIds(child))
    }
  }
  return ids
}

function toggle(node: AccountNode) {
  const ids = collectDescendantIds(node)
  const s = new Set(expanded.value)
  if (s.has(node.id)) {
    for (const id of ids) s.delete(id)
  } else {
    for (const id of ids) s.add(id)
  }
  expanded.value = s
}

function isOpen(id: number) {
  return expanded.value.has(id)
}

function onRowClick(node: AccountNode) {
  if (node.children && node.children.length > 0) {
    toggle(node)
  }
}

const flatRows = computed(() => {
  const result: { node: AccountNode; depth: number }[] = []
  function walk(nodes: AccountNode[], depth: number) {
    for (const node of nodes) {
      result.push({ node, depth })
      if (node.children && node.children.length > 0 && isOpen(node.id)) {
        walk(node.children, depth + 1)
      }
    }
  }
  walk(props.nodes, 0)
  return result
})
</script>

<template>
  <div role="tree">
    <div
      v-for="item in flatRows"
      :key="item.node.id"
      role="treeitem"
      :aria-expanded="item.node.children && item.node.children.length > 0 ? isOpen(item.node.id) : undefined"
    >
      <div
        class="flex items-center gap-2 px-4 py-2.5 hover:bg-muted/50 select-none text-sm border-b last:border-b-0 group"
        :class="{
          'opacity-60 italic': item.node.isPlaceholder,
          'cursor-pointer': item.node.children && item.node.children.length > 0,
        }"
        :style="{ paddingLeft: `${item.depth * 24 + 16}px` }"
        @click="onRowClick(item.node)"
      >
        <div
          class="w-4 h-4 flex items-center justify-center shrink-0 cursor-pointer"
          @click.stop="toggle(item.node)"
        >
          <ChevronRight
            v-if="item.node.children && item.node.children.length > 0"
            class="h-4 w-4 text-muted-foreground transition-transform duration-150"
            :class="{ 'rotate-90': isOpen(item.node.id) }"
          />
        </div>

        <span v-if="item.node.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ item.node.code }}</span>

        <span class="flex-1 font-medium truncate">{{ item.node.name }}</span>

        <Badge variant="secondary" class="text-xs shrink-0">{{ ACCOUNT_TYPE_LABELS[item.node.accountType] }}</Badge>

        <span
          class="font-mono tabular-nums text-right w-28 shrink-0"
          :class="isNegative(item.node.balance) ? 'text-rose-500' : ''"
        >{{ formatMoney(item.node.balance) }}</span>

        <Button
          variant="ghost"
          size="sm"
          class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0"
          @click.stop="emit('select', item.node)"
        >View</Button>
      </div>
    </div>
  </div>
</template>
