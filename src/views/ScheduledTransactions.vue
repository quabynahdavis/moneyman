<script setup lang="ts">
import { ref, computed, onMounted } from "vue"
import { toast } from "vue-sonner"
import { useRecurringStore } from "@/stores/recurringStore"
import { useAccountStore } from "@/stores/accountStore"
import { useConfirm } from "@/composables/useConfirm"
import {
  Button, Card, CardContent, CardHeader, CardTitle, CardDescription,
  Input, Label,
} from "@/components/ui"
import AccountPicker from "@/components/AccountPicker.vue"
import {
  CalendarClock, Plus, Play, Pencil, Trash2, ToggleLeft, ToggleRight,
  AlertCircle, Check,
} from "@lucide/vue"
import type { RecurringFrequency } from "@/types"

const store = useRecurringStore()
const accountStore = useAccountStore()
const { confirm } = useConfirm()

// ── Form state ──

const showForm = ref(false)
const editingId = ref<number | null>(null)

const formDescription = ref("")
const formFrequency = ref<RecurringFrequency>("MONTHLY")
const formIntervalCount = ref("1")
const formNextDate = ref(new Date().toISOString().split("T")[0])
const formEndDate = ref("")
const formAutoExecute = ref(false)
const formNum = ref("")

interface FormSplit { id: number; accountId: string; debitCents: number; creditCents: number; memo: string }
let splitCounter = 0
const formSplits = ref<FormSplit[]>([
  { id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "" },
  { id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "" },
])

const formError = ref("")
const formSaving = ref(false)

const formTotalDebits = computed(() => formSplits.value.reduce((s, r) => s + r.debitCents, 0))
const formTotalCredits = computed(() => formSplits.value.reduce((s, r) => s + r.creditCents, 0))
const formBalanced = computed(() =>
  formTotalDebits.value === formTotalCredits.value &&
  formTotalCredits.value > 0 &&
  formSplits.value.length >= 2 &&
  formSplits.value.every((r) => r.accountId !== ""),
)

const frequencies: { value: RecurringFrequency; label: string }[] = [
  { value: "DAILY", label: "Daily" },
  { value: "WEEKLY", label: "Weekly" },
  { value: "BIWEEKLY", label: "Bi-Weekly" },
  { value: "MONTHLY", label: "Monthly" },
  { value: "QUARTERLY", label: "Quarterly" },
  { value: "SEMI_ANNUAL", label: "Semi-Annual" },
  { value: "ANNUAL", label: "Annual" },
]

function formatCents(c: number) { return (c / 100).toFixed(2) }

function parseCents(v: string): number {
  const cleaned = v.replace(/[^0-9.]/g, "")
  const n = parseFloat(cleaned)
  return isNaN(n) ? 0 : Math.round(n * 100)
}

function addSplit() {
  formSplits.value.push({ id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "" })
}
function removeSplit(idx: number) {
  if (formSplits.value.length > 2) formSplits.value.splice(idx, 1)
}

function onDebitInput(idx: number, value: string) {
  formSplits.value[idx].debitCents = parseCents(value)
  formSplits.value[idx].creditCents = 0
}
function onCreditInput(idx: number, value: string) {
  formSplits.value[idx].creditCents = parseCents(value)
  formSplits.value[idx].debitCents = 0
}

function openCreate() {
  editingId.value = null
  resetForm()
  showForm.value = true
}

function openEdit(txn: NonNullable<ReturnType<typeof store.getById>>) {
  editingId.value = txn.id!
  formDescription.value = txn.description
  formFrequency.value = txn.frequency
  formIntervalCount.value = String(txn.intervalCount)
  formNextDate.value = txn.nextDate
  formEndDate.value = txn.endDate ?? ""
  formAutoExecute.value = txn.autoExecute
  formNum.value = txn.num ?? ""
  formSplits.value = txn.splits.map((s) => ({
    id: ++splitCounter,
    accountId: String(s.accountId),
    debitCents: s.debit,
    creditCents: s.credit,
    memo: s.memo ?? "",
  }))
  showForm.value = true
  formError.value = ""
}

function resetForm() {
  formDescription.value = ""
  formFrequency.value = "MONTHLY"
  formIntervalCount.value = "1"
  formNextDate.value = new Date().toISOString().split("T")[0]
  formEndDate.value = ""
  formAutoExecute.value = false
  formNum.value = ""
  formSplits.value = [
    { id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "" },
    { id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "" },
  ]
  formError.value = ""
}

async function saveForm() {
  if (!formBalanced.value) {
    formError.value = "Splits must be balanced (debits = credits)"
    return
  }
  formSaving.value = true
  formError.value = ""
  try {
    const splits = formSplits.value.map((s) => ({
      accountId: parseInt(s.accountId),
      debit: s.debitCents,
      credit: s.creditCents,
      memo: s.memo || null,
    }))

    if (editingId.value) {
      await store.update({
        id: editingId.value,
        frequency: formFrequency.value,
        intervalCount: parseInt(formIntervalCount.value) || 1,
        nextDate: formNextDate.value,
        endDate: formEndDate.value || null,
        autoExecute: formAutoExecute.value,
        description: formDescription.value,
        num: formNum.value || null,
        splits,
      })
      toast.success("Schedule updated")
    } else {
      await store.create({
        frequency: formFrequency.value,
        intervalCount: parseInt(formIntervalCount.value) || 1,
        nextDate: formNextDate.value,
        endDate: formEndDate.value || null,
        autoExecute: formAutoExecute.value,
        description: formDescription.value,
        num: formNum.value || null,
        splits,
      })
      toast.success("Schedule created")
    }
    showForm.value = false
  } catch (e: any) {
    formError.value = typeof e === "string" ? e : "Failed to save schedule"
  } finally {
    formSaving.value = false
  }
}

async function handleExecute(id: number) {
  const ok = await confirm({
    title: "Execute Now",
    message: "Generate a transaction from this schedule and advance the next date?",
    confirmLabel: "Execute",
  })
  if (!ok) return
  try {
    await store.execute(id)
    toast.success("Transaction created from schedule")
  } catch (e: any) {
    toast.error(typeof e === "string" ? e : "Failed to execute")
  }
}

async function handleDelete(id: number) {
  const ok = await confirm({
    title: "Delete Schedule",
    message: "This will permanently remove this recurring schedule.",
    confirmLabel: "Delete",
    variant: "destructive",
  })
  if (!ok) return
  try {
    await store.remove(id)
    toast.success("Schedule deleted")
  } catch (e: any) {
    toast.error(typeof e === "string" ? e : "Failed to delete")
  }
}

async function toggleActive(txn: NonNullable<ReturnType<typeof store.getById>>) {
  try {
    await store.update({ id: txn.id!, isActive: !txn.isActive })
    toast.success(txn.isActive ? "Schedule paused" : "Schedule activated")
  } catch (e: any) {
    toast.error(typeof e === "string" ? e : "Failed to toggle")
  }
}

const frequencyLabel: Record<string, string> = {
  DAILY: "Daily", WEEKLY: "Weekly", BIWEEKLY: "Bi-Weekly",
  MONTHLY: "Monthly", QUARTERLY: "Quarterly", SEMI_ANNUAL: "Semi-Annual", ANNUAL: "Annual",
}

onMounted(() => {
  store.fetchAll()
  accountStore.fetchAccounts()
})
</script>

<template>
  <div class="space-y-4">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Scheduled Transactions</h2>
        <p class="text-sm text-muted-foreground">Recurring and upcoming transactions</p>
      </div>
      <Button @click="openCreate">
        <Plus class="h-4 w-4 mr-2" /> New Schedule
      </Button>
    </div>

    <!-- Error message -->
    <div
      v-if="store.error"
      class="rounded-md bg-destructive/10 p-3 text-sm text-destructive flex items-start gap-2"
    >
      <AlertCircle class="h-4 w-4 mt-0.5 shrink-0" />{{ store.error }}
    </div>

    <!-- Create/Edit form modal -->
    <Card v-if="showForm">
      <CardHeader>
        <CardTitle>{{ editingId ? "Edit" : "New" }} Schedule</CardTitle>
        <CardDescription>Define the recurring transaction template</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div v-if="formError" class="rounded-md bg-destructive/10 p-2 text-xs text-destructive">{{ formError }}</div>

        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1 col-span-2">
            <Label>Description</Label>
            <Input v-model="formDescription" placeholder="e.g. Monthly Rent" />
          </div>
          <div class="space-y-1">
            <Label>Frequency</Label>
            <select
              v-model="formFrequency"
              class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option v-for="f in frequencies" :key="f.value" :value="f.value">{{ f.label }}</option>
            </select>
          </div>
          <div class="space-y-1">
            <Label>Every</Label>
            <Input v-model="formIntervalCount" type="number" min="1" />
          </div>
          <div class="space-y-1">
            <Label>Next Date</Label>
            <Input v-model="formNextDate" type="date" />
          </div>
          <div class="space-y-1">
            <Label>End Date (optional)</Label>
            <Input v-model="formEndDate" type="date" />
          </div>
          <div class="space-y-1">
            <Label>Num / Check #</Label>
            <Input v-model="formNum" placeholder="Optional" />
          </div>
          <div class="flex items-center gap-2 col-span-2">
            <Label class="mb-0">Auto Execute</Label>
            <input v-model="formAutoExecute" type="checkbox" class="h-4 w-4" />
          </div>
        </div>

        <!-- Splits section -->
        <div>
          <div class="flex items-center justify-between mb-2">
            <Label>Splits</Label>
            <Button variant="outline" size="sm" @click="addSplit"><Plus class="h-3 w-3 mr-1" /> Add</Button>
          </div>
          <div class="rounded-md border overflow-hidden text-sm">
            <table class="w-full">
              <thead class="bg-muted/50">
                <tr>
                  <th class="px-2 py-1 text-left font-medium text-muted-foreground text-xs">Account</th>
                  <th class="px-2 py-1 text-right font-medium text-muted-foreground text-xs">Debit</th>
                  <th class="px-2 py-1 text-right font-medium text-muted-foreground text-xs">Credit</th>
                  <th class="px-2 py-1 text-left font-medium text-muted-foreground text-xs">Memo</th>
                  <th class="w-8"></th>
                </tr>
              </thead>
              <tbody class="divide-y">
                <tr v-for="(split, idx) in formSplits" :key="split.id">
                  <td class="px-2 py-1 min-w-[160px]">
                    <AccountPicker
                      :model-value="split.accountId"
                      placeholder="Select account..."
                      @update:model-value="formSplits[idx].accountId = $event"
                    />
                  </td>
                  <td class="px-2 py-1">
                    <input
                      :value="split.debitCents ? formatCents(split.debitCents) : ''"
                      type="text" inputmode="decimal" placeholder="0.00"
                      class="w-full rounded border border-input bg-background px-2 py-1 text-xs text-right font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                      @input="onDebitInput(idx, ($event.target as HTMLInputElement).value)"
                    />
                  </td>
                  <td class="px-2 py-1">
                    <input
                      :value="split.creditCents ? formatCents(split.creditCents) : ''"
                      type="text" inputmode="decimal" placeholder="0.00"
                      class="w-full rounded border border-input bg-background px-2 py-1 text-xs text-right font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                      @input="onCreditInput(idx, ($event.target as HTMLInputElement).value)"
                    />
                  </td>
                  <td class="px-2 py-1">
                    <input v-model="split.memo" placeholder="Memo" class="w-full rounded border border-input bg-background px-2 py-1 text-xs focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
                  </td>
                  <td class="px-2 py-1 text-center">
                    <button v-if="formSplits.length > 2" class="text-muted-foreground hover:text-destructive" @click="removeSplit(idx)">
                      <Trash2 class="h-3 w-3" />
                    </button>
                  </td>
                </tr>
              </tbody>
              <tfoot class="bg-muted/30 border-t">
                <tr>
                  <td class="px-2 py-1 text-xs text-muted-foreground">
                    Totals
                    <span v-if="!formBalanced" class="ml-2 text-destructive font-semibold">(Off by {{ formatCents(Math.abs(formTotalDebits - formTotalCredits)) }})</span>
                    <span v-else class="ml-2 text-emerald-500 font-semibold">Balanced ✓</span>
                  </td>
                  <td class="px-2 py-1 text-right text-xs font-mono">{{ formatCents(formTotalDebits) }}</td>
                  <td class="px-2 py-1 text-right text-xs font-mono">{{ formatCents(formTotalCredits) }}</td>
                  <td></td><td></td>
                </tr>
              </tfoot>
            </table>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2 border-t">
          <Button variant="outline" @click="showForm = false">Cancel</Button>
          <Button @click="saveForm" :disabled="formSaving || !formBalanced">
            {{ formSaving ? "Saving..." : editingId ? "Update Schedule" : "Create Schedule" }}
          </Button>
        </div>
      </CardContent>
    </Card>

    <!-- Loading -->
    <div v-if="store.loading" class="text-center text-sm text-muted-foreground py-8">Loading...</div>

    <!-- Upcoming section (overdue + due soon) -->
    <Card v-if="!store.loading">
      <CardHeader>
        <CardTitle>Upcoming</CardTitle>
        <CardDescription>Active schedules due now or soon</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div v-if="store.upcoming.length === 0" class="px-6 py-8 text-center text-sm text-muted-foreground">
          No upcoming schedules. Create one to automate recurring bills or income.
        </div>
        <div v-else class="divide-y">
          <div
            v-for="txn in store.upcoming"
            :key="txn.id"
            class="flex items-center gap-4 px-6 py-4"
          >
            <CalendarClock class="h-5 w-5 text-amber-500 shrink-0" />
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium truncate">{{ txn.description || "(no description)" }}</p>
              <p class="text-xs text-muted-foreground">
                {{ frequencyLabel[txn.frequency] || txn.frequency }}
                {{ txn.intervalCount > 1 ? `× ${txn.intervalCount}` : "" }}
                — next: {{ txn.nextDate }}
              </p>
            </div>
            <div class="text-sm font-mono tabular-nums shrink-0">
              {{ formatCents(txn.splits.reduce((s, sp) => s + sp.debit, 0)) }}
            </div>
            <div class="flex gap-1 shrink-0">
              <Button variant="ghost" size="icon" class="h-8 w-8" title="Execute now" @click="handleExecute(txn.id!)">
                <Play class="h-3.5 w-3.5" />
              </Button>
              <Button variant="ghost" size="icon" class="h-8 w-8" title="Edit" @click="openEdit(txn)">
                <Pencil class="h-3.5 w-3.5" />
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Active schedules -->
    <Card v-if="!store.loading && store.activeTransactions.length > 0">
      <CardHeader>
        <CardTitle>Active Schedules</CardTitle>
        <CardDescription>{{ store.activeTransactions.length }} active recurring rules</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="divide-y">
          <div
            v-for="txn in store.activeTransactions"
            :key="txn.id"
            class="flex items-center gap-4 px-6 py-4"
          >
            <CalendarClock class="h-5 w-5 text-muted-foreground shrink-0" />
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium truncate">{{ txn.description || "(no description)" }}</p>
              <p class="text-xs text-muted-foreground">
                {{ frequencyLabel[txn.frequency] || txn.frequency }}
                — next: {{ txn.nextDate }}
                <span v-if="txn.endDate"> · until {{ txn.endDate }}</span>
                <span v-if="txn.autoExecute" class="ml-1 text-emerald-500">· Auto</span>
              </p>
            </div>
            <div class="text-sm font-mono tabular-nums shrink-0">
              {{ formatCents(txn.splits.reduce((s, sp) => s + sp.debit, 0)) }}
            </div>
            <div class="flex gap-1 shrink-0">
              <Button variant="ghost" size="icon" class="h-8 w-8" :title="txn.autoExecute ? 'Auto-execute on' : 'Auto-execute off'">
                <component :is="txn.autoExecute ? ToggleRight : ToggleLeft" class="h-3.5 w-3.5" />
              </Button>
              <Button variant="ghost" size="icon" class="h-8 w-8" title="Execute now" @click="handleExecute(txn.id!)">
                <Play class="h-3.5 w-3.5" />
              </Button>
              <Button variant="ghost" size="icon" class="h-8 w-8" title="Edit" @click="openEdit(txn)">
                <Pencil class="h-3.5 w-3.5" />
              </Button>
              <Button variant="ghost" size="icon" class="h-8 w-8" title="Toggle active" @click="toggleActive(txn)">
                <component :is="txn.isActive ? ToggleRight : ToggleLeft" class="h-3.5 w-3.5" />
              </Button>
              <Button variant="ghost" size="icon" class="h-8 w-8 text-destructive" title="Delete" @click="handleDelete(txn.id!)">
                <Trash2 class="h-3.5 w-3.5" />
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Past schedules -->
    <Card v-if="!store.loading && store.pastTransactions.length > 0">
      <CardHeader>
        <CardTitle>Completed / Inactive</CardTitle>
        <CardDescription>{{ store.pastTransactions.length }} schedules that have ended or been paused</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="divide-y">
          <div
            v-for="txn in store.pastTransactions"
            :key="txn.id"
            class="flex items-center gap-4 px-6 py-4 opacity-60"
          >
            <CalendarClock class="h-5 w-5 text-muted-foreground shrink-0" />
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium truncate">{{ txn.description || "(no description)" }}</p>
              <p class="text-xs text-muted-foreground">
                {{ frequencyLabel[txn.frequency] || txn.frequency }}
                — last: {{ txn.lastGenerated || txn.nextDate }}
              </p>
            </div>
            <div class="flex gap-1 shrink-0">
              <Button variant="ghost" size="icon" class="h-8 w-8" title="Reactivate" @click="toggleActive(txn)">
                <ToggleRight class="h-3.5 w-3.5" />
              </Button>
              <Button variant="ghost" size="icon" class="h-8 w-8 text-destructive" title="Delete" @click="handleDelete(txn.id!)">
                <Trash2 class="h-3.5 w-3.5" />
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
