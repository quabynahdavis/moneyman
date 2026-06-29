<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion"
import { Folder, FileText } from "@lucide/vue"
import { Badge, Button } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"

defineOptions({ name: "AccountTree" })

defineProps<{
  accounts: AccountNode[]
}>()

const emit = defineEmits<{
  (e: "select-account", account: AccountNode): void
}>()
</script>

<template>
  <Accordion type="multiple" class="w-full">
    <template v-for="account in accounts" :key="account.id">
      <AccordionItem
        v-if="account.children && account.children.length > 0"
        :value="account.id.toString()"
        class="border-b-0"
      >
        <AccordionTrigger
          class="py-2 hover:no-underline hover:bg-muted/50 px-2 rounded-md transition-colors items-center"
          :class="{ 'opacity-60 italic': account.isPlaceholder }"
        >
          <div class="flex items-center gap-2 text-sm flex-1 min-w-0">
            <Folder class="h-4 w-4 text-muted-foreground shrink-0" />
            <span v-if="account.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ account.code }}</span>
            <span class="font-medium truncate">{{ account.name }}</span>
            <Badge variant="secondary" class="text-xs shrink-0 ml-auto">{{ ACCOUNT_TYPE_LABELS[account.accountType] }}</Badge>
            <span
              class="font-mono tabular-nums text-right w-28 shrink-0"
              :class="isNegative(account.balance) ? 'text-rose-500' : ''"
            >{{ formatMoney(account.balance) }}</span>
            <Button
              variant="ghost"
              size="sm"
              class="h-7 px-2 opacity-0 group-hover/accordion-trigger:opacity-100 shrink-0 font-medium"
              @click.stop="emit('select-account', account)"
            >View</Button>
          </div>
        </AccordionTrigger>

        <AccordionContent class="pb-0 pl-4 border-l ml-4 mt-1">
          <AccountTree
            :accounts="account.children"
            @select-account="(a: AccountNode) => emit('select-account', a)"
          />
        </AccordionContent>
      </AccordionItem>

      <div
        v-else
        @click="emit('select-account', account)"
        class="flex items-center gap-2 py-2 px-2 text-sm hover:bg-muted/50 rounded-md cursor-pointer transition-colors group"
        :class="{ 'opacity-60 italic': account.isPlaceholder }"
      >
        <FileText class="h-4 w-4 text-primary shrink-0" />
        <span v-if="account.code" class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0">{{ account.code }}</span>
        <span class="font-medium truncate flex-1">{{ account.name }}</span>
        <Badge variant="secondary" class="text-xs shrink-0">{{ ACCOUNT_TYPE_LABELS[account.accountType] }}</Badge>
        <span
          class="font-mono tabular-nums text-right w-28 shrink-0"
          :class="isNegative(account.balance) ? 'text-rose-500' : ''"
        >{{ formatMoney(account.balance) }}</span>
        <Button
          variant="ghost"
          size="sm"
          class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0 font-medium"
          @click.stop="emit('select-account', account)"
        >View</Button>
      </div>
    </template>
  </Accordion>
</template>
