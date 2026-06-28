# Dark Mode

Moneyman supports light, dark, and system-following themes.

## CSS Variables

All colors are defined as CSS custom properties in `assets/css/main.css`:

- `:root` block — Light theme HSL values
- `.dark` block — Dark theme HSL values

The Tailwind config maps these to semantic color names (`background`, `foreground`, `primary`, `secondary`, `muted`, `accent`, `destructive`, `border`, `ring`, etc.).

## Toggle Behavior

1. User clicks the theme button in the sidebar
2. `uiStore.setTheme("light" | "dark" | "system")` is called
3. The store watches `isDark` and toggles `document.documentElement.classList.toggle("dark", isDark)`
4. The `dark` class on `<html>` activates the `.dark` CSS variable block
5. Preference is persisted to `localStorage` under `moneyman-theme`
