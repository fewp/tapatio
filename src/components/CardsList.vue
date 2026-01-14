<template>
  <div
    class="w-full flex flex-col items px-6 py-4 bg-black/15 h-full overflow-scroll gap-4"
  >
    <card v-for="(task, index) in tasks" :key="index" :task="task" />
  </div>
</template>

<script setup lang="ts">
import type { CardTaskCategory } from "./CardsContainer.vue";

export type Task = {
  category: CardTaskCategory;
  title: string;
  progress: [number, number];
  reward?: string;
};

const tasks: Task[] = [
  {
    category: "weekly",
    title: "Daily login",
    progress: [2, 5],
  },
  {
    category: "weekly",
    title: "Clear a Commission 15 time(s)",
    progress: [0, 15],
  },
  {
    category: "weekly",
    title: "Clear a Bounty 20 time(s)",
    progress: [12, 20],
  },
  {
    category: "weekly",
    title: "Daily login",
    progress: [2, 5],
  },
  {
    category: "weekly",
    title: "Clear a Commission 15 time(s)",
    progress: [0, 15],
  },
  {
    category: "weekly",
    title: "Clear a Bounty 20 time(s)",
    progress: [12, 20],
  },
];
</script>
