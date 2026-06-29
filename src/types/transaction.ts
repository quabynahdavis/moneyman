export type TransactionState = "UNRECONCILED" | "CLEARED" | "RECONCILED" | "VOID"

export type ReconcileState = "n" | "c" | "r"

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
  debit: number
  credit: number
  memo: string | null
  reconcileState: ReconcileState
  quantity: string | null
  action: string | null
}

export interface Transaction {
  id?: number
  currencyCode: string
  description: string
  notes: string | null
  num: string | null
  postDate: string
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
  const totalDebit = splits.reduce((acc, s) => acc + s.debit, 0)
  const totalCredit = splits.reduce((acc, s) => acc + s.credit, 0)
  return totalDebit === totalCredit
}

export function computeTransactionTotal(splits: Split[]): number {
  return splits.reduce((acc, s) => acc + s.debit - s.credit, 0)
}

export function getNetAmount(splits: Split[], accountId: number): number {
  return splits
    .filter((s) => s.accountId === accountId)
    .reduce((acc, s) => acc + s.debit - s.credit, 0)
}
