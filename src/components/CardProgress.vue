<template>
  <div class="flex flex-col gap-2 items-start justify-start w-3/5">
    <task-counter :count="count" />

    <task-progress-bar :count="count" class="w-4/5" />
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardProps = {
  count: Task["progress"];
};

const props = defineProps<CardProps>();
</script>
