import { ref } from "vue"

export interface ConfirmOptions {
  title: string
  message: string
  confirmLabel?: string
  cancelLabel?: string
  variant?: "default" | "destructive"
}

const isOpen = ref(false)
const options = ref<ConfirmOptions>({ title: "", message: "" })
let resolveFn: ((value: boolean) => void) | null = null

export function useConfirm() {
  function confirm(opts: ConfirmOptions): Promise<boolean> {
    options.value = { confirmLabel: "Confirm", cancelLabel: "Cancel", variant: "default", ...opts }
    isOpen.value = true
    return new Promise((res) => {
      resolveFn = res
    })
  }

  function onConfirm() {
    isOpen.value = false
    resolveFn?.(true)
    resolveFn = null
  }

  function onCancel() {
    isOpen.value = false
    resolveFn?.(false)
    resolveFn = null
  }

  return { isOpen, options, confirm, onConfirm, onCancel }
}
