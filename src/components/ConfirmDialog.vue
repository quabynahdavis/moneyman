<script setup lang="ts">
import { useConfirm } from "@/composables/useConfirm"
import { Button } from "@/components/ui"
import {
  DialogRoot as Dialog,
  DialogContent,
} from "reka-ui"

const { isOpen, options, onConfirm, onCancel } = useConfirm()
</script>

<template>
  <Dialog :open="isOpen" @update:open="onCancel">
    <DialogContent
      class="fixed left-1/2 top-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border bg-background p-6 shadow-lg"
      @escape-key-down="onCancel"
      @pointer-down-outside="onCancel"
    >
      <h2 class="text-lg font-semibold mb-2">{{ options.title }}</h2>
      <p class="text-sm text-muted-foreground mb-6">{{ options.message }}</p>
      <div class="flex justify-end gap-2">
        <Button variant="outline" @click="onCancel">{{ options.cancelLabel }}</Button>
        <Button
          :variant="options.variant === 'destructive' ? 'destructive' : 'default'"
          @click="onConfirm"
        >{{ options.confirmLabel }}</Button>
      </div>
    </DialogContent>
  </Dialog>
</template>
