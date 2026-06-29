<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue"
import { useRoute, useRouter } from "vue-router"
import {
  Card, CardContent, CardHeader, CardTitle, CardDescription,
  Button, Input, Label,
} from "@/components/ui"
import {
  SearchIcon, Check, X, ChevronLeft, ChevronRight, Wallet,
  AlertTriangle, Circle, CheckCircle2, RotateCw,
} from "@lucide/vue"
import { useReconciliationStore } from "@/stores/reconciliationStore"
import { useAccountStore } from "@/stores/accountStore"
import { cn } from "@/lib/utils"
import AccountPicker from "@/components/AccountPicker.vue"
import DatePicker from "@/components/DatePicker.vue"

const route = useRoute()
const router = useRouter()
const reconStore = useReconciliationStore()
const accountStore = useAccountStore()

const accountIdParam = computed(() => {
  const v = route.params.accountId
  return v ? Number(v) : null
})

// Start session form
const selectedAccountId = ref<string>("")
const statementDate = ref(new Date().toISOString().split("T")[0])
const endingBalanceStr = ref("0.00")
const startingBalanceStr = ref("0.00")
const showStartForm = ref(true)
const searchQuery = ref("")

const filteredSplits = computed(() => {
  const q = searchQuery.value.toLowerCase().trim()
  if (!q) return reconStore.unreconciledSplits
  return reconStore.unreconciledSplits.filter(
    (s) =>
      s.description.toLowerCase().includes(q) ||
      (s.num && s.num.toLowerCase().includes(q)) ||
      (s.memo && s.memo.toLowerCase().includes(q)),
  )
})

const formatCents = (cents: number) => (cents / 100).toFixed(2)

const balanceColor = computed(() => {
  if (reconStore.isBalanced) return "text-emerald-500"
  return reconStore.difference > 0 ? "text-amber-500" : "text-red-500"
})

onMounted(async () => {
  await accountStore.fetchAccounts()
  if (accountIdParam.value) {
    selectedAccountId.value = String(accountIdParam.value)
    showStartForm.value = false
  }
})

watch(accountIdParam, async (id) => {
  if (id && reconStore.session?.accountId !== id) {
    reconStore.reset()
    selectedAccountId.value = String(id)
    showStartForm.value = false
  }
})

async function startSession() {
  const startingCents = Math.round(parseFloat(startingBalanceStr.value || "0") * 100)
  const endingCents = Math.round(parseFloat(endingBalanceStr.value || "0") * 100)
  await reconStore.startSession({
    accountId: Number(selectedAccountId.value),
    statementDate: statementDate.value,
    endingBalance: endingCents,
    startingBalance: startingCents,
  })
  showStartForm.value = false
}

async function toggleSplit(splitId: number) {
  await reconStore.toggleSplit(splitId)
}

async function finalize() {
  await reconStore.finalize()
}

function viewAccount(id: number) {
  router.push({ name: "reconciliation-session", params: { accountId: id } })
}
</script>

<template>
  <div class="space-y-4">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Reconciliation</h2>
        <p class="text-sm text-muted-foreground">Match ledger entries against bank statements</p>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" v-if="reconStore.isActive" @click="finalize" :disabled="!reconStore.isBalanced">
          <Check class="h-4 w-4 mr-2" />
          Finalize
        </Button>
        <Button variant="outline" @click="showStartForm = true" v-if="!reconStore.isActive || showStartForm">
          <Wallet class="h-4 w-4 mr-2" />
          New Session
        </Button>
      </div>
    </div>

    <!-- Start Session Form -->
    <Card v-if="showStartForm && !reconStore.isActive">
      <CardHeader>
        <CardTitle>Start Reconciliation</CardTitle>
        <CardDescription>Select an account and enter statement details</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1 col-span-2">
            <Label>Account</Label>
            <AccountPicker
              v-model="selectedAccountId"
              select-placeholders
              placeholder="Select account..."
            />
          </div>
          <div class="space-y-1">
            <Label>Statement Date</Label>
            <DatePicker v-model="statementDate" />
          </div>
          <div class="space-y-1">
            <Label>Starting Balance</Label>
            <Input v-model="startingBalanceStr" placeholder="0.00" />
          </div>
          <div class="space-y-1 col-span-2">
            <Label>Ending Balance (per statement)</Label>
            <Input v-model="endingBalanceStr" placeholder="0.00" />
          </div>
        </div>
        <div class="flex justify-end mt-4">
          <Button @click="startSession" :disabled="!selectedAccountId">
            <RotateCw class="h-4 w-4 mr-2" />
            Begin Reconciliation
          </Button>
        </div>
      </CardContent>
    </Card>

    <!-- Active Reconciliation -->
    <template v-if="reconStore.isActive">
      <!-- Widgets row: Statement / Cleared / Difference -->
      <div class="grid grid-cols-3 gap-4">
        <Card>
          <CardHeader class="pb-2">
            <CardTitle class="text-sm font-medium text-muted-foreground">Statement Balance</CardTitle>
          </CardHeader>
          <CardContent>
            <p class="text-2xl font-bold">
              {{ formatCents(reconStore.statementBalance) }}
            </p>
            <p class="text-xs text-muted-foreground mt-1">
              {{ reconStore.session?.statementDate }}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="pb-2">
            <CardTitle class="text-sm font-medium text-muted-foreground">Cleared Total</CardTitle>
          </CardHeader>
          <CardContent>
            <p class="text-2xl font-bold">
              {{ formatCents(reconStore.clearedTotal) }}
            </p>
            <p class="text-xs text-muted-foreground mt-1">
              {{ reconStore.clearedSplits.length }} cleared splits
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="pb-2">
            <CardTitle class="text-sm font-medium text-muted-foreground">Difference</CardTitle>
          </CardHeader>
          <CardContent>
            <p :class="cn('text-2xl font-bold', balanceColor)">
              {{ formatCents(reconStore.difference) }}
              <span v-if="reconStore.isBalanced" class="text-sm ml-2">✓ Balanced</span>
            </p>
            <p class="text-xs text-muted-foreground mt-1">
              Cleared - Statement = {{ formatCents(reconStore.difference) }}
            </p>
          </CardContent>
        </Card>
      </div>

      <!-- Split list with search -->
      <Card>
        <CardHeader class="pb-2">
          <div class="flex items-center justify-between">
            <CardTitle class="text-sm">Unreconciled Transactions</CardTitle>
            <div class="relative w-64">
              <SearchIcon class="absolute left-2 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground" />
              <input
                v-model="searchQuery"
                placeholder="Search..."
                class="w-full rounded-md border border-input bg-background pl-7 pr-3 py-1.5 text-xs focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              />
            </div>
          </div>
        </CardHeader>
        <CardContent class="p-0">
          <div class="max-h-[420px] overflow-y-auto">
            <table class="w-full text-xs">
              <thead class="bg-muted/50 sticky top-0">
                <tr>
                  <th class="px-3 py-2 text-left font-medium text-muted-foreground w-8"></th>
                  <th class="px-3 py-2 text-left font-medium text-muted-foreground">Date</th>
                  <th class="px-3 py-2 text-left font-medium text-muted-foreground">Num</th>
                  <th class="px-3 py-2 text-left font-medium text-muted-foreground">Description</th>
                  <th class="px-3 py-2 text-right font-medium text-muted-foreground">Debit</th>
                  <th class="px-3 py-2 text-right font-medium text-muted-foreground">Credit</th>
                </tr>
              </thead>
              <tbody class="divide-y">
                <tr
                  v-for="split in filteredSplits"
                  :key="split.id"
                  class="hover:bg-muted/30 cursor-pointer transition-colors"
                  :class="{ 'bg-amber-50 dark:bg-amber-950/10': split.reconcileState === 'c' }"
                  @click="toggleSplit(split.id)"
                >
                  <td class="px-3 py-2 text-center">
                    <Circle
                      v-if="split.reconcileState === 'n'"
                      class="h-3.5 w-3.5 text-muted-foreground/40 inline-block"
                    />
                    <CheckCircle2
                      v-else-if="split.reconcileState === 'c'"
                      class="h-3.5 w-3.5 text-amber-500 inline-block"
                    />
                    <Check
                      v-else
                      class="h-3.5 w-3.5 text-emerald-500 inline-block"
                    />
                  </td>
                  <td class="px-3 py-2 whitespace-nowrap font-mono">{{ split.postDate }}</td>
                  <td class="px-3 py-2 whitespace-nowrap text-muted-foreground">{{ split.num || "" }}</td>
                  <td class="px-3 py-2">
                    <span class="truncate block max-w-[240px]">{{ split.description }}</span>
                    <span v-if="split.memo" class="text-muted-foreground truncate block max-w-[240px]">{{ split.memo }}</span>
                  </td>
                  <td class="px-3 py-2 text-right font-mono whitespace-nowrap">
                    {{ split.debit ? formatCents(split.debit) : "" }}
                  </td>
                  <td class="px-3 py-2 text-right font-mono whitespace-nowrap">
                    {{ split.credit ? formatCents(split.credit) : "" }}
                  </td>
                </tr>
                <tr v-if="filteredSplits.length === 0">
                  <td colspan="6" class="p-6 text-center text-sm text-muted-foreground">
                    All transactions reconciled for this account
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </template>

    <!-- Completed state -->
    <Card v-if="reconStore.session?.state === 'completed'">
      <CardHeader>
        <div class="flex items-center gap-2">
          <Check class="h-5 w-5 text-emerald-500" />
          <CardTitle>Reconciliation Complete</CardTitle>
        </div>
        <CardDescription>
          Completed on {{ reconStore.session.completedAt }}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <p class="text-sm text-muted-foreground">
          All cleared transactions have been marked as reconciled.
        </p>
        <Button variant="outline" class="mt-4" @click="reconStore.reset()">
          Start New Session
        </Button>
      </CardContent>
    </Card>

    <!-- Error display -->
    <div
      v-if="reconStore.error"
      class="rounded-md bg-destructive/10 p-3 text-sm text-destructive"
    >
      {{ reconStore.error }}
    </div>
  </div>
</template>
