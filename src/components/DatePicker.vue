<script setup lang="ts">
import { computed } from "vue"
import { Button } from "@/components/ui"
import { CalendarIcon } from "@lucide/vue"
import { Popover, PopoverTrigger, PopoverContent } from "@/components/ui/popover"
import { Calendar } from "@/components/ui/calendar"
import { getLocalTimeZone, today, parseDate, type DateValue } from "@internationalized/date"
import { cn } from "@/lib/utils"

const props = withDefaults(defineProps<{
  modelValue?: string
  placeholder?: string
  disabled?: boolean
}>(), {
  modelValue: "",
  placeholder: "Select a date",
  disabled: false,
})

const emit = defineEmits<{
  "update:modelValue": [value: string]
}>()

const dateValue = computed({
  get: () => {
    if (!props.modelValue) return today(getLocalTimeZone())
    try { return parseDate(props.modelValue) }
    catch { return today(getLocalTimeZone()) }
  },
  set: (val: DateValue) => emit("update:modelValue", val.toString()),
})
</script>

<template>
  <Popover>
    <PopoverTrigger as-child :disabled="disabled">
      <Button
        variant="outline"
        :disabled="disabled"
        :class="cn('w-full justify-start text-left font-normal', !modelValue && 'text-muted-foreground')"
      >
        <CalendarIcon class="mr-2 h-4 w-4 shrink-0" />
        {{ modelValue ? new Intl.DateTimeFormat("en-US", { dateStyle: "long" }).format(parseDate(modelValue).toDate(getLocalTimeZone())) : placeholder }}
      </Button>
    </PopoverTrigger>
    <PopoverContent class="w-auto p-0">
      <Calendar v-model="dateValue" :initial-focus="true" :default-placeholder="today(getLocalTimeZone())" />
    </PopoverContent>
  </Popover>
</template>
