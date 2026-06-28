# Sidebar Navigation

The `DefaultLayout` component renders a collapsible sidebar with navigation links and a header bar.

## Navigation Items

| # | Label | Icon |
|---|---|---|
| 1 | Dashboard | `LayoutDashboard` |
| 2 | Accounts | `BookType` |
| 3 | Ledger | `NotebookText` |
| 4 | Reconcile | `CheckCircle` |
| 5 | Scheduled | `CalendarClock` |
| 6 | Invoices | `FileText` |
| 7 | Contacts | `Users` |
| 8 | Budgets | `PiggyBank` |
| 9 | Reports | `BarChart3` |
| 10 | Settings | `Settings` |

## Behavior

- **Active State** — The current route's nav item is highlighted with `bg-accent`
- **Collapse** — Toggle via hamburger menu; state persisted to `uiStore`
- **Collapsed Mode** — Icons only, labels hidden, width reduces to `w-16`
- **Theme Toggle** — Bottom of sidebar switches between light/dark

## Header

- **Back/Forward** — Custom navigation stack (not browser history) tracked via `navStack` and `navIndex`; guarded against re-entrant watch mutation
- **Refresh Button** — `RefreshCw` icon dispatches route-specific refresh actions (re-fetches accounts, transactions, etc.)
- **Search Bar** — Visual placeholder (non-functional)
- **Title** — Displays `route.meta.title`
