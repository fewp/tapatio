<template>
  <div class="flex gap-4">
    <span class="text-md text-secondary font-regular"> Count </span>
    <div class="flex">
      <span class="text-md text-secondary font-bold">{{ formattedCount }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardProps = {
  count: Task["progress"];
};

const props = defineProps<CardProps>();

const formattedCount = computed<string>(() => {
  return `${props.count[0]} / ${props.count[1]}`;
});
</script>
