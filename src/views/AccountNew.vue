<script setup lang="ts">
import { ref, onMounted, computed } from "vue"
import { useRouter, useRoute } from "vue-router"
import { Button, Input, Label } from "@/components/ui"
import { useAccountStore } from "@/stores/accountStore"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountType } from "@/types"

const router = useRouter()
const route = useRoute()
const accountStore = useAccountStore()

const editId = computed(() => {
  const v = route.query.editId
  return v ? Number(v) : null
})

const name = ref("")
const accountType = ref<AccountType>("BANK")
const parentId = ref<number | null>(null)
const code = ref("")
const description = ref("")
const isPlaceholder = ref(false)
const saving = ref(false)
const error = ref("")
const loading = ref(false)

const anchorTypes = ["ASSET", "LIABILITY", "EQUITY", "INCOME", "EXPENSE"] as AccountType[]

const isEditing = computed(() => editId.value !== null)

onMounted(async () => {
  if (!editId.value) return
  loading.value = true
  try {
    await accountStore.fetchAccounts()
    const account = accountStore.getAccountById(editId.value)
    if (account) {
      name.value = account.name
      accountType.value = account.accountType as AccountType
      parentId.value = account.parentId
      code.value = account.code ?? ""
      description.value = account.description ?? ""
      isPlaceholder.value = account.isPlaceholder
    } else {
      error.value = "Account not found"
    }
  } finally {
    loading.value = false
  }
})

function getParentCandidates() {
  return accountStore.activeAccounts.filter(
    (a) => a.isPlaceholder || anchorTypes.includes(a.accountType as AccountType),
  )
}

async function save() {
  if (!name.value.trim()) { error.value = "Account name is required"; return }
  saving.value = true; error.value = ""
  try {
    if (isEditing.value) {
      await accountStore.updateExistingAccount({
        id: editId.value!,
        name: name.value.trim(),
        accountType: accountType.value,
        parentId: parentId.value,
        code: code.value || null,
        description: description.value || null,
        isPlaceholder: isPlaceholder.value,
      })
    } else {
      await accountStore.createNewAccount({
        parentId: parentId.value,
        accountType: accountType.value,
        code: code.value || null,
        name: name.value.trim(),
        description: description.value || null,
        isPlaceholder: isPlaceholder.value,
      })
    }
    router.push({ name: "accounts" })
  } catch (e: any) {
    error.value = typeof e === "string" ? e : "Failed to save account"
  } finally { saving.value = false }
}
</script>

<template>
  <div class="space-y-6">
    <div v-if="loading" class="p-8 text-center text-sm text-muted-foreground">Loading...</div>
    <template v-else>
      <div v-if="error" class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">{{ error }}</div>

      <div class="space-y-1">
        <Label>Account Type</Label>
        <select v-model="accountType" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
          <option v-for="(label, key) in ACCOUNT_TYPE_LABELS" :key="key" :value="key">{{ label }}</option>
        </select>
      </div>

      <div class="space-y-1">
        <Label>Parent Account</Label>
        <select v-model="parentId" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
          <option :value="null">None (Root level)</option>
          <option v-for="p in getParentCandidates()" :key="p.id" :value="p.id">{{ p.name }} ({{ p.accountType }})</option>
        </select>
      </div>

      <div class="space-y-1"><Label>Account Code</Label><Input v-model="code" placeholder="e.g. 1100" /></div>
      <div class="space-y-1"><Label>Name *</Label><Input v-model="name" placeholder="Checking Account" /></div>
      <div class="space-y-1"><Label>Description</Label><Input v-model="description" placeholder="Optional description" /></div>

      <label class="flex items-center gap-2 text-sm">
        <input type="checkbox" v-model="isPlaceholder" class="rounded border-input" />
        Placeholder (parent only, no transactions)
      </label>

      <div class="flex justify-end gap-2 pt-4 border-t">
        <Button variant="outline" @click="router.back()">Cancel</Button>
        <Button :disabled="saving" @click="save">{{ saving ? "Saving..." : isEditing ? "Update Account" : "Create Account" }}</Button>
      </div>
    </template>
  </div>
</template>
