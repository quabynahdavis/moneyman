import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { Account, AccountTreeNode } from "@/types"
import { useAccountingEngine } from "@/composables/useAccountingEngine"

export const useAccountStore = defineStore("accounts", () => {
  const accounts = ref<Account[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const { buildAccountTree } = useAccountingEngine()

  const accountTree = computed<AccountTreeNode[]>(() => {
    return buildAccountTree(accounts.value)
  })

  const activeAccounts = computed(() =>
    accounts.value.filter((a) => a.isActive),
  )

  const accountMap = computed(() => {
    const map = new Map<number, Account>()
    for (const acc of accounts.value) {
      map.set(acc.id, acc)
    }
    return map
  })

  function getAccountById(id: number): Account | undefined {
    return accountMap.value.get(id)
  }

  function getChildren(parentId: number): Account[] {
    return accounts.value.filter((a) => a.parentId === parentId)
  }

  function getDescendantIds(accountId: number): number[] {
    const ids: number[] = []
    const children = getChildren(accountId)
    for (const child of children) {
      ids.push(child.id)
      ids.push(...getDescendantIds(child.id))
    }
    return ids
  }

  function getLeafAccounts(parentId: number | null = null): Account[] {
    const list = parentId ? getChildren(parentId) : accounts.value
    return list.filter((a) => {
      const children = getChildren(a.id)
      return children.length === 0
    })
  }

  function getAccountsByType(type: string): Account[] {
    return accounts.value.filter((a) => a.accountType === type)
  }

  function setAccounts(data: Account[]) {
    accounts.value = data
  }

  function addAccount(account: Account) {
    accounts.value.push(account)
  }

  function updateAccount(id: number, updates: Partial<Account>) {
    const idx = accounts.value.findIndex((a) => a.id === id)
    if (idx !== -1) {
      accounts.value[idx] = { ...accounts.value[idx], ...updates }
    }
  }

  function removeAccount(id: number) {
    accounts.value = accounts.value.filter((a) => a.id !== id)
  }

  return {
    accounts,
    loading,
    error,
    accountTree,
    activeAccounts,
    accountMap,
    getAccountById,
    getChildren,
    getDescendantIds,
    getLeafAccounts,
    getAccountsByType,
    setAccounts,
    addAccount,
    updateAccount,
    removeAccount,
  }
})
