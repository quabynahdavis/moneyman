# Dark Mode

Moneyman supports light, dark, and system-following themes.

## CSS Variables

All colors are defined as CSS custom properties in `assets/css/main.css`:

- `:root` block — Light theme HSL values
- `.dark` block — Dark theme HSL values

The Tailwind config maps these to semantic color names (`background`, `foreground`, `primary`, `secondary`, `muted`, `accent`, `destructive`, `border`, `ring`, etc.).

## Theme Selector

1. User clicks the theme icon (Sun/Moon/Monitor) at the bottom of the sidebar
2. A Popover opens with three options: Light (Sun), Dark (Moon), System (Monitor)
3. The active theme is marked with a checkmark
4. Selecting a theme calls `uiStore.setTheme("light" | "dark" | "system")`
5. The store watches `isDark` and toggles `document.documentElement.classList.toggle("dark", isDark)`
6. The `dark` class on `<html>` activates the `.dark` CSS variable block
7. Preference is persisted to `localStorage` under `moneyman-theme`

The `system` mode respects `prefers-color-scheme` via `window.matchMedia`.
