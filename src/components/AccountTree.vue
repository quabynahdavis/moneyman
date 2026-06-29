<script setup lang="ts">
import { ChevronRight } from "@lucide/vue"
import { Badge, Button, Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"

const props = withDefaults(
  defineProps<{
    nodes: AccountNode[]
    depth?: number
  }>(),
  {
    depth: 0,
  },
)

const emit = defineEmits<{
  select: [node: AccountNode]
}>()

function onRowClick(node: AccountNode) {
  emit("select", node)
}
</script>

<template>
  <div class="w-full">
    <Accordion type="multiple" class="w-full">
      <template v-for="node in nodes" :key="node.id">
        <!-- Parent account with sub-accounts: render as AccordionItem -->
        <AccordionItem
          v-if="node.children && node.children.length > 0"
          :value="String(node.id)"
          class="border-b last:border-b-0"
        >
          <div
            class="flex items-center gap-2 px-4 py-2 hover:bg-muted/50 select-none text-sm group cursor-pointer"
            :class="{ 'opacity-60 italic': node.isPlaceholder }"
            :style="{ paddingLeft: `${depth * 24 + 16}px` }"
            @click="onRowClick(node)"
          >
            <AccordionTrigger
              class="w-4 h-4 p-0 flex items-center justify-center shrink-0 hover:no-underline [&[data-state=open]>svg]:rotate-90"
              @click.stop
            >
              <template #icon>
                <ChevronRight class="h-4 w-4 text-muted-foreground transition-transform duration-150" />
              </template>
            </AccordionTrigger>

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
              class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0 font-medium"
              @click.stop="emit('select', node)"
            >View</Button>
          </div>

          <AccordionContent>
            <!-- Recursive call for children -->
            <AccountTree
              :nodes="node.children"
              :depth="depth + 1"
              @select="emit('select', $event)"
            />
          </AccordionContent>
        </AccordionItem>

        <!-- Leaf account: render as simple row -->
        <div
          v-else
          class="flex items-center gap-2 px-4 py-2 hover:bg-muted/50 select-none text-sm border-b last:border-b-0 group cursor-pointer"
          :style="{ paddingLeft: `${depth * 24 + 16}px` }"
          @click="onRowClick(node)"
        >
          <div class="w-4 h-4 shrink-0" />
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
            class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0 font-medium"
            @click.stop="emit('select', node)"
          >View</Button>
        </div>
      </template>
    </Accordion>
  </div>
</template>
