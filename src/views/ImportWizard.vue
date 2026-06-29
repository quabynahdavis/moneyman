<script setup lang="ts">
import { ref, computed, onMounted } from "vue"
import { useRouter } from "vue-router"
import {
  Card, CardContent, CardHeader, CardTitle, CardDescription,
  Button, Input, Label,
} from "@/components/ui"
import {
  Upload, FileSpreadsheet, ArrowRight, Check, X, AlertCircle,
  Save, Download, Table2,
} from "@lucide/vue"
import { useImportStore } from "@/stores/importStore"
import { useAccountStore } from "@/stores/accountStore"
import { cn } from "@/lib/utils"
import AccountPicker from "@/components/AccountPicker.vue"

const router = useRouter()
const importStore = useImportStore()
const accountStore = useAccountStore()

const profileName = ref("")
const showSaveProfile = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)

const columnLabels: Record<string, string> = {
  date: "Date",
  description: "Description",
  amount: "Amount",
  memo: "Memo",
  num: "Check #",
}

const availableColumns = computed(() => {
  const detected = detectColumns()
  return ["date", "description", "amount", "memo", "num"].filter(
    (c) => importStore.columnMapping[c as keyof typeof importStore.columnMapping] === undefined,
  )
})

const mappedColumns = computed(() => {
  return Object.entries(importStore.columnMapping).map(([field, idx]) => ({
    field,
    label: columnLabels[field] || field,
    index: idx,
  }))
})

const formatCents = (cents: number) => (cents / 100).toFixed(2)

const commitSummary = computed(() => {
  if (!importStore.commitResult) return null
  const total = importStore.commitResult.imported + importStore.commitResult.skipped
  return {
    total,
    imported: importStore.commitResult.imported,
    skipped: importStore.commitResult.skipped,
    pct: total > 0 ? Math.round((importStore.commitResult.imported / total) * 100) : 0,
  }
})

onMounted(() => {
  accountStore.fetchAccounts()
  importStore.loadProfiles()
})

function handleFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    importStore.setFile(reader.result as string, file.name)
    detectColumns()
  }
  reader.readAsText(file)
}

function detectColumns() {
  const lines = importStore.rawContent.trim().split("\n")
  if (lines.length < 1) return
  const header = lines[0].toLowerCase()
  const parts = header.split(",")
  const mapping: Record<string, number> = {}
  parts.forEach((part, idx) => {
    const p = part.trim().replace(/["']/g, "")
    if (/date/.test(p)) mapping.date = idx
    else if (/desc|memo|payee|name/.test(p) && !/memo/.test(p)) mapping.description = idx
    else if (/amount|sum|total|value/.test(p)) mapping.amount = idx
    else if (/memo|note/.test(p) && !/desc/.test(p)) mapping.memo = idx
    else if (/num|check|ref|number/.test(p)) mapping.num = idx
  })
  importStore.columnMapping = mapping as any
}

function setMapping(field: string, event: Event) {
  const val = (event.target as HTMLSelectElement).value
  const idx = val ? parseInt(val) : undefined

  // Remove any existing mapping for this column index
  const newMapping = { ...importStore.columnMapping }
  for (const [key, v] of Object.entries(newMapping)) {
    if (v === idx) delete (newMapping as any)[key]
  }
  if (idx !== undefined) {
    ;(newMapping as any)[field] = idx
  }
  importStore.columnMapping = newMapping
}

function getHeaderColumns(): string[] {
  const lines = importStore.rawContent.trim().split("\n")
  if (lines.length < 1) return []
  return lines[0].split(",").map((s) => s.trim().replace(/["']/g, ""))
}

async function saveProfile() {
  if (!profileName.value.trim()) return
  await importStore.saveProfile(profileName.value.trim())
  profileName.value = ""
  showSaveProfile.value = false
}

function loadProfile(profileId: number) {
  const profile = importStore.profiles.find((p) => p.id === profileId)
  if (!profile) return
  try {
    const mapping = JSON.parse(profile.columnMapping)
    importStore.columnMapping = mapping
    if (profile.defaultAccountId) {
      importStore.selectedAccountId = profile.defaultAccountId
    }
  } catch {}
}
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Import Transactions</h2>
        <p class="text-sm text-muted-foreground">Import from CSV or OFX files</p>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" @click="importStore.reset()" v-if="importStore.currentStep !== 'upload'">
          <X class="h-4 w-4 mr-2" /> Cancel
        </Button>
      </div>
    </div>

    <!-- Step indicators -->
    <div class="flex items-center gap-2 text-xs text-muted-foreground">
      <span :class="{ 'text-foreground font-semibold': importStore.currentStep === 'upload' }">
        <Upload class="h-3 w-3 inline mr-1" />Upload
      </span>
      <ArrowRight class="h-3 w-3" />
      <span :class="{ 'text-foreground font-semibold': importStore.currentStep === 'mapping' }">
        <Table2 class="h-3 w-3 inline mr-1" />Map
      </span>
      <ArrowRight class="h-3 w-3" />
      <span :class="{ 'text-foreground font-semibold': importStore.currentStep === 'preview' }">
        <FileSpreadsheet class="h-3 w-3 inline mr-1" />Preview
      </span>
      <ArrowRight class="h-3 w-3" />
      <span :class="{ 'text-foreground font-semibold': importStore.currentStep === 'commit' }">
        <Download class="h-3 w-3 inline mr-1" />Commit
      </span>
    </div>

    <!-- Upload step -->
    <Card v-if="importStore.currentStep === 'upload'">
      <CardHeader>
        <CardTitle>Select File</CardTitle>
        <CardDescription>CSV or OFX format supported</CardDescription>
      </CardHeader>
      <CardContent>
        <div
          class="flex flex-col items-center justify-center border-2 border-dashed border-muted-foreground/25 rounded-lg py-12 px-6 cursor-pointer hover:border-muted-foreground/50 transition-colors"
          @click="fileInput?.click()"
        >
          <Upload class="h-10 w-10 text-muted-foreground mb-4" />
          <p class="text-sm font-medium">Click to select a file</p>
          <p class="text-xs text-muted-foreground mt-1">CSV or OFX files</p>
          <input ref="fileInput" type="file" accept=".csv,.ofx,.qfx" class="hidden" @change="handleFileUpload" />
        </div>
        <div class="mt-4">
          <Label>Or use a saved profile</Label>
          <div class="flex gap-2 mt-1">
            <select
              v-if="importStore.profiles.length > 0"
              class="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm"
              @change="(e: any) => loadProfile(parseInt(e.target.value))"
            >
              <option value="">Select profile...</option>
              <option v-for="p in importStore.profiles" :key="p.id" :value="p.id">
                {{ p.name }} ({{ p.fileFormat }})
              </option>
            </select>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Mapping step -->
    <Card v-if="importStore.currentStep === 'mapping'">
      <CardHeader>
        <CardTitle>Map Columns</CardTitle>
        <CardDescription>Match file columns to transaction fields</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="space-y-3">
          <div class="grid grid-cols-[120px_1fr] gap-2 items-center" v-for="col in ['date', 'description', 'amount', 'memo', 'num']" :key="col">
            <Label class="text-sm">{{ columnLabels[col] }}</Label>
            <select
              class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
              :value="(importStore.columnMapping as any)[col] ?? ''"
              @change="(e: any) => setMapping(col, e)"
            >
              <option value="">-- Not mapped --</option>
              <option
                v-for="(hdr, idx) in getHeaderColumns()"
                :key="idx"
                :value="idx"
              >
                {{ hdr }} (Column {{ idx }})
              </option>
            </select>
          </div>
          <div class="flex items-center gap-2 mt-4">
            <Button @click="importStore.runPreview()" :disabled="!importStore.columnMapping.date || !importStore.columnMapping.amount">
              <ArrowRight class="h-4 w-4 mr-2" />
              Preview
            </Button>
            <Button variant="outline" @click="showSaveProfile = !showSaveProfile">
              <Save class="h-4 w-4 mr-2" /> Save Mapping
            </Button>
          </div>
          <div v-if="showSaveProfile" class="flex gap-2 items-center mt-2">
            <Input v-model="profileName" placeholder="Profile name..." class="max-w-xs" />
            <Button size="sm" @click="saveProfile" :disabled="!profileName.trim()">Save</Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Preview step -->
    <Card v-if="importStore.currentStep === 'preview'">
      <CardHeader>
        <div class="flex items-center justify-between">
          <div>
            <CardTitle>Preview</CardTitle>
            <CardDescription>
              {{ importStore.previewItems.length }} rows parsed
              <span v-if="importStore.totalAmount"> · Net: {{ formatCents(importStore.totalAmount) }}</span>
            </CardDescription>
          </div>
          <div class="flex items-center gap-2">
            <Button variant="outline" @click="importStore.goToStep('mapping')">
              <ArrowRight class="h-4 w-4 mr-2 rotate-180" /> Back
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <div class="space-y-3">
          <div class="flex items-center gap-2">
            <Label class="text-sm shrink-0">Default Account</Label>
            <div class="w-64">
              <AccountPicker
                :model-value="String(importStore.selectedAccountId ?? '')"
                select-placeholders
                placeholder="Select account..."
                @update:model-value="importStore.selectedAccountId = $event ? Number($event) : null"
              />
            </div>
          </div>

          <div class="max-h-[320px] overflow-y-auto border rounded-md">
            <table class="w-full text-xs">
              <thead class="bg-muted/50 sticky top-0">
                <tr>
                  <th class="px-2 py-1.5 text-left font-medium text-muted-foreground">#</th>
                  <th class="px-2 py-1.5 text-left font-medium text-muted-foreground">Date</th>
                  <th class="px-2 py-1.5 text-left font-medium text-muted-foreground">Description</th>
                  <th class="px-2 py-1.5 text-right font-medium text-muted-foreground">Debit</th>
                  <th class="px-2 py-1.5 text-right font-medium text-muted-foreground">Credit</th>
                </tr>
              </thead>
              <tbody class="divide-y">
                <tr v-for="item in importStore.previewItems" :key="item.rowIndex" class="hover:bg-muted/30">
                  <td class="px-2 py-1.5 text-muted-foreground">{{ item.rowIndex }}</td>
                  <td class="px-2 py-1.5 whitespace-nowrap font-mono">{{ item.date }}</td>
                  <td class="px-2 py-1.5 max-w-[200px]">
                    <span class="truncate block">{{ item.description }}</span>
                  </td>
                  <td class="px-2 py-1.5 text-right font-mono">
                    {{ item.debit ? formatCents(item.debit) : "" }}
                  </td>
                  <td class="px-2 py-1.5 text-right font-mono">
                    {{ item.credit ? formatCents(item.credit) : "" }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <div class="flex justify-end gap-2 mt-2">
            <Button
              @click="importStore.commit()"
              :disabled="!importStore.selectedAccountId || importStore.previewItems.length === 0"
            >
              <Download class="h-4 w-4 mr-2" />
              Import {{ importStore.previewItems.length }} Transactions
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Commit step -->
    <Card v-if="importStore.currentStep === 'commit' && commitSummary">
      <CardHeader>
        <div class="flex items-center gap-2">
          <Check class="h-5 w-5 text-emerald-500" />
          <CardTitle>Import Complete</CardTitle>
        </div>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-3 gap-4">
          <div class="text-center p-4 bg-muted/30 rounded-lg">
            <p class="text-2xl font-bold text-emerald-500">{{ commitSummary.imported }}</p>
            <p class="text-xs text-muted-foreground mt-1">Imported</p>
          </div>
          <div class="text-center p-4 bg-muted/30 rounded-lg">
            <p class="text-2xl font-bold text-amber-500">{{ commitSummary.skipped }}</p>
            <p class="text-xs text-muted-foreground mt-1">Skipped (duplicates)</p>
          </div>
          <div class="text-center p-4 bg-muted/30 rounded-lg">
            <p class="text-2xl font-bold">{{ commitSummary.pct }}%</p>
            <p class="text-xs text-muted-foreground mt-1">Success Rate</p>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-4">
          <Button variant="outline" @click="router.push({ name: 'ledger' })">
            View Ledger
          </Button>
          <Button @click="importStore.reset()">Import Another File</Button>
        </div>
      </CardContent>
    </Card>

    <!-- Error -->
    <div
      v-if="importStore.error"
      class="rounded-md bg-destructive/10 p-3 text-sm text-destructive flex items-start gap-2"
    >
      <AlertCircle class="h-4 w-4 mt-0.5 shrink-0" />{{ importStore.error }}
    </div>

    <!-- Loading overlay -->
    <div v-if="importStore.loading" class="flex items-center justify-center py-8">
      <div class="flex items-center gap-2 text-sm text-muted-foreground">
        <div class="animate-spin h-4 w-4 border-2 border-current border-t-transparent rounded-full" />
        Processing...
      </div>
    </div>
  </div>
</template>
