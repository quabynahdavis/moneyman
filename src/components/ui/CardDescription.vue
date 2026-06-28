<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("text-sm text-muted-foreground", props.class))
</script>

<template>
  <p :class="computedClass">
    <slot />
  </p>
</template>
