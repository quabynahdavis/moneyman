import type { Account, AccountNode } from "@/types"

export function buildAccountTree(accounts: Account[]): AccountNode[] {
  const map = new Map<number, AccountNode>()
  const roots: AccountNode[] = []

  for (const acc of accounts) {
    map.set(acc.id, {
      id: acc.id,
      parentId: acc.parentId,
      accountType: acc.accountType,
      code: acc.code,
      name: acc.name,
      description: acc.description,
      currencyCode: acc.currencyCode,
      isPlaceholder: acc.isPlaceholder,
      isActive: acc.isActive,
      sortOrder: acc.sortOrder,
      balance: acc.balance,
      children: [],
    })
  }

  for (const acc of accounts) {
    const node = map.get(acc.id)!
    if (acc.parentId !== null && map.has(acc.parentId)) {
      map.get(acc.parentId)!.children!.push(node)
    } else {
      roots.push(node)
    }
  }

  return roots
}
