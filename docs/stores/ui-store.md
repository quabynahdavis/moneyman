# UI Store (`useUiStore`)

Controls theme, sidebar, and modal state.

## State

- `theme: "light" | "dark" | "system"` (persisted to localStorage)
- `sidebar: "expanded" | "collapsed"` (persisted)
- `activeModal: string | null`
- `modalData: Record<string, unknown>`

## Computed

| Property | Description |
|---|---|
| `isDark` | Resolved boolean (respects system preference) |
| `sidebarCollapsed` | Shortcut for `sidebar === "collapsed"` |

## Methods

| Method | Description |
|---|---|
| `setTheme(t)` | Update + persist theme |
| `toggleSidebar()` | Toggle sidebar state |
| `setSidebar(state)` | Explicit setter |
| `openModal(name, data?)` | Open a modal |
| `closeModal()` | Close current modal |

The store watches `isDark` and toggles the `dark` class on `<html>` automatically.
