<script setup lang="ts">
import { ref, computed, onMounted } from "vue"
import { useRouter } from "vue-router"
import { Card, CardContent, CardHeader, CardTitle, Button, Input, Label } from "@/components/ui"
import { useAccountStore } from "@/stores/accountStore"
import { useTransactionStore } from "@/stores/transactionStore"
import { Plus, Trash2, AlertCircle } from "@lucide/vue"

const router = useRouter()
const accountStore = useAccountStore()
const txnStore = useTransactionStore()

const date = ref(new Date().toISOString().split("T")[0])
const payee = ref("")
const description = ref("")
const number = ref("")

interface SplitRow { id: number; accountId: number | null; debitAmount: string; creditAmount: string; memo: string }
let splitCounter = 0
const splits = ref<SplitRow[]>([
  { id: ++splitCounter, accountId: null, debitAmount: "", creditAmount: "", memo: "" },
  { id: ++splitCounter, accountId: null, debitAmount: "", creditAmount: "", memo: "" },
])

const saving = ref(false)
const error = ref("")

onMounted(() => accountStore.fetchAccounts())

const leafAccounts = computed(() => accountStore.activeAccounts.filter((a) => !a.placeholder))

const totalDebits = computed(() => splits.value.reduce((s, r) => s + (parseFloat(r.debitAmount) || 0), 0))
const totalCredits = computed(() => splits.value.reduce((s, r) => s + (parseFloat(r.creditAmount) || 0), 0))
const isBalanced = computed(() =>
  Math.abs(totalDebits.value - totalCredits.value) < 0.001 &&
  splits.value.length >= 2 &&
  splits.value.every((r) => r.accountId !== null),
)

function addSplit() { splits.value.push({ id: ++splitCounter, accountId: null, debitAmount: "", creditAmount: "", memo: "" }) }
function removeSplit(index: number) { if (splits.value.length > 2) splits.value.splice(index, 1) }

function autoBalance() {
  const diff = totalDebits.value - totalCredits.value
  if (Math.abs(diff) < 0.001) return
  const last = splits.value[splits.value.length - 1]
  if (diff > 0) last.creditAmount = diff.toFixed(2)
  else last.debitAmount = (-diff).toFixed(2)
}

async function save() {
  if (!isBalanced.value) { error.value = "Transaction must be balanced (debits = credits)"; return }
  saving.value = true; error.value = ""
  try {
    await txnStore.postNewTransaction({
      date: date.value,
      payee: payee.value || null,
      description: description.value || null,
      number: number.value || null,
      splits: splits.value.map((s) => ({
        accountId: s.accountId!,
        debitAmount: s.debitAmount || "0",
        creditAmount: s.creditAmount || "0",
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
  <div class="max-w-2xl mx-auto">
    <Card>
      <CardHeader>
        <CardTitle>New Transaction</CardTitle>
        <p class="text-sm text-muted-foreground">Record a new double-entry transaction</p>
      </CardHeader>
      <CardContent class="space-y-4">
        <div v-if="error" class="rounded-md bg-destructive/10 p-3 text-sm text-destructive flex items-start gap-2">
          <AlertCircle class="h-4 w-4 mt-0.5 shrink-0" />{{ error }}
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1"><Label>Date</Label><Input v-model="date" type="date" /></div>
          <div class="space-y-1"><Label>Number</Label><Input v-model="number" placeholder="Check #" /></div>
          <div class="space-y-1 col-span-2"><Label>Payee</Label><Input v-model="payee" placeholder="Payee or counterparty" /></div>
          <div class="space-y-1 col-span-2"><Label>Description</Label><Input v-model="description" placeholder="Transaction memo" /></div>
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
                  <td class="px-3 py-1.5">
                    <select v-model="split.accountId" class="w-full rounded border border-input bg-background px-2 py-1 text-xs ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
                      <option :value="null">Select account...</option>
                      <option v-for="acc in leafAccounts" :key="acc.id" :value="acc.id">{{ acc.name }}</option>
                    </select>
                  </td>
                  <td class="px-3 py-1.5">
                    <input v-model="split.debitAmount" type="number" step="0.01" min="0" placeholder="0.00" class="w-full rounded border border-input bg-background px-2 py-1 text-xs text-right font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
                  </td>
                  <td class="px-3 py-1.5">
                    <input v-model="split.creditAmount" type="number" step="0.01" min="0" placeholder="0.00" class="w-full rounded border border-input bg-background px-2 py-1 text-xs text-right font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
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
                    <span v-if="!isBalanced" class="ml-2 text-destructive font-semibold">(Off by {{ Math.abs(totalDebits - totalCredits).toFixed(2) }})</span>
                    <span v-else class="ml-2 text-emerald-500 font-semibold">Balanced ✓</span>
                  </td>
                  <td class="px-3 py-1.5 text-right text-xs font-mono">{{ totalDebits.toFixed(2) }}</td>
                  <td class="px-3 py-1.5 text-right text-xs font-mono">{{ totalCredits.toFixed(2) }}</td>
                  <td></td><td></td>
                </tr>
              </tfoot>
            </table>
          </div>
          <div class="flex justify-end mt-2">
            <Button variant="ghost" size="sm" @click="autoBalance" :disabled="isBalanced">Auto-Balance</Button>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2 border-t">
          <Button variant="outline" @click="router.back()">Cancel</Button>
          <Button :disabled="saving || !isBalanced" @click="save">{{ saving ? "Posting..." : "Post Transaction" }}</Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
