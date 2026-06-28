<script setup lang="ts">
import { cn } from "@/lib/utils"
import { ChevronDown } from "@lucide/vue"
import {
  SelectContent,
  SelectGroup,
  SelectLabel,
  SelectRoot,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectTrigger,
  SelectValue,
  type SelectContentProps,
} from "reka-ui"
import { computed, type HTMLAttributes } from "vue"

interface ExtendedSelectContentProps extends SelectContentProps {
  class?: HTMLAttributes["class"]
}
const contentProps = defineProps<ExtendedSelectContentProps>()
const computedClass = computed(() =>
  cn(
    "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
    contentProps.class,
  ),
)

const triggerClass = computed(() =>
  cn(
    "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
  ),
)
</script>

<template>
  <SelectRoot v-bind="$attrs">
    <SelectTrigger :class="triggerClass">
      <SelectValue placeholder="Select..." />
      <ChevronDown class="h-4 w-4 opacity-50" />
    </SelectTrigger>
    <SelectContent :class="computedClass" v-bind="contentProps">
      <SelectScrollUpButton class="flex cursor-default items-center justify-center py-1">
        <ChevronDown class="h-4 w-4 rotate-180" />
      </SelectScrollUpButton>
      <SelectGroup>
        <SelectLabel v-if="$slots.label" class="py-1.5 pl-8 pr-2 text-sm font-semibold">
          <slot name="label" />
        </SelectLabel>
        <slot />
      </SelectGroup>
      <SelectScrollDownButton class="flex cursor-default items-center justify-center py-1">
        <ChevronDown class="h-4 w-4" />
      </SelectScrollDownButton>
    </SelectContent>
  </SelectRoot>
</template>
