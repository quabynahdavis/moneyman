export type InvoiceType = "INVOICE" | "BILL"
export type InvoiceStatus = "DRAFT" | "SENT" | "PAID" | "OVERDUE" | "CANCELLED"
export type ContactType = "CLIENT" | "VENDOR" | "BOTH"
export type LineType = "ITEM" | "SUBTOTAL" | "DISCOUNT"

export interface Contact {
  id?: number
  contactType: ContactType
  name: string
  company: string | null
  email: string | null
  phone: string | null
  addressLine1: string | null
  addressLine2: string | null
  city: string | null
  state: string | null
  postalCode: string | null
  country: string | null
  taxId: string | null
  defaultTerms: number
  currencyCode: string
  notes: string | null
  isActive: boolean
  createdAt?: string
  updatedAt?: string
}

export interface InvoiceLine {
  id?: number
  invoiceId?: number
  lineType: LineType
  description: string
  quantity: number
  unitPrice: string
  taxRate: number
  amount: string
  sortOrder: number
}

export interface Invoice {
  id?: number
  invoiceType: InvoiceType
  invoiceNumber: string
  contactId: number
  contact?: Contact
  currencyCode: string
  issueDate: string
  dueDate: string
  paymentTerms: number
  status: InvoiceStatus
  subtotal: string
  taxTotal: string
  total: string
  paidAmount: string
  notes: string | null
  txnId: number | null
  lines: InvoiceLine[]
  createdAt?: string
  updatedAt?: string
}
