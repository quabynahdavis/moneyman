<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("rounded-lg border bg-card text-card-foreground shadow-sm", props.class))
</script>

<template>
  <div :class="computedClass">
    <slot />
  </div>
</template>
