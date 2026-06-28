<script setup lang="ts">
import { ref } from "vue"
import { ChevronRight } from "@lucide/vue"
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
</script>

<template>
  <div>
    <div
      v-for="node in nodes"
      :key="node.id"
      class="border-b last:border-b-0"
    >
      <!-- Header row -->
      <div
        class="flex items-center gap-2 px-4 py-2.5 hover:bg-muted/50 cursor-pointer select-none text-sm"
        :class="{ 'opacity-60 italic': node.isPlaceholder }"
        :style="{ paddingLeft: `${(depth ?? 0) * 24 + 16}px` }"
        @click="toggle(node.id)"
      >
        <div class="w-4 h-4 flex items-center justify-center shrink-0">
          <ChevronRight
            v-if="node.children && node.children.length > 0"
            class="h-4 w-4 text-muted-foreground transition-transform duration-150"
            :class="{ 'rotate-90': expanded.has(node.id) }"
          />
        </div>

        <span v-if="node.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ node.code }}</span>

        <span class="flex-1 font-medium truncate">{{ node.name }}</span>

        <Badge variant="secondary" class="text-xs shrink-0">{{ ACCOUNT_TYPE_LABELS[node.accountType] }}</Badge>

        <span
          class="font-mono tabular-nums text-right w-28 shrink-0"
          :class="isNegative(node.balance) ? 'text-rose-500' : ''"
        >{{ formatMoney(node.balance) }}</span>

        <Button
          variant="ghost"
          size="sm"
          class="h-7 px-2 opacity-0 hover:opacity-100 shrink-0"
          @click.stop="emit('select', node)"
        >View</Button>
      </div>

      <!-- Children (accordion body) -->
      <div
        v-if="node.children && node.children.length > 0 && expanded.has(node.id)"
        class="overflow-hidden"
      >
        <AccountTree
          :nodes="node.children"
          :depth="(depth ?? 0) + 1"
          @select="emit('select', $event)"
        />
      </div>
    </div>
  </div>
</template>
