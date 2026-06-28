<script setup lang="ts">
import { ref, computed } from "vue"
import { ChevronRight, ChevronDown } from "@lucide/vue"
import { Badge, Button } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"

defineOptions({ name: "AccountTree" })

const props = defineProps<{
  nodes: AccountNode[]
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

interface FlatRow {
  node: AccountNode
  depth: number
  isLast: boolean
  parentIsLast: boolean[]
}

const flatRows = computed(() => {
  const result: FlatRow[] = []
  function walk(nodes: AccountNode[], depth: number, parentIsLast: boolean[]) {
    const len = nodes.length
    for (let i = 0; i < len; i++) {
      const node = nodes[i]
      const isLast = i === len - 1
      result.push({ node, depth, isLast, parentIsLast })
      if (hasChildren(node) && isOpen(node.id)) {
        walk(node.children!, depth + 1, [...parentIsLast, isLast])
      }
    }
  }
  walk(props.nodes, 0, [])
  return result
})
</script>

<template>
  <div class="w-full font-mono text-sm" role="tree">
    <div
      v-for="item in flatRows"
      :key="item.node.id"
      role="treeitem"
      :aria-expanded="hasChildren(item.node) ? isOpen(item.node.id) : undefined"
      class="flex items-center group hover:bg-muted/50 cursor-pointer"
      :class="{ 'opacity-60 italic': item.node.isPlaceholder }"
      @click="hasChildren(item.node) && toggle(item.node.id)"
    >
      <!-- Indent with tree lines -->
      <div class="flex shrink-0" style="width: 20px;">
        <template v-for="(pl, di) in item.parentIsLast" :key="di">
          <div class="w-5 shrink-0 flex justify-center">
            <div v-if="!pl" class="w-px h-full bg-border" />
          </div>
        </template>
      </div>

      <!-- Toggle / branch icon -->
      <button
        class="w-5 h-6 flex items-center justify-center shrink-0 text-muted-foreground focus:outline-none"
        @click.stop="hasChildren(item.node) && toggle(item.node.id)"
        :tabindex="hasChildren(item.node) ? 0 : -1"
      >
        <template v-if="hasChildren(item.node)">
          <ChevronRight v-if="!isOpen(item.node.id)" class="h-4 w-4" />
          <ChevronDown v-else class="h-4 w-4" />
        </template>
        <div v-else class="w-3 h-px bg-border" />
      </button>

      <!-- Code -->
      <span
        v-if="item.node.code"
        class="text-xs text-muted-foreground tabular-nums w-16 shrink-0"
      >
        {{ item.node.code }}
      </span>

      <!-- Name -->
      <span class="flex-1 truncate min-w-0 ml-1">
        {{ item.node.name }}
      </span>

      <!-- Type badge -->
      <Badge variant="secondary" class="text-xs shrink-0 ml-2">
        {{ ACCOUNT_TYPE_LABELS[item.node.accountType] }}
      </Badge>

      <!-- Balance -->
      <span
        :class="[
          'text-sm tabular-nums w-28 shrink-0 text-right ml-2',
          isNegative(item.node.balance) ? 'text-rose-500' : '',
        ]"
      >
        {{ formatMoney(item.node.balance) }}
      </span>

      <!-- View button -->
      <Button
        variant="ghost"
        size="sm"
        class="h-6 px-2 opacity-0 group-hover:opacity-100 shrink-0 ml-1"
        @click.stop="emit('select', item.node)"
      >View</Button>
    </div>

    <div v-if="flatRows.length === 0" class="p-8 text-center text-sm text-muted-foreground">
      No accounts found.
    </div>
  </div>
</template>
