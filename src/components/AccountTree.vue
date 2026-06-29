<script setup lang="ts">
import { ref } from "vue"
import { ChevronRight } from "@lucide/vue"
import { Badge, Button, Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"

defineOptions({ name: "AccountTree" })

const props = withDefaults(
  defineProps<{
    nodes: AccountNode[]
    depth?: number
  }>(),
  { depth: 0 },
)

const emit = defineEmits<{
  select: [node: AccountNode]
}>()

const openItems = ref<string[]>([])
</script>

<template>
  <Accordion type="multiple" v-model="openItems" class="w-full">
    <template v-for="node in nodes" :key="node.id">
      <AccordionItem
        v-if="node.children && node.children.length > 0"
        :value="String(node.id)"
        class="border-b last:border-b-0"
      >
        <AccordionTrigger
          :class="'flex items-center gap-1.5 px-4 py-2 hover:bg-muted/50 hover:no-underline text-sm group w-full [&[data-state=open]>[data-chevron]]:rotate-90' + (node.isPlaceholder ? ' opacity-60 italic' : '')"
          :style="{ paddingLeft: `${depth * 24 + 16}px` }"
        >
          <ChevronRight
            data-chevron
            class="h-4 w-4 text-muted-foreground transition-transform duration-150 shrink-0"
          />

          <span v-if="node.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ node.code }}</span>
          <span class="flex-1 font-medium truncate text-left">{{ node.name }}</span>
          <Badge variant="secondary" class="text-xs shrink-0 ml-1">{{ ACCOUNT_TYPE_LABELS[node.accountType] }}</Badge>
          <span
            class="font-mono tabular-nums text-right w-28 shrink-0"
            :class="isNegative(node.balance) ? 'text-rose-500' : ''"
          >{{ formatMoney(node.balance) }}</span>
          <Button
            variant="ghost"
            size="sm"
            class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0 font-medium"
            @click.stop="emit('select', node)"
          >View</Button>

          <template #icon></template>
        </AccordionTrigger>

        <AccordionContent>
          <AccountTree
            :nodes="node.children"
            :depth="depth + 1"
            @select="emit('select', $event)"
          />
        </AccordionContent>
      </AccordionItem>

      <div
        v-else
        class="flex items-center gap-1.5 px-4 py-2 hover:bg-muted/50 select-none text-sm border-b last:border-b-0 group"
        :class="node.isPlaceholder ? 'opacity-60 italic' : ''"
        :style="{ paddingLeft: `${depth * 24 + 16}px` }"
      >
        <div class="w-4 h-4 shrink-0" />
        <span v-if="node.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ node.code }}</span>
        <span class="flex-1 font-medium truncate">{{ node.name }}</span>
        <Badge variant="secondary" class="text-xs shrink-0 ml-1">{{ ACCOUNT_TYPE_LABELS[node.accountType] }}</Badge>
        <span
          class="font-mono tabular-nums text-right w-28 shrink-0"
          :class="isNegative(node.balance) ? 'text-rose-500' : ''"
        >{{ formatMoney(node.balance) }}</span>
        <Button
          variant="ghost"
          size="sm"
          class="h-7 px-2 opacity-0 hover:opacity-100 shrink-0 font-medium"
          @click.stop="emit('select', node)"
        >View</Button>
      </div>
    </template>
  </Accordion>
</template>
