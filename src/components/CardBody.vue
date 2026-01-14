<template>
  <div class="flex w-full">
    <card-progress :count="progress" />
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardProps = {
  progress: Task["progress"];
};

const props = defineProps<CardProps>();
</script>
