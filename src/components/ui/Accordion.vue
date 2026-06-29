<script setup lang="ts">
import { AccordionRoot, type AccordionRootEmits, type AccordionRootProps } from "reka-ui"

const props = defineProps<AccordionRootProps>()
const emit = defineEmits<AccordionRootEmits>()
</script>

<template>
  <AccordionRoot
    v-bind="props"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <slot />
  </AccordionRoot>
</template>
