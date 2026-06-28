export type AccountType =
  | "ROOT"
  | "ASSET"
  | "BANK"
  | "CASH"
  | "INVESTMENT"
  | "STOCK"
  | "MUTUAL_FUND"
  | "RECEIVABLE"
  | "LIABILITY"
  | "CREDIT_CARD"
  | "PAYABLE"
  | "EQUITY"
  | "INCOME"
  | "EXPENSE"

export interface Account {
  id: number
  parentId: number | null
  accountType: AccountType
  code: string | null
  name: string
  description: string | null
  currencyCode: string
  isPlaceholder: boolean
  isActive: boolean
  sortOrder: number
  balance: string
  createdAt: string
  updatedAt: string
}

export interface AccountNode {
  id: number
  parentId: number | null
  accountType: AccountType
  code: string | null
  name: string
  description: string | null
  currencyCode: string
  isPlaceholder: boolean
  isActive: boolean
  sortOrder: number
  balance: string
  children?: AccountNode[]
}

export interface AccountBalance {
  accountId: number
  balance: string
}

export const ACCOUNT_TYPE_LABELS: Record<AccountType, string> = {
  ROOT: "Root",
  ASSET: "Asset",
  BANK: "Bank",
  CASH: "Cash",
  INVESTMENT: "Investment",
  STOCK: "Stock",
  MUTUAL_FUND: "Mutual Fund",
  RECEIVABLE: "Accounts Receivable",
  LIABILITY: "Liability",
  CREDIT_CARD: "Credit Card",
  PAYABLE: "Accounts Payable",
  EQUITY: "Equity",
  INCOME: "Income",
  EXPENSE: "Expense",
}

export const ACCOUNT_TYPE_TAXONOMY: AccountType[] = [
  "ROOT",
  "ASSET",
  "BANK",
  "CASH",
  "INVESTMENT",
  "STOCK",
  "MUTUAL_FUND",
  "RECEIVABLE",
  "LIABILITY",
  "CREDIT_CARD",
  "PAYABLE",
  "EQUITY",
  "INCOME",
  "EXPENSE",
]

export function isAccountParentType(t: AccountType): boolean {
  return ["ROOT", "ASSET", "LIABILITY", "EQUITY", "INCOME", "EXPENSE"].includes(t)
}

export function getAccountNormalBalance(t: AccountType): "debit" | "credit" {
  switch (t) {
    case "ASSET":
    case "BANK":
    case "CASH":
    case "INVESTMENT":
    case "STOCK":
    case "MUTUAL_FUND":
    case "RECEIVABLE":
    case "EXPENSE":
      return "debit"
    case "LIABILITY":
    case "CREDIT_CARD":
    case "PAYABLE":
    case "EQUITY":
    case "INCOME":
    case "ROOT":
      return "credit"
  }
}
