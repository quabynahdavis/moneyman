import type { Account, AccountNode, Split, Transaction } from "@/types"
import { Decimal, toDecimal } from "@/utils/decimal"

export function useAccountingEngine() {
  function validateBalanced(splits: Split[]): void {
    if (splits.length < 2) {
      throw new Error("Transaction must have at least 2 splits")
    }

    let totalDebits = new Decimal(0)
    let totalCredits = new Decimal(0)

    for (const split of splits) {
      const d = toDecimal(split.debitAmount || "0")
      const c = toDecimal(split.creditAmount || "0")

      if (d.greaterThan(0) && c.greaterThan(0)) {
        throw new Error("Split cannot have both debit and credit amounts")
      }

      totalDebits = totalDebits.plus(d)
      totalCredits = totalCredits.plus(c)
    }

    if (!totalDebits.equals(totalCredits)) {
      throw new Error(
        `Transaction out of balance: debits=${totalDebits.toString()}, credits=${totalCredits.toString()}`,
      )
    }
  }

  function createSimpleSplits(
    debitAccountId: number,
    creditAccountId: number,
    amount: string,
    memo?: string,
  ): [Split, Split] {
    return [
      { accountId: debitAccountId, debitAmount: amount, creditAmount: "0", memo: memo || null, quantity: null, action: null, reconciledDate: null },
      { accountId: creditAccountId, debitAmount: "0", creditAmount: amount, memo: memo || null, quantity: null, action: null, reconciledDate: null },
    ]
  }

  function accountNetChange(accountId: number, splits: Split[]): string {
    return splits
      .filter((s) => s.accountId === accountId)
      .reduce((acc, s) => {
        return acc.plus(s.debitAmount || "0").minus(s.creditAmount || "0")
      }, new Decimal(0))
      .toString()
  }

  function computeRunningBalance(
    startingBalance: string,
    splits: Split[],
    normalBalance: "debit" | "credit",
  ): { splitId?: number; runningBalance: string }[] {
    let running = toDecimal(startingBalance)
    return splits.map((s) => {
      const change = toDecimal(s.debitAmount || "0").minus(s.creditAmount || "0")
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
      description: null,
      notes: null,
      payee: null,
      number: null,
      date: new Date().toISOString().split("T")[0],
      datePosted: new Date().toISOString().split("T")[0],
      state: "UNRECONCILED",
      splits: [],
    }
  }

  function createEmptySplit(): Split {
    return {
      accountId: 0,
      debitAmount: "0",
      creditAmount: "0",
      memo: null,
      quantity: null,
      action: null,
      reconciledDate: null,
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
