<script setup lang="ts">
import { ref, computed, watch } from "vue"
import { Check, ChevronsUpDown, ChevronLeft, Folder, SearchIcon } from "@lucide/vue"
import { Button } from "@/components/ui"
import {
  Command,
  CommandList,
  CommandGroup,
  CommandItem,
  CommandEmpty,
} from "@/components/ui/command"

import {
  DialogRoot as Dialog,
  DialogContent,
  DialogOverlay,
  DialogPortal,
} from "reka-ui"
import { useAccountStore } from "@/stores/accountStore"
import type { Account } from "@/types"

const props = withDefaults(defineProps<{
  modelValue: string
  placeholder?: string
  searchPlaceholder?: string
  emptyText?: string
}>(), {
  placeholder: "Select account...",
  searchPlaceholder: "Search accounts...",
  emptyText: "No matching accounts",
})

const emit = defineEmits<{
  "update:modelValue": [value: string]
}>()

const accountStore = useAccountStore()
const open = ref(false)
const searchQuery = ref("")
const selectedTab = ref<string>("")

// Map<tabParentId, drillAccountId> — tracks drill-down for each tab
const drillState = ref<Record<number, number>>({})

const topLevelGroups = computed(() => {
  return accountStore.activeAccounts
    .filter((a) => a.parentId === 1)
    .map((parent) => ({
      parent,
      children: accountStore.activeAccounts.filter(
        (child) =>
          child.parentId === parent.id &&
          (!child.isPlaceholder || hasChildren(child.id)),
      ),
    }))
    .filter((g) => g.children.length > 0)
})

function getDrillParentId(tabParentId: number): number | null {
  return drillState.value[tabParentId] ?? null
}

function getTabItems(tabParentId: number): Account[] {
  const drillId = getDrillParentId(tabParentId)
  if (drillId) {
    return accountStore.activeAccounts.filter(
      (a) => a.parentId === drillId && (!a.isPlaceholder || hasChildren(a.id)),
    )
  }
  const group = topLevelGroups.value.find((g) => g.parent.id === tabParentId)
  return group?.children ?? []
}

function getDrillAccount(tabParentId: number): Account | null {
  const drillId = getDrillParentId(tabParentId)
  if (!drillId) return null
  return accountStore.getAccountById(drillId) ?? null
}

function hasChildren(accountId: number): boolean {
  return accountStore.activeAccounts.some((a) => a.parentId === accountId && !a.isPlaceholder)
}

function handleAccountClick(account: Account, tabParentId: number) {
  if (hasChildren(account.id)) {
    drillState.value = { ...drillState.value, [tabParentId]: account.id }
  } else {
    selectAccount(account.id.toString())
  }
}

function drillBack(tabParentId: number) {
  const next = { ...drillState.value }
  delete next[tabParentId]
  drillState.value = next
}

function selectAccount(id: string) {
  emit("update:modelValue", id)
  open.value = false
  resetState()
}

function resetState() {
  searchQuery.value = ""
  drillState.value = {}
  selectedTab.value = ""
}

watch(topLevelGroups, (groups) => {
  if (groups.length > 0 && !selectedTab.value) {
    selectedTab.value = String(groups[0].parent.id)
  }
})

function onOpenChange(val: boolean) {
  open.value = val
  if (val) {
    if (topLevelGroups.value.length > 0 && !selectedTab.value) {
      selectedTab.value = String(topLevelGroups.value[0].parent.id)
    }
  } else {
    resetState()
  }
}

const selectedLabel = computed(() => {
  if (!props.modelValue) return props.placeholder
  const acc = accountStore.getAccountById(parseInt(props.modelValue))
  return acc ? `${acc.name} (${acc.accountType})` : props.placeholder
})

const searchResults = computed(() => {
  const q = searchQuery.value.toLowerCase().trim()
  if (!q) return []
  return accountStore.activeAccounts.filter((a) => {
    if (a.isPlaceholder && !hasChildren(a.id)) return false
    return (
      a.name.toLowerCase().includes(q) ||
      (a.code && a.code.toLowerCase().includes(q)) ||
      a.accountType.toLowerCase().replace(/_/g, " ").includes(q)
    )
  })
})

function selectFromSearch(account: Account) {
  if (hasChildren(account.id)) {
    searchQuery.value = ""
    const group = topLevelGroups.value.find((g) => g.parent.id === account.parentId)
    if (group) {
      selectedTab.value = String(group.parent.id)
      drillState.value = { ...drillState.value, [group.parent.id]: account.id }
    }
  } else {
    selectAccount(account.id.toString())
  }
}
</script>

<template>
  <Dialog :open="open" @update:open="onOpenChange">
    <Button
      variant="outline"
      role="combobox"
      :aria-expanded="open"
      class="w-full justify-between font-normal"
      @click="open = true"
    >
      {{ selectedLabel }}
      <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
    </Button>
    <DialogPortal>
      <DialogOverlay class="fixed inset-0 z-50 bg-black/80" />
      <DialogContent
        class="fixed left-1/2 top-1/2 z-50 w-full max-w-lg -translate-x-1/2 -translate-y-1/2 rounded-lg border bg-popover p-0 shadow-lg overflow-hidden"
        @escape-key-down="open = false"
        @pointer-down-outside="open = false"
      >
        <Command>
          <div class="p-2 pb-0">
            <div class="flex items-center gap-2 rounded-lg border border-input bg-input/30 h-8 px-2">
              <SearchIcon class="h-4 w-4 shrink-0 opacity-50" />
              <input
                v-model="searchQuery"
                :placeholder="searchPlaceholder"
                class="w-full bg-transparent text-sm text-foreground placeholder:text-muted-foreground outline-none"
              />
            </div>
          </div>

          <!-- Search results -->
          <template v-if="searchQuery">
            <CommandList>
              <CommandEmpty>{{ emptyText }}</CommandEmpty>
              <CommandGroup>
                <CommandItem
                  v-for="account in searchResults"
                  :key="account.id"
                  :value="String(account.id)"
                  @select="selectFromSearch(account)"
                >
                  <Folder v-if="hasChildren(account.id)" class="h-4 w-4 text-muted-foreground shrink-0" />
                  <span>{{ account.name }}</span>
                  <span class="ml-auto text-xs text-muted-foreground">{{ account.accountType.replace(/_/g, " ") }}</span>
                  <Check
                    v-if="modelValue === String(account.id) && !hasChildren(account.id)"
                    class="ml-2 h-4 w-4 shrink-0"
                  />
                </CommandItem>
              </CommandGroup>
            </CommandList>
          </template>

          <!-- Tab bar (always visible when groups exist) -->
          <div v-if="searchQuery || topLevelGroups.length > 0" class="flex gap-1 px-2 py-1 border-b">
            <button
              v-for="group in topLevelGroups"
              :key="group.parent.id"
              class="rounded-md px-2.5 py-1 text-xs font-medium transition-colors"
              :class="selectedTab === String(group.parent.id)
                ? 'bg-primary text-primary-foreground'
                : 'text-muted-foreground hover:text-foreground hover:bg-muted'"
              @click="selectedTab = String(group.parent.id); drillState = {}"
            >
              {{ group.parent.name }}
            </button>
          </div>

          <!-- Results area (scrollable) -->
          <div class="max-h-[280px] overflow-y-auto p-1">
            <!-- Search results -->
            <template v-if="searchQuery">
              <div v-if="searchResults.length === 0" class="p-6 text-center text-sm text-muted-foreground">
                {{ emptyText }}
              </div>
              <div v-else class="space-y-0.5">
                <button
                  v-for="account in searchResults"
                  :key="account.id"
                  class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-muted transition-colors"
                  @click="selectFromSearch(account)"
                >
                  <Folder v-if="hasChildren(account.id)" class="h-4 w-4 shrink-0 text-muted-foreground" />
                  <span class="truncate">{{ account.name }}</span>
                  <span class="ml-auto text-xs text-muted-foreground shrink-0">{{ account.accountType.replace(/_/g, " ") }}</span>
                  <Check
                    v-if="modelValue === String(account.id) && !hasChildren(account.id)"
                    class="ml-1 h-4 w-4 shrink-0"
                  />
                </button>
              </div>
            </template>

            <!-- Tab content -->
            <template v-else-if="selectedTab">
              <!-- Drill breadcrumb -->
              <div v-if="getDrillParentId(Number(selectedTab))" class="flex items-center gap-1 px-2 py-1 mb-1 border-b">
                <button
                  class="flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
                  @click="drillBack(Number(selectedTab))"
                >
                  <ChevronLeft class="h-3.5 w-3.5" />
                  <span class="font-medium">{{ getDrillAccount(Number(selectedTab))?.name }}</span>
                </button>
              </div>

              <div v-if="getTabItems(Number(selectedTab)).length === 0" class="p-6 text-center text-sm text-muted-foreground">
                No accounts in this category
              </div>

              <div v-else class="space-y-0.5">
                <button
                  v-for="account in getTabItems(Number(selectedTab))"
                  :key="account.id"
                  :data-selected="modelValue === String(account.id) && !hasChildren(account.id)"
                  class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-muted data-[selected=true]:bg-muted transition-colors"
                  @click="handleAccountClick(account, Number(selectedTab))"
                >
                  <Folder v-if="hasChildren(account.id)" class="h-4 w-4 shrink-0 text-muted-foreground" />
                  <span class="truncate">{{ account.name }}</span>
                  <span class="ml-auto text-xs text-muted-foreground shrink-0">{{ account.accountType.replace(/_/g, " ") }}</span>
                  <Check
                    v-if="modelValue === String(account.id) && !hasChildren(account.id)"
                    class="ml-1 h-4 w-4 shrink-0"
                  />
                </button>
              </div>
            </template>
          </div>
        </Command>
      </DialogContent>
    </DialogPortal>
  </Dialog>
</template>
