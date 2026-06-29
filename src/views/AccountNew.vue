<script setup lang="ts">
import { ref, onMounted, computed } from "vue"
import { useRouter, useRoute } from "vue-router"
import { Button, Input, Label } from "@/components/ui"
import { useAccountStore } from "@/stores/accountStore"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountType } from "@/types"
import { toast } from "vue-sonner"
import Combobox from "@/components/Combobox.vue"

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
const loading = ref(false)
const error = ref("")

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
      toast.success("Account loaded for editing")
    } else {
      error.value = "Account not found"
      toast.error("Account not found")
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
  if (!name.value.trim()) {
    error.value = "Account name is required"
    toast.error("Account name is required")
    return
  }
  saving.value = true
  error.value = ""
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
      toast.success("Account updated")
    } else {
      await accountStore.createNewAccount({
        parentId: parentId.value,
        accountType: accountType.value,
        code: code.value || null,
        name: name.value.trim(),
        description: description.value || null,
        isPlaceholder: isPlaceholder.value,
      })
      toast.success("Account created")
    }
    router.push({ name: "accounts" })
  } catch (e: any) {
    const msg = typeof e === "string" ? e : "Failed to save account"
    error.value = msg
    toast.error(msg)
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
        <Combobox
          :model-value="accountType"
          :items="Object.entries(ACCOUNT_TYPE_LABELS).map(([value, label]) => ({ value, label }))"
          placeholder="Select account type..."
          @update:model-value="accountType = $event as AccountType"
        />
      </div>

      <div class="space-y-1">
        <Label>Parent Account</Label>
        <Combobox
          :model-value="parentId?.toString() ?? ''"
          :items="[
            { value: '', label: 'None (Root level)' },
            ...getParentCandidates().map(p => ({ value: p.id.toString(), label: `${p.name} (${p.accountType})` })),
          ]"
          placeholder="Select parent..."
          @update:model-value="parentId = $event ? Number($event) : null"
        />
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
