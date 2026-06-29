export interface ImportProfile {
  id?: number
  name: string
  fileFormat: string
  columnMapping: string
  defaultAccountId?: number
  createdAt?: string
  updatedAt?: string
}

export interface ImportPreviewItem {
  rowIndex: number
  date: string
  description: string
  amountCents: number
  debit: number
  credit: number
  memo?: string
  num?: string
  isDuplicate: boolean
  matchedTransactionId?: number
}

export interface CommitImportResult {
  imported: number
  skipped: number
}

export type WizardStep = "upload" | "mapping" | "preview" | "commit"

export interface ColumnMapping {
  date?: number
  description?: number
  amount?: number
  memo?: number
  num?: number
}
