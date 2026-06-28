<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("p-6 pt-0", props.class))
</script>

<template>
  <div :class="computedClass">
    <slot />
  </div>
</template>
