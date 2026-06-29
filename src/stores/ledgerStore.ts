import { defineStore } from "pinia"
import { ref, computed } from "vue"
import type { Transaction, Split, AccountType } from "@/types"
import { useAccountStore } from "./accountStore"
import { fromCents } from "@/utils/decimal"

export type AccountContext =
  | "ASSET"
  | "BANK"
  | "CASH"
  | "LIABILITY"
  | "CREDIT_CARD"
  | "PAYABLE"
  | "EXPENSE"
  | "INCOME"

export interface LedgerRow {
  id: string
  type: "transaction" | "split"
  txnId: number
  splitId?: number
  postDate: string
  num: string | null
  description: string
  transferAccount: string
  memo: string | null
  debit: number
  credit: number
  balance: number
  state: string
  splitCount: number
  expanded: boolean
}

export const ACCOUNT_CONTEXT_LABELS: Record<AccountContext, { debit: string; credit: string }> = {
  ASSET: { debit: "Deposit", credit: "Withdrawal" },
  BANK: { debit: "Deposit", credit: "Withdrawal" },
  CASH: { debit: "Deposit", credit: "Withdrawal" },
  LIABILITY: { debit: "Payment", credit: "Charge" },
  CREDIT_CARD: { debit: "Payment", credit: "Charge" },
  PAYABLE: { debit: "Payment", credit: "Charge" },
  EXPENSE: { debit: "Expense", credit: "Rebate" },
  INCOME: { debit: "Decrease", credit: "Income" },
}

function getAccountContext(accountType: string): AccountContext {
  const ctx: Record<string, AccountContext> = {
    ASSET: "ASSET", BANK: "BANK", CASH: "CASH",
    LIABILITY: "LIABILITY", CREDIT_CARD: "CREDIT_CARD", PAYABLE: "PAYABLE",
    EXPENSE: "EXPENSE", INCOME: "INCOME",
  }
  return ctx[accountType] || "ASSET"
}

export const useLedgerStore = defineStore("ledger", () => {
  const accountContext = ref<AccountContext>("ASSET")
  const accountId = ref<number | null>(null)

  const contextLabels = computed(() => ACCOUNT_CONTEXT_LABELS[accountContext.value])

  function setAccountContext(accountType: string) {
    accountContext.value = getAccountContext(accountType)
  }

  function buildLedgerRows(
    transactions: Transaction[],
    forAccountId: number | null,
    normalBalance: "debit" | "credit",
  ): LedgerRow[] {
    const rows: LedgerRow[] = []

    for (const txn of transactions) {
      const matchingSplits = forAccountId
        ? txn.splits.filter((s) => s.accountId === forAccountId)
        : txn.splits

      if (forAccountId && matchingSplits.length === 0) continue

      // For transfer column: if 2 splits total, show opposite account name
      const otherSplits = forAccountId
        ? txn.splits.filter((s) => s.accountId !== forAccountId)
        : []

      let transferAccount = ""
      if (txn.splits.length === 2 && otherSplits.length === 1) {
        transferAccount = otherSplits[0].accountName ?? ""
      } else if (txn.splits.length > 2) {
        transferAccount = "\u2014 Split Transaction \u2014"
      } else if (txn.splits.length === 2 && !forAccountId) {
        const self = txn.splits[0]
        transferAccount = txn.splits.find((s) => s.accountId !== self.accountId)?.accountName ?? ""
      }

      // Aggregate debit/credit for the matching splits
      const totalDebit = matchingSplits.reduce((sum, s) => sum + s.debit, 0)
      const totalCredit = matchingSplits.reduce((sum, s) => sum + s.credit, 0)

      rows.push({
        id: `txn-${txn.id}`,
        type: "transaction",
        txnId: txn.id!,
        postDate: txn.postDate,
        num: txn.num,
        description: txn.description,
        transferAccount,
        memo: null,
        debit: totalDebit,
        credit: totalCredit,
        balance: 0,
        state: txn.state,
        splitCount: txn.splits.length,
        expanded: false,
      })
    }

    // Sort by postDate ascending for running balance
    rows.sort((a, b) => a.postDate.localeCompare(b.postDate))

    // Compute running balance
    let running = 0
    for (const row of rows) {
      const change = row.debit - row.credit
      running += normalBalance === "debit" ? change : -change
      row.balance = running
    }

    return rows.reverse() // Sort back to most-recent-first for display
  }

  return {
    accountContext,
    contextLabels,
    setAccountContext,
    buildLedgerRows,
  }
})
