<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("flex flex-col space-y-1.5 p-6", props.class))
</script>

<template>
  <div :class="computedClass">
    <slot />
  </div>
</template>
