import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { Account, AccountNode } from "@/types"
import { Decimal, toDecimal } from "@/utils/decimal"
import * as api from "@/services/api"

export const useAccountStore = defineStore("accounts", () => {
  const accounts = ref<Account[]>([])
  const tree = ref<AccountNode[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

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

  function rollupBalances(nodes: AccountNode[]): AccountNode[] {
    return nodes.map((node) => {
      const children = node.children ? rollupBalances(node.children) : []
      const childSum = children.reduce((acc, child) => {
        return acc.plus(child.balance)
      }, new Decimal(0))
      const total = toDecimal(node.balance).plus(childSum)
      return { ...node, balance: total.toString(), children }
    })
  }

  const treeWithRollup = computed(() => rollupBalances(tree.value))

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

  function addAccount(account: Account) {
    accounts.value.push(account)
  }

  function updateAccountInStore(id: number, updates: Partial<Account>) {
    const idx = accounts.value.findIndex((a) => a.id === id)
    if (idx !== -1) {
      accounts.value[idx] = { ...accounts.value[idx], ...updates }
    }
  }

  function removeAccount(id: number) {
    accounts.value = accounts.value.filter((a) => a.id !== id)
  }

  // ── API-backed actions ──

  async function fetchAccounts() {
    loading.value = true
    error.value = null
    try {
      const data = await api.listAccounts()
      accounts.value = data
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to fetch accounts"
    } finally {
      loading.value = false
    }
  }

  async function fetchAccountTree() {
    loading.value = true
    error.value = null
    try {
      tree.value = await api.getAccountTree()
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to fetch account tree"
    } finally {
      loading.value = false
    }
  }

  async function createNewAccount(payload: api.CreateAccountPayload) {
    error.value = null
    try {
      const account = await api.createAccount(payload)
      accounts.value.push(account)
      await fetchAccountTree()
      return account
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to create account"
      throw error.value
    }
  }

  async function updateExistingAccount(payload: api.UpdateAccountPayload) {
    error.value = null
    try {
      const account = await api.updateAccount(payload)
      const idx = accounts.value.findIndex((a) => a.id === account.id)
      if (idx !== -1) {
        accounts.value[idx] = account
      }
      await fetchAccountTree()
      return account
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "Failed to update account"
      throw error.value
    }
  }

  return {
    accounts,
    tree,
    loading,
    error,
    treeWithRollup,
    activeAccounts,
    accountMap,
    getAccountById,
    getChildren,
    getDescendantIds,
    getLeafAccounts,
    getAccountsByType,
    addAccount,
    updateAccountInStore,
    removeAccount,
    fetchAccounts,
    fetchAccountTree,
    createNewAccount,
    updateExistingAccount,
  }
})
