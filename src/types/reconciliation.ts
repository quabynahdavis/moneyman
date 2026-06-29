export interface ReconciliationSession {
  id?: number
  accountId: number
  statementDate: string
  endingBalance: number
  startingBalance: number
  state: "open" | "completed" | "cancelled"
  createdAt?: string
  completedAt?: string
}

export interface ReconcileSplit {
  id: number
  transactionId: number
  accountId: number
  accountName?: string
  accountType?: string
  postDate: string
  description: string
  num?: string
  debit: number
  credit: number
  memo?: string
  reconcileState: "n" | "c" | "r"
}

export interface ReconciliationData {
  session: ReconciliationSession
  splits: ReconcileSplit[]
  clearedTotal: number
  difference: number
}
