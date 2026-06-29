<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion"
import {
  DropdownMenuRoot as DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
} from "reka-ui"
import {
  ContextMenuRoot as ContextMenu,
  ContextMenuTrigger,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuPortal,
} from "reka-ui"
import { Folder, FileText, Pencil, Trash2 } from "@lucide/vue"
import { Badge } from "@/components/ui"
import { formatMoney, isNegative } from "@/utils/decimal"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountNode } from "@/types"
import { computed } from "vue"

defineOptions({ name: "AccountTree" })

const props = defineProps<{
  accounts: AccountNode[]
}>()

const emit = defineEmits<{
  (e: "select-account" | "edit-account" | "delete-account", account: AccountNode): void
}>()

const defaultValues = computed(() =>
  props.accounts
    .filter((a) => a.children && a.children.length > 0)
    .map((a) => a.id.toString()),
)

function select(account: AccountNode) {
  emit("select-account", account)
}

function edit(account: AccountNode) {
  emit("edit-account", account)
}

function del(account: AccountNode) {
  emit("delete-account", account)
}

const itemClass =
  "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
const contentClass =
  "z-50 min-w-40 rounded-md border bg-popover p-1 text-popover-foreground shadow-md"
</script>

<template>
  <Accordion type="multiple" class="w-full" :default-value="defaultValues">
    <template v-for="account in accounts" :key="account.id">
      <!-- Parent row -->
      <AccordionItem
        v-if="account.children && account.children.length > 0"
        :value="account.id.toString()"
        class="border-b-0"
      >
        <AccordionTrigger
          class="py-1.5 hover:no-underline hover:bg-muted/50 px-2 rounded-md transition-colors items-center"
          :class="{ 'opacity-60 italic': account.isPlaceholder }"
        >
          <ContextMenu>
            <ContextMenuTrigger class="flex flex-1 items-center gap-2 text-sm min-w-0">
              <Folder class="h-4 w-4 text-muted-foreground shrink-0" />
              <span
                v-if="account.code"
                class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0"
              >{{ account.code }}</span>
              <span class="font-medium truncate">{{ account.name }}</span>
              <Badge
                variant="secondary"
                class="text-xs shrink-0 ml-auto"
              >{{ ACCOUNT_TYPE_LABELS[account.accountType] }}</Badge>
              <span
                class="font-mono tabular-nums text-right w-28 shrink-0"
                :class="isNegative(account.balance) ? 'text-rose-500' : ''"
              >{{ formatMoney(account.balance) }}</span>
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <div
                    class="h-7 w-7 opacity-0 group-hover/accordion-trigger:opacity-100 shrink-0 flex items-center justify-center rounded-md hover:bg-muted transition-colors cursor-pointer"
                    @click.stop
                  >⋮</div>
                </DropdownMenuTrigger>
                <DropdownMenuPortal>
                  <DropdownMenuContent :align="'end'" :class="contentClass">
                    <DropdownMenuItem :class="itemClass" @select="edit(account)">
                      <Pencil class="h-4 w-4 mr-2" /> Edit
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      :class="[itemClass, 'text-destructive focus:text-destructive focus:bg-destructive/10']"
                      @select="del(account)"
                    >
                      <Trash2 class="h-4 w-4 mr-2" /> Delete
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenuPortal>
              </DropdownMenu>
              <span
                class="h-7 px-2 opacity-0 group-hover/accordion-trigger:opacity-100 shrink-0 font-medium inline-flex items-center justify-center rounded-md text-sm cursor-pointer hover:bg-muted transition-colors select-none"
                @click.stop="select(account)"
              >View</span>
            </ContextMenuTrigger>
            <ContextMenuPortal>
              <ContextMenuContent :class="contentClass">
                <ContextMenuItem :class="itemClass" @select="edit(account)">
                  <Pencil class="h-4 w-4 mr-2" /> Edit
                </ContextMenuItem>
                <ContextMenuItem
                  :class="[itemClass, 'text-destructive focus:text-destructive focus:bg-destructive/10']"
                  @select="del(account)"
                >
                  <Trash2 class="h-4 w-4 mr-2" /> Delete
                </ContextMenuItem>
              </ContextMenuContent>
            </ContextMenuPortal>
          </ContextMenu>
        </AccordionTrigger>

        <AccordionContent class="pb-0 pl-4 border-l ml-4 mt-1">
          <AccountTree
            :accounts="account.children"
            @select-account="(a: AccountNode) => emit('select-account', a)"
            @edit-account="(a: AccountNode) => emit('edit-account', a)"
            @delete-account="(a: AccountNode) => emit('delete-account', a)"
          />
        </AccordionContent>
      </AccordionItem>

      <!-- Leaf row -->
      <ContextMenu v-else>
        <ContextMenuTrigger
          @click="select(account)"
          class="flex items-center gap-2 py-1.5 px-2 text-sm hover:bg-muted/50 rounded-md cursor-pointer transition-colors group"
          :class="{ 'opacity-60 italic': account.isPlaceholder }"
        >
          <FileText class="h-4 w-4 text-primary shrink-0" />
          <span
            v-if="account.code"
            class="text-xs text-muted-foreground font-mono tabular-nums w-14 shrink-0"
          >{{ account.code }}</span>
          <span class="font-medium truncate flex-1">{{ account.name }}</span>
          <Badge
            variant="secondary"
            class="text-xs shrink-0"
          >{{ ACCOUNT_TYPE_LABELS[account.accountType] }}</Badge>
          <span
            class="font-mono tabular-nums text-right w-28 shrink-0"
            :class="isNegative(account.balance) ? 'text-rose-500' : ''"
          >{{ formatMoney(account.balance) }}</span>
          <DropdownMenu>
            <DropdownMenuTrigger as-child>
              <div
                class="h-7 w-7 opacity-0 group-hover:opacity-100 shrink-0 flex items-center justify-center rounded-md hover:bg-muted transition-colors cursor-pointer"
                @click.stop
              >⋮</div>
            </DropdownMenuTrigger>
            <DropdownMenuPortal>
              <DropdownMenuContent :align="'end'" :class="contentClass">
                <DropdownMenuItem :class="itemClass" @select="edit(account)">
                  <Pencil class="h-4 w-4 mr-2" /> Edit
                </DropdownMenuItem>
                <DropdownMenuItem
                  :class="[itemClass, 'text-destructive focus:text-destructive focus:bg-destructive/10']"
                  @select="del(account)"
                >
                  <Trash2 class="h-4 w-4 mr-2" /> Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenuPortal>
          </DropdownMenu>
          <span
            class="h-7 px-2 opacity-0 group-hover:opacity-100 shrink-0 font-medium inline-flex items-center justify-center rounded-md text-sm cursor-pointer hover:bg-muted transition-colors select-none"
            @click.stop="select(account)"
          >View</span>
        </ContextMenuTrigger>
        <ContextMenuPortal>
          <ContextMenuContent :class="contentClass">
            <ContextMenuItem :class="itemClass" @select="edit(account)">
              <Pencil class="h-4 w-4 mr-2" /> Edit
            </ContextMenuItem>
            <ContextMenuItem
              :class="[itemClass, 'text-destructive focus:text-destructive focus:bg-destructive/10']"
              @select="del(account)"
            >
              <Trash2 class="h-4 w-4 mr-2" /> Delete
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenuPortal>
      </ContextMenu>
    </template>
  </Accordion>
</template>
