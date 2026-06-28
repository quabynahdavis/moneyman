export type TransactionState = "UNRECONCILED" | "CLEARED" | "RECONCILED" | "VOID"

export type RecurringFrequency =
  | "DAILY"
  | "WEEKLY"
  | "BIWEEKLY"
  | "MONTHLY"
  | "QUARTERLY"
  | "SEMI_ANNUAL"
  | "ANNUAL"

export interface Split {
  id?: number
  transactionId?: number
  accountId: number
  accountName?: string
  accountType?: string
  debitAmount: string
  creditAmount: string
  memo: string | null
  quantity: string | null
  action: string | null
  reconciledDate: string | null
}

export interface Transaction {
  id?: number
  currencyCode: string
  description: string | null
  notes: string | null
  payee: string | null
  number: string | null
  date: string
  datePosted: string
  state: TransactionState
  splits: Split[]
  createdAt?: string
  updatedAt?: string
}

export interface RecurringTransaction {
  id?: number
  templateTxnId: number
  templateTransaction?: Transaction
  frequency: RecurringFrequency
  intervalCount: number
  nextDate: string
  endDate: string | null
  autoExecute: boolean
  lastGenerated: string | null
  isActive: boolean
}

export function isTransactionBalanced(splits: Split[]): boolean {
  const total = splits.reduce(
    (acc, s) => {
      const d = new Decimal(s.debitAmount || "0")
      const c = new Decimal(s.creditAmount || "0")
      return { debit: acc.debit.plus(d), credit: acc.credit.plus(c) }
    },
    { debit: new Decimal(0), credit: new Decimal(0) },
  )
  return total.debit.equals(total.credit)
}

export function computeTransactionTotal(splits: Split[]): string {
  return splits
    .reduce((acc, s) => {
      return acc.plus(s.debitAmount || "0").minus(s.creditAmount || "0")
    }, new Decimal(0))
    .toString()
}

import Decimal from "decimal.js"
