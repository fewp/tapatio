<template>
  <div
    class="w-full shadow-[0_4px_0_0_rgba(0,0,0,0.5)] outline outline-white px-4 py-3 flex flex-col justify-between bg-[url(/assets/svg/cardBackgroundPattern.svg)] bg-no-repeat bg-contain min-h-40 bg-[#F0F2F5]/95 rounded-2xl"
  >
    <card-header :title="task.title" :category="task.category" />
    <card-body :progress="task.progress" />
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardProps = {
  task: Task;
};

const props = defineProps<CardProps>();
</script>
