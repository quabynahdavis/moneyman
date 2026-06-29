<script setup lang="ts">
import { ref, computed, onMounted } from "vue"
import { useRouter } from "vue-router"
import { Button, Input, Label } from "@/components/ui"
import { useAccountStore } from "@/stores/accountStore"
import { useTransactionStore } from "@/stores/transactionStore"
import { Plus, Trash2, AlertCircle } from "@lucide/vue"
import { toCents } from "@/utils/decimal"
import AccountPicker from "@/components/AccountPicker.vue"
import DatePicker from "@/components/DatePicker.vue"

const router = useRouter()
const accountStore = useAccountStore()
const txnStore = useTransactionStore()

const postDateStr = ref(new Date().toISOString().split("T")[0])
const description = ref("")
const num = ref("")

interface SplitRow { id: number; accountId: string; debitCents: number; creditCents: number; memo: string; debitStr: string; creditStr: string }
let splitCounter = 0
const splits = ref<SplitRow[]>([
  { id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "", debitStr: "", creditStr: "" },
  { id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "", debitStr: "", creditStr: "" },
])

const saving = ref(false)
const error = ref("")

onMounted(() => accountStore.fetchAccounts())

const totalDebits = computed(() => splits.value.reduce((s, r) => s + r.debitCents, 0))
const totalCredits = computed(() => splits.value.reduce((s, r) => s + r.creditCents, 0))
const isBalanced = computed(() =>
  totalDebits.value === totalCredits.value &&
  splits.value.length >= 2 &&
  splits.value.every((r) => r.accountId !== ""),
)

const diffCents = computed(() => Math.abs(totalDebits.value - totalCredits.value))

function addSplit() { splits.value.push({ id: ++splitCounter, accountId: "", debitCents: 0, creditCents: 0, memo: "", debitStr: "", creditStr: "" }) }
function removeSplit(index: number) { if (splits.value.length > 2) splits.value.splice(index, 1) }

function parseCents(value: string): number {
  const cleaned = value.replace(/[^0-9.]/g, "")
  const num = parseFloat(cleaned)
  if (isNaN(num)) return 0
  return toCents(num)
}

function onDebitFocus(idx: number) {
  const s = splits.value[idx]
  if (s.debitCents) s.debitStr = (s.debitCents / 100).toFixed(2)
}
function onDebitBlur(idx: number) {
  const s = splits.value[idx]
  s.debitCents = parseCents(s.debitStr)
  s.creditCents = 0
  s.creditStr = ""
  if (s.debitCents) s.debitStr = (s.debitCents / 100).toFixed(2)
}

function onCreditFocus(idx: number) {
  const s = splits.value[idx]
  if (s.creditCents) s.creditStr = (s.creditCents / 100).toFixed(2)
}
function onCreditBlur(idx: number) {
  const s = splits.value[idx]
  s.creditCents = parseCents(s.creditStr)
  s.debitCents = 0
  s.debitStr = ""
  if (s.creditCents) s.creditStr = (s.creditCents / 100).toFixed(2)
}

function clearAmounts(idx: number) {
  const s = splits.value[idx]
  s.debitCents = 0; s.debitStr = ""
  s.creditCents = 0; s.creditStr = ""
}

function autoBalance() {
  const diff = totalDebits.value - totalCredits.value
  if (diff === 0) return
  const last = splits.value[splits.value.length - 1]
  if (diff > 0) { last.creditCents = diff; last.debitCents = 0; last.creditStr = (diff / 100).toFixed(2); last.debitStr = "" }
  else { last.debitCents = -diff; last.creditCents = 0; last.debitStr = ((-diff) / 100).toFixed(2); last.creditStr = "" }
}

async function save() {
  if (!isBalanced.value) { error.value = "Transaction must be balanced (debits = credits)"; return }
  saving.value = true; error.value = ""
  try {
    await txnStore.postNewTransaction({
      postDate: postDateStr.value,
      description: description.value || null,
      num: num.value || null,
      splits: splits.value.map((s) => ({
        accountId: parseInt(s.accountId),
        debit: s.debitCents,
        credit: s.creditCents,
        memo: s.memo || null,
      })),
    })
    router.push({ name: "ledger" })
  } catch (e: any) {
    error.value = typeof e === "string" ? e : "Failed to post transaction"
  } finally { saving.value = false }
}
</script>

<template>
  <div class="space-y-6">
    <div v-if="error" class="rounded-md bg-destructive/10 p-3 text-sm text-destructive flex items-start gap-2">
      <AlertCircle class="h-4 w-4 mt-0.5 shrink-0" />{{ error }}
    </div>

    <div class="grid grid-cols-2 gap-3">
      <div class="space-y-1">
        <Label>Date</Label>
        <DatePicker v-model="postDateStr" />
      </div>
      <div class="space-y-1"><Label>Num</Label><Input v-model="num" placeholder="Check #" /></div>
      <div class="space-y-1 col-span-2"><Label>Description</Label><Input v-model="description" placeholder="Payee or description" /></div>
    </div>

    <div>
      <div class="flex items-center justify-between mb-2">
        <Label>Splits</Label>
        <Button variant="outline" size="sm" @click="addSplit"><Plus class="h-3 w-3 mr-1" /> Add Split</Button>
      </div>
      <div class="rounded-md border overflow-hidden">
        <table class="w-full text-sm">
          <thead class="bg-muted/50">
            <tr>
              <th class="px-3 py-2 text-left font-medium text-muted-foreground">Account</th>
              <th class="px-3 py-2 text-right font-medium text-muted-foreground">Debit</th>
              <th class="px-3 py-2 text-right font-medium text-muted-foreground">Credit</th>
              <th class="px-3 py-2 text-left font-medium text-muted-foreground">Memo</th>
              <th class="px-3 py-2 w-8"></th>
            </tr>
          </thead>
          <tbody class="divide-y">
            <tr v-for="(split, idx) in splits" :key="split.id">
              <td class="px-3 py-1.5 min-w-[200px]">
                <AccountPicker
                  :model-value="split.accountId"
                  placeholder="Select account..."
                  @update:model-value="splits[idx].accountId = $event"
                />
              </td>
              <td class="px-3 py-1.5">
                <input
                  v-model="split.debitStr"
                  type="text"
                  inputmode="decimal"
                  placeholder="0.00"
                  class="w-full rounded border border-input bg-background px-2 py-1 text-xs text-right font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  @focus="onDebitFocus(idx)"
                  @blur="onDebitBlur(idx)"
                  @input="split.creditStr = ''"
                />
              </td>
              <td class="px-3 py-1.5">
                <input
                  v-model="split.creditStr"
                  type="text"
                  inputmode="decimal"
                  placeholder="0.00"
                  class="w-full rounded border border-input bg-background px-2 py-1 text-xs text-right font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  @focus="onCreditFocus(idx)"
                  @blur="onCreditBlur(idx)"
                  @input="split.debitStr = ''"
                />
              </td>
              <td class="px-3 py-1.5">
                <input v-model="split.memo" placeholder="Memo" class="w-full rounded border border-input bg-background px-2 py-1 text-xs focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
              </td>
              <td class="px-3 py-1.5 text-center">
                <button v-if="splits.length > 2" class="text-muted-foreground hover:text-destructive" @click="removeSplit(idx)"><Trash2 class="h-3 w-3" /></button>
              </td>
            </tr>
          </tbody>
          <tfoot class="bg-muted/30 border-t">
            <tr>
              <td class="px-3 py-1.5 text-xs font-medium text-muted-foreground">
                Totals
                <span v-if="!isBalanced" class="ml-2 text-destructive font-semibold">(Off by {{ (diffCents / 100).toFixed(2) }})</span>
                <span v-else class="ml-2 text-emerald-500 font-semibold">Balanced ✓</span>
              </td>
              <td class="px-3 py-1.5 text-right text-xs font-mono">{{ (totalDebits / 100).toFixed(2) }}</td>
              <td class="px-3 py-1.5 text-right text-xs font-mono">{{ (totalCredits / 100).toFixed(2) }}</td>
              <td></td><td></td>
            </tr>
          </tfoot>
        </table>
      </div>
      <div class="flex justify-end mt-2">
        <Button variant="ghost" size="sm" @click="autoBalance" :disabled="isBalanced">Auto-Balance</Button>
      </div>
    </div>

    <div class="flex justify-end gap-2 pt-4 border-t">
      <Button variant="outline" @click="router.back()">Cancel</Button>
      <Button :disabled="saving || !isBalanced" @click="save">{{ saving ? "Posting..." : "Post Transaction" }}</Button>
    </div>
  </div>
</template>
