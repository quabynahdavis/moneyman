import type { Account, AccountNode, Split, Transaction } from "@/types"
import { Decimal, toDecimal } from "@/utils/decimal"

export function useAccountingEngine() {
  function validateBalanced(splits: Split[]): void {
    if (splits.length < 2) {
      throw new Error("Transaction must have at least 2 splits")
    }

    let totalDebits = 0
    let totalCredits = 0

    for (const split of splits) {
      if (split.debit > 0 && split.credit > 0) {
        throw new Error("Split cannot have both debit and credit amounts")
      }

      totalDebits += split.debit
      totalCredits += split.credit
    }

    if (totalDebits !== totalCredits) {
      throw new Error(
        `Transaction out of balance: debits=${totalDebits} cents, credits=${totalCredits} cents`,
      )
    }
  }

  function createSimpleSplits(
    debitAccountId: number,
    creditAccountId: number,
    amountCents: number,
    memo?: string,
  ): [Split, Split] {
    return [
      { accountId: debitAccountId, debit: amountCents, credit: 0, memo: memo || null, reconcileState: "n", quantity: null, action: null },
      { accountId: creditAccountId, debit: 0, credit: amountCents, memo: memo || null, reconcileState: "n", quantity: null, action: null },
    ]
  }

  function accountNetChange(accountId: number, splits: Split[]): number {
    return splits
      .filter((s) => s.accountId === accountId)
      .reduce((acc, s) => acc + s.debit - s.credit, 0)
  }

  function computeRunningBalance(
    startingBalance: string,
    splits: Split[],
    normalBalance: "debit" | "credit",
  ): { splitId?: number; runningBalance: string }[] {
    let running = toDecimal(startingBalance)
    return splits.map((s) => {
      const change = toDecimal(s.debit - s.credit)
      const signedChange = normalBalance === "debit" ? change : change.negated()
      running = running.plus(signedChange)
      return {
        splitId: s.id,
        runningBalance: running.toString(),
      }
    })
  }

  function buildAccountTree(accounts: Account[]): AccountNode[] {
    const map = new Map<number, AccountNode>()
    const roots: AccountNode[] = []

    for (const acc of accounts) {
      map.set(acc.id, { ...acc, children: [] })
    }

    for (const acc of accounts) {
      const node = map.get(acc.id)!
      if (acc.parentId !== null && map.has(acc.parentId)) {
        const parent = map.get(acc.parentId)!
        parent.children = parent.children || []
        parent.children.push(node)
      } else {
        roots.push(node)
      }
    }

    return roots
  }

  function flattenAccountTree(tree: AccountNode[]): AccountNode[] {
    const result: AccountNode[] = []
    function walk(nodes: AccountNode[]) {
      for (const node of nodes) {
        result.push(node)
        if (node.children && node.children.length > 0) {
          walk(node.children)
        }
      }
    }
    walk(tree)
    return result
  }

  function createEmptyTransaction(): Transaction {
    return {
      currencyCode: "USD",
      description: "",
      notes: null,
      num: null,
      postDate: new Date().toISOString().split("T")[0],
      state: "UNRECONCILED",
      splits: [],
    }
  }

  function createEmptySplit(): Split {
    return {
      accountId: 0,
      debit: 0,
      credit: 0,
      memo: null,
      reconcileState: "n",
      quantity: null,
      action: null,
    }
  }

  return {
    validateBalanced,
    createSimpleSplits,
    accountNetChange,
    computeRunningBalance,
    buildAccountTree,
    flattenAccountTree,
    createEmptyTransaction,
    createEmptySplit,
  }
}
