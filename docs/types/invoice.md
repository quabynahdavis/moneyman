# Invoice Types

## `Invoice`

| Field | Type | Description |
|---|---|---|
| `invoiceType` | `"INVOICE" \| "BILL"` | Direction |
| `invoiceNumber` | `string` | Unique reference |
| `contactId` | `number` | Client/vendor |
| `currencyCode` | `string` | ISO 4217 |
| `issueDate` / `dueDate` | `string` | ISO dates |
| `status` | `"DRAFT" \| "SENT" \| "PAID" \| "OVERDUE" \| "CANCELLED"` | Lifecycle |
| `subtotal`, `taxTotal`, `total`, `paidAmount` | `string` | Decimal strings |
| `lines` | `InvoiceLine[]` | Line items |

## `Contact`

| Field | Type | Description |
|---|---|---|
| `contactType` | `"CLIENT" \| "VENDOR" \| "BOTH"` | Role |
| `name`, `company`, `email`, `phone` | Various | Basic info |
| `address*` fields | Various | Postal address |
| `taxId` | `string \| null` | Tax registration |
| `defaultTerms` | `number` | Net days (e.g. 30) |
