# Sidebar Navigation

The `DefaultLayout` component renders a collapsible sidebar with navigation links.

## Navigation Items

1. Dashboard (`LayoutDashboard`)
2. Accounts (`BookType`)
3. Ledger (`NotebookText`)
4. Reconcile (`CheckCircle`)
5. Scheduled (`CalendarClock`)
6. Invoices (`FileText`)
7. Contacts (`Users`)
8. Budgets (`PiggyBank`)
9. Reports (`BarChart3`)

## Behavior

- **Active State** — The current route's nav item is highlighted with `bg-accent`
- **Collapse** — Toggle via hamburger menu; state persisted to localStorage
- **Collapsed Mode** — Icons only, labels hidden, width reduces to `w-16`
- **Theme Toggle** — Bottom of sidebar switches between light/dark

## Header

- Displays the current route's `meta.title`
- Search bar (visual placeholder)
- Theme-aware background (`bg-card`)
