<script setup lang="ts">
import { ref, computed, h } from "vue"
import {
  getCoreRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  getExpandedRowModel,
  useVueTable,
  type SortingState,
  type ExpandedState,
} from "@tanstack/vue-table"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Badge,
} from "@/components/ui"
import { ChevronDown, ChevronRight, ArrowUpDown, ArrowUp, ArrowDown } from "@lucide/vue"
import type { Transaction } from "@/types"
import { formatMoney } from "@/utils/decimal"
import Decimal from "decimal.js"

interface LedgerRow {
  id: string
  type: "transaction" | "split"
  txnId: number
  splitId?: number
  date: string
  payee: string | null
  number: string | null
  description: string | null
  accountName?: string
  memo: string | null
  debit: string
  credit: string
  state: string
  isSubRow: boolean
}

const props = defineProps<{
  transactions: Transaction[]
  showAccountColumn?: boolean
}>()

const sorting = ref<SortingState>([{ id: "date", desc: true }])
const expanded = ref<ExpandedState>({})
const globalFilter = ref("")
const rowSelection = ref({})

const columns = [
  { accessorKey: "date", header: "Date", enableSorting: true },
  { accessorKey: "number", header: "Num", enableSorting: true },
  { accessorKey: "payee", header: "Payee", enableSorting: true },
  { accessorKey: "description", header: "Description" },
  { accessorKey: "accountName", header: "Account" },
  { accessorKey: "memo", header: "Memo" },
  { accessorKey: "debit", header: "Debit", enableSorting: true },
  { accessorKey: "credit", header: "Credit", enableSorting: true },
  { accessorKey: "state", header: "State", enableSorting: true },
]

const ledgerRows = computed(() => {
  const rows: LedgerRow[] = []
  for (const txn of props.transactions) {
    rows.push({
      id: `txn-${txn.id}`,
      type: "transaction",
      txnId: txn.id!,
      date: txn.date,
      payee: txn.payee,
      number: txn.number,
      description: txn.description,
      memo: null,
      debit: txn.splits.reduce((sum, s) => sum.plus(s.debitAmount || "0"), new Decimal(0)).toString(),
      credit: txn.splits.reduce((sum, s) => sum.plus(s.creditAmount || "0"), new Decimal(0)).toString(),
      state: txn.state,
      isSubRow: false,
      accountName: undefined,
    })
    for (const split of txn.splits) {
      rows.push({
        id: `split-${txn.id}-${split.id || 0}`,
        type: "split",
        txnId: txn.id!,
        splitId: split.id,
        date: "",
        payee: null,
        number: null,
        description: null,
        accountName: split.accountName,
        memo: split.memo,
        debit: split.debitAmount,
        credit: split.creditAmount,
        state: "",
        isSubRow: true,
      })
    }
  }
  return rows
})

const visibleColumns = computed(() => {
  return props.showAccountColumn ? columns : columns.filter((c) => c.accessorKey !== "accountName")
})

const table = useVueTable({
  get data() { return ledgerRows.value as LedgerRow[] },
  columns: visibleColumns.value as any[],
  state: {
    sorting: sorting as any,
    expanded: expanded as any,
    globalFilter: globalFilter as any,
    rowSelection: rowSelection as any,
  },
  onSortingChange: (updater: any) => {
    sorting.value = typeof updater === "function" ? updater(sorting.value) : updater
  },
  onExpandedChange: (updater: any) => {
    expanded.value = typeof updater === "function" ? updater(expanded.value) : updater
  },
  onGlobalFilterChange: (updater: any) => {
    globalFilter.value = typeof updater === "function" ? updater(globalFilter.value) : updater
  },
  getCoreRowModel: getCoreRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),
  getSubRows: (row: any) => row.type === "transaction"
    ? ledgerRows.value.filter((r: LedgerRow) => r.type === "split" && r.txnId === row.txnId)
    : [],
  enableExpanding: true,
  enableSorting: true,
  enableMultiSort: false,
  enableRowSelection: false,
})

function formatCellValue(cell: any, columnId: string): string {
  const val = cell.getValue() as string
  if (columnId === "debit") {
    return val && val !== "0" ? formatMoney(val) : ""
  }
  if (columnId === "credit") {
    return val && val !== "0" ? formatMoney(val) : ""
  }
  return val || ""
}

function toggleRow(rowId: string) {
  const current: Record<string, boolean> = expanded.value as Record<string, boolean>
  expanded.value = { ...current, [rowId]: !current[rowId] } as any
}

function getSortIcon(columnId: string) {
  const sort = sorting.value.find((s: { id: string }) => s.id === columnId)
  if (!sort) return ArrowUpDown
  return sort.desc ? ArrowDown : ArrowUp
}
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center gap-2">
      <input
        v-model="globalFilter"
        placeholder="Filter transactions..."
        class="h-9 rounded-md border bg-background px-3 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring w-64"
      />
    </div>
    <div class="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
            <TableHead v-for="header in headerGroup.headers" :key="header.id">
              <button
                v-if="header.column.getCanSort()"
                class="flex items-center gap-1 hover:text-foreground"
                @click="header.column.getToggleSortingHandler()?.($event)"
              >
                <component :is="getSortIcon(header.column.id)" class="h-3 w-3" />
                {{ header.isPlaceholder ? "" : header.column.columnDef.header as string }}
              </button>
              <span v-else>
                {{ header.isPlaceholder ? "" : header.column.columnDef.header as string }}
              </span>
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="table.getRowModel().rows.length > 0">
            <template v-for="row in table.getRowModel().rows" :key="row.id">
              <TableRow
                v-if="row.original.type === 'transaction'"
                :class="{ 'bg-muted/30': row.getIsExpanded() }"
              >
                <TableCell
                  v-for="cell in row.getVisibleCells()"
                  :key="cell.id"
                  :class="{ 'cursor-pointer': cell.column.id === 'date' }"
                >
                  <div
                    v-if="cell.column.id === 'date'"
                    class="flex items-center gap-1"
                    @click="toggleRow(row.id)"
                  >
                    <component
                      :is="row.getIsExpanded() ? ChevronDown : ChevronRight"
                      class="h-4 w-4 text-muted-foreground"
                    />
                    {{ cell.getValue() as string }}
                  </div>
                  <span v-else>
                    {{ formatCellValue(cell, cell.column.id) }}
                  </span>
                </TableCell>
              </TableRow>
              <TableRow
                v-if="row.original.type === 'split' && row.getParentRow()?.getIsExpanded()"
                class="bg-muted/10 text-xs"
              >
                <TableCell
                  v-for="cell in row.getVisibleCells()"
                  :key="cell.id"
                >
                  <span class="text-muted-foreground">
                    {{ formatCellValue(cell, cell.column.id) }}
                  </span>
                </TableCell>
              </TableRow>
            </template>
          </template>
          <tr v-else>
            <td :colspan="columns.length" class="h-24 text-center text-muted-foreground">
              No transactions found.
            </td>
          </tr>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
