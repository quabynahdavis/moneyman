<script setup lang="ts">
import { ref, computed, h } from "vue"
import {
  createColumnHelper,
  getCoreRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  getExpandedRowModel,
  useVueTable,
  type SortingState,
  type ExpandedState,
} from "@tanstack/vue-table"
import Table from "@/components/ui/Table.vue"
import TableBody from "@/components/ui/TableBody.vue"
import TableCell from "@/components/ui/TableCell.vue"
import TableHead from "@/components/ui/TableHead.vue"
import TableHeader from "@/components/ui/TableHeader.vue"
import TableRow from "@/components/ui/TableRow.vue"
import { Badge } from "@/components/ui/Badge.vue"
import { ChevronDown, ChevronRight, ArrowUpDown, ArrowUp, ArrowDown } from "@lucide/vue"
import type { Transaction } from "@/types"
import { formatMoney } from "@/utils/decimal"

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

const columnHelper = createColumnHelper<LedgerRow>()

const columns = computed(() => {
  const cols = [
    columnHelper.accessor("date", {
      header: "Date",
      cell: (info) => info.getValue(),
      enableSorting: true,
    }),
    columnHelper.accessor("number", {
      header: "Num",
      cell: (info) => info.getValue() || "",
      enableSorting: true,
    }),
    columnHelper.accessor("payee", {
      header: "Payee",
      cell: (info) => info.getValue() || "",
      enableSorting: true,
    }),
    columnHelper.accessor("description", {
      header: "Description",
      cell: (info) => info.getValue() || "",
    }),
  ]

  if (props.showAccountColumn) {
    cols.push(
      columnHelper.accessor("accountName", {
        header: "Account",
        cell: (info) => info.getValue() || "",
      }),
    )
  }

  cols.push(
    columnHelper.accessor("memo", {
      header: "Memo",
      cell: (info) => info.getValue() || "",
    }),
    columnHelper.accessor("debit", {
      header: "Debit",
      cell: (info) => {
        const val = info.getValue()
        return val && val !== "0" ? formatMoney(val) : ""
      },
      enableSorting: true,
    }),
    columnHelper.accessor("credit", {
      header: "Credit",
      cell: (info) => {
        const val = info.getValue()
        return val && val !== "0" ? formatMoney(val) : ""
      },
      enableSorting: true,
    }),
    columnHelper.accessor("state", {
      header: "State",
      cell: (info) => {
        const state = info.getValue()
        const variant = state === "RECONCILED" ? "success"
          : state === "CLEARED" ? "warning"
          : state === "VOID" ? "destructive"
          : "secondary"
        return h(Badge, { variant }, () => state)
      },
      enableSorting: true,
    }),
  )

  return cols
})

const ledgerRows = computed<LedgerRow[]>(() => {
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
      debit: txn.splits.reduce((sum, s) => (sum.plus(s.debitAmount || "0")), new Decimal(0)).toString(),
      credit: txn.splits.reduce((sum, s) => (sum.plus(s.creditAmount || "0")), new Decimal(0)).toString(),
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

const table = useVueTable({
  get data() { return ledgerRows.value },
  columns: columns.value,
  state: {
    get sorting() { return sorting.value },
    set sorting(updater) { sorting.value = typeof updater === "function" ? updater(sorting.value) : updater },
    get expanded() { return expanded.value },
    set expanded(updater) { expanded.value = typeof updater === "function" ? updater(expanded.value) : updater },
    get globalFilter() { return globalFilter.value },
    set globalFilter(updater) { globalFilter.value = typeof updater === "function" ? updater(globalFilter.value) : updater },
    get rowSelection() { return rowSelection.value },
    set rowSelection(updater) { rowSelection.value = typeof updater === "function" ? updater(rowSelection.value) : updater },
  },
  getCoreRowModel: getCoreRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),
  getSubRows: (row) => row.type === "transaction" ? ledgerRows.value.filter((r) => r.type === "split" && r.txnId === row.txnId) : [],
  enableExpanding: true,
  enableSorting: true,
  enableMultiSort: false,
  enableRowSelection: false,
})

function toggleRow(rowId: string) {
  const current = { ...expanded.value }
  current[rowId] = !current[rowId]
  expanded.value = current
}

function getSortIcon(columnId: string) {
  const sort = sorting.value.find((s) => s.id === columnId)
  if (!sort) return ArrowUpDown
  return sort.desc ? ArrowDown : ArrowUp
}

import Decimal from "decimal.js"
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
                {{ header.isPlaceholder ? "" : (header.column.columnDef.header as string) }}
              </button>
              <span v-else>
                {{ header.isPlaceholder ? "" : (header.column.columnDef.header as string) }}
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
                    {{ cell.getValue() as string }}
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
                    {{ cell.getValue() as string }}
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
