<script setup lang="ts">
import { ref, onMounted } from "vue"
import { Button, Input, Label, Select, SelectItem } from "@/components/ui"
import Dialog from "@/components/ui/Dialog.vue"
import { useAccountStore } from "@/stores/accountStore"
import { ACCOUNT_TYPE_LABELS } from "@/types"
import type { AccountType, Account } from "@/types"

const emit = defineEmits<{
  saved: [account: Account]
  closed: []
}>()

const accountStore = useAccountStore()

const name = ref("")
const accountType = ref<AccountType>("BANK")
const parentId = ref<number | null>(null)
const code = ref("")
const description = ref("")
const placeholder = ref(false)
const saving = ref(false)
const error = ref("")

const anchorTypes = ["ASSET", "LIABILITY", "EQUITY", "INCOME", "EXPENSE"] as AccountType[]

const validLeafTypes = accountStore.activeAccounts.filter(
  (a) => !a.placeholder && !anchorTypes.includes(a.accountType as AccountType),
)

function getParentCandidates(): Account[] {
  return accountStore.activeAccounts.filter(
    (a) => a.placeholder || anchorTypes.includes(a.accountType as AccountType),
  )
}

async function save() {
  if (!name.value.trim()) {
    error.value = "Account name is required"
    return
  }
  saving.value = true
  error.value = ""
  try {
    const account = await accountStore.createNewAccount({
      parentId: parentId.value,
      accountType: accountType.value,
      code: code.value || null,
      name: name.value.trim(),
      description: description.value || null,
      placeholder: placeholder.value,
    })
    emit("saved", account)
  } catch (e: any) {
    error.value = typeof e === "string" ? e : "Failed to create account"
  } finally {
    saving.value = false
  }
}

function cancel() {
  emit("closed")
}
</script>

<template>
  <div class="space-y-4">
    <h2 class="text-lg font-semibold">New Account</h2>
    <p class="text-sm text-muted-foreground">Add a new account to the Chart of Accounts</p>

    <div v-if="error" class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
      {{ error }}
    </div>

    <div class="space-y-3">
      <div class="space-y-1">
        <Label>Account Type</Label>
        <select
          v-model="accountType"
          class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <option v-for="(label, key) in ACCOUNT_TYPE_LABELS" :key="key" :value="key">
            {{ label }}
          </option>
        </select>
      </div>

      <div class="space-y-1">
        <Label>Parent Account</Label>
        <select
          v-model="parentId"
          class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <option :value="null">None (Root level)</option>
          <option
            v-for="p in getParentCandidates()"
            :key="p.id"
            :value="p.id"
          >
            {{ p.name }} ({{ p.accountType }})
          </option>
        </select>
      </div>

      <div class="space-y-1">
        <Label>Account Code</Label>
        <Input v-model="code" placeholder="e.g. 1100" />
      </div>

      <div class="space-y-1">
        <Label>Name *</Label>
        <Input v-model="name" placeholder="Checking Account" />
      </div>

      <div class="space-y-1">
        <Label>Description</Label>
        <Input v-model="description" placeholder="Optional description" />
      </div>

      <label class="flex items-center gap-2 text-sm">
        <input type="checkbox" v-model="placeholder" class="rounded border-input" />
        Placeholder (parent account, no transactions allowed)
      </label>
    </div>

    <div class="flex justify-end gap-2 pt-2">
      <Button variant="outline" @click="cancel">Cancel</Button>
      <Button :disabled="saving" @click="save">
        {{ saving ? "Saving..." : "Create Account" }}
      </Button>
    </div>
  </div>
</template>
