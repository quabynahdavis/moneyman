<script setup lang="ts">
import { ref, computed } from "vue"
import { ChevronDown, ChevronRight, Pencil, Trash2, EyeOff } from "@lucide/vue"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui"
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
import type { Transaction, AccountType } from "@/types"
import { formatCents } from "@/utils/decimal"
import { useLedgerStore, ACCOUNT_CONTEXT_LABELS, type AccountContext } from "@/stores/ledgerStore"
import { getAccountNormalBalance } from "@/types/account"

const props = defineProps<{
  transactions: Transaction[]
  accountId: number | null
  accountType?: string | null
}>()

const emit = defineEmits<{
  (e: "void-transaction", txnId: number): void
  (e: "edit-transaction", txnId: number): void
}>()

const ledgerStore = useLedgerStore()
const expanded = ref<Record<string, boolean>>({})

const normalBalance = computed<"debit" | "credit">(() =>
  props.accountType ? getAccountNormalBalance(props.accountType as AccountType) : "debit",
)

const context = computed<AccountContext>(() => {
  const ctx: Record<string, AccountContext> = {
    ASSET: "ASSET", BANK: "BANK", CASH: "CASH",
    LIABILITY: "LIABILITY", CREDIT_CARD: "CREDIT_CARD", PAYABLE: "PAYABLE",
    EXPENSE: "EXPENSE", INCOME: "INCOME",
  }
  return ctx[props.accountType ?? ""] || "ASSET"
})

const labels = computed(() => ACCOUNT_CONTEXT_LABELS[context.value])

interface LedgerRow {
  id: string
  txnId: number
  postDate: string
  num: string | null
  description: string
  transferAccount: string
  debit: number
  credit: number
  balance: number
  state: string
  splitCount: number
}

const ledgerRows = computed<LedgerRow[]>(() => {
  const rows: LedgerRow[] = []

  for (const txn of props.transactions) {
    if (!txn.splits) continue

    const matchingSplits = props.accountId
      ? txn.splits.filter((s) => s.accountId === props.accountId)
      : txn.splits

    if (props.accountId && matchingSplits.length === 0) continue

    const otherSplits = props.accountId
      ? txn.splits.filter((s) => s.accountId !== props.accountId)
      : []

    let transferAccount = ""
    if (txn.splits.length === 2 && (otherSplits.length === 1 || !props.accountId)) {
      const splitA = txn.splits[0]
      const opposite = props.accountId
        ? otherSplits[0]
        : txn.splits.find((s) => s.accountId !== splitA.accountId)
      transferAccount = opposite?.accountName ?? ""
    } else if (txn.splits.length > 2) {
      transferAccount = "\u2014 Split Transaction \u2014"
    }

    const totalDebit = matchingSplits.reduce((sum, s) => sum + s.debit, 0)
    const totalCredit = matchingSplits.reduce((sum, s) => sum + s.credit, 0)

    rows.push({
      id: `txn-${txn.id}`,
      txnId: txn.id!,
      postDate: txn.postDate,
      num: txn.num,
      description: txn.description,
      transferAccount,
      debit: totalDebit,
      credit: totalCredit,
      balance: 0,
      state: txn.state,
      splitCount: txn.splits.length,
    })
  }

  rows.sort((a, b) => a.postDate.localeCompare(b.postDate))

  let running = 0
  for (const row of rows) {
    const change = row.debit - row.credit
    running += normalBalance.value === "debit" ? change : -change
    row.balance = running
  }

  return rows.reverse()
})

function toggleExpand(txnId: number) {
  const key = `txn-${txnId}`
  expanded.value = { ...expanded.value, [key]: !expanded.value[key] }
}

function isExpanded(txnId: number): boolean {
  return !!expanded.value[`txn-${txnId}`]
}

function getSplitsForTxn(txnId: number): Transaction["splits"] {
  const txn = props.transactions.find((t) => t.id === txnId)
  return txn?.splits ?? []
}

function stateBadge(state: string): string {
  switch (state) {
    case "RECONCILED": return "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
    case "CLEARED": return "bg-blue-500/10 text-blue-600 dark:text-blue-400"
    case "VOID": return "bg-destructive/10 text-destructive"
    default: return "bg-muted text-muted-foreground"
  }
}

const dropdownItemClass = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-highlighted:bg-muted data-highlighted:text-foreground"
</script>

<template>
  <div class="rounded-md border overflow-x-auto">
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead class="w-8"></TableHead>
          <TableHead class="whitespace-nowrap">Date</TableHead>
          <TableHead class="whitespace-nowrap">Num</TableHead>
          <TableHead class="whitespace-nowrap">Description</TableHead>
          <TableHead class="whitespace-nowrap">Transfer</TableHead>
          <TableHead class="whitespace-nowrap text-right">{{ labels.debit }}</TableHead>
          <TableHead class="whitespace-nowrap text-right">{{ labels.credit }}</TableHead>
          <TableHead class="whitespace-nowrap text-right">Balance</TableHead>
          <TableHead class="whitespace-nowrap">R</TableHead>
          <TableHead class="w-10"></TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <template v-if="ledgerRows.length > 0">
          <template v-for="row in ledgerRows" :key="row.id">
            <ContextMenu>
              <ContextMenuTrigger as-child>
                <TableRow
                  class="cursor-pointer hover:bg-muted/50"
                  :class="{ 'bg-muted/30': isExpanded(row.txnId) }"
                  @click="toggleExpand(row.txnId)"
                >
                  <TableCell class="py-1.5">
                    <component
                      :is="isExpanded(row.txnId) ? ChevronDown : ChevronRight"
                      class="h-4 w-4 text-muted-foreground"
                    />
                  </TableCell>
                  <TableCell class="py-1.5 whitespace-nowrap font-mono text-xs">{{ row.postDate }}</TableCell>
                  <TableCell class="py-1.5 whitespace-nowrap font-mono text-xs text-muted-foreground">{{ row.num || "" }}</TableCell>
                  <TableCell class="py-1.5 text-sm font-medium">{{ row.description }}</TableCell>
                  <TableCell class="py-1.5 text-sm text-muted-foreground">{{ row.transferAccount }}</TableCell>
                  <TableCell class="py-1.5 text-right font-mono text-sm tabular-nums">{{ row.debit ? formatCents(row.debit) : "" }}</TableCell>
                  <TableCell class="py-1.5 text-right font-mono text-sm tabular-nums">{{ row.credit ? formatCents(row.credit) : "" }}</TableCell>
                  <TableCell class="py-1.5 text-right font-mono text-sm tabular-nums font-medium">{{ formatCents(row.balance) }}</TableCell>
                  <TableCell class="py-1.5">
                    <span
                      v-if="row.state !== 'UNRECONCILED'"
                      class="inline-block w-5 text-center text-xs rounded"
                      :class="stateBadge(row.state)"
                    >{{ row.state === "RECONCILED" ? "R" : row.state === "CLEARED" ? "C" : "V" }}</span>
                  </TableCell>
                  <TableCell class="py-1.5" @click.stop>
                    <DropdownMenu>
                      <DropdownMenuTrigger as-child>
                        <div
                          class="h-7 w-7 opacity-0 group-hover/row:opacity-100 shrink-0 flex items-center justify-center rounded-md hover:bg-muted transition-colors cursor-pointer"
                          @click.stop
                        >⋮</div>
                      </DropdownMenuTrigger>
                      <DropdownMenuPortal>
                        <DropdownMenuContent :align="'end'" class="min-w-[120px] rounded-md border bg-popover p-1 shadow-md">
                          <DropdownMenuItem :class="dropdownItemClass" @select="emit('edit-transaction', row.txnId)">
                            <Pencil class="h-4 w-4" /> Edit
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            v-if="row.state !== 'VOID'"
                            :class="dropdownItemClass"
                            @select="emit('void-transaction', row.txnId)"
                          >
                            <EyeOff class="h-4 w-4" /> Void
                          </DropdownMenuItem>
                        </DropdownMenuContent>
                      </DropdownMenuPortal>
                    </DropdownMenu>
                  </TableCell>
                </TableRow>
              </ContextMenuTrigger>
              <ContextMenuPortal>
                <ContextMenuContent class="min-w-[120px] rounded-md border bg-popover p-1 shadow-md">
                  <ContextMenuItem :class="dropdownItemClass" @select="emit('edit-transaction', row.txnId)">
                    <Pencil class="h-4 w-4" /> Edit
                  </ContextMenuItem>
                  <ContextMenuItem
                    v-if="row.state !== 'VOID'"
                    :class="dropdownItemClass"
                    @select="emit('void-transaction', row.txnId)"
                  >
                    <EyeOff class="h-4 w-4" /> Void
                  </ContextMenuItem>
                </ContextMenuContent>
              </ContextMenuPortal>
            </ContextMenu>
            <!-- Expanded splits -->
            <TableRow
              v-if="isExpanded(row.txnId)"
              class="border-b-0"
            >
              <TableCell colspan="10" class="p-0">
                <table class="w-full text-xs bg-muted/10">
                  <thead>
                    <tr class="border-b border-muted-foreground/10">
                      <th class="px-2 py-1 text-left text-muted-foreground font-medium w-8"></th>
                      <th class="px-2 py-1 text-left text-muted-foreground font-medium">Account</th>
                      <th class="px-2 py-1 text-left text-muted-foreground font-medium">Memo</th>
                      <th class="px-2 py-1 text-right text-muted-foreground font-medium">Debit</th>
                      <th class="px-2 py-1 text-right text-muted-foreground font-medium">Credit</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr
                      v-for="split in getSplitsForTxn(row.txnId)"
                      :key="split.id"
                      class="border-b border-muted-foreground/5"
                    >
                      <td class="px-2 py-1"></td>
                      <td class="px-2 py-1 font-mono">
                        <span :class="{ 'font-semibold': accountId && split.accountId === accountId }">
                          {{ split.accountName }}
                        </span>
                      </td>
                      <td class="px-2 py-1 text-muted-foreground">{{ split.memo || "" }}</td>
                      <td class="px-2 py-1 text-right font-mono tabular-nums">{{ split.debit ? formatCents(split.debit) : "" }}</td>
                      <td class="px-2 py-1 text-right font-mono tabular-nums">{{ split.credit ? formatCents(split.credit) : "" }}</td>
                    </tr>
                  </tbody>
                </table>
              </TableCell>
            </TableRow>
          </template>
        </template>
        <TableRow v-else>
          <TableCell colspan="10" class="h-24 text-center text-muted-foreground">
            No transactions found.
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
