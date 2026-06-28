# Native Features

## Window Configuration (`tauri.conf.json`)

```json
{
  "app": {
    "windows": [
      {
        "title": "Moneyman",
        "width": 1200,
        "height": 800,
        "decorations": true,
        "transparent": false
      }
    ]
  }
}
```

## Planned Native Integrations

- **File System** — `tauri-plugin-fs` for PDF export and receipt attachments
- **Dialog** — `tauri-plugin-dialog` for file open/save dialogs (OFX/CSV import)
- **Notification** — `tauri-plugin-notification` for scheduled transaction reminders
- **Shell** — `tauri-plugin-shell` for opening PDFs in system viewer
- **Auto-update** — `tauri-plugin-updater` for app updates
