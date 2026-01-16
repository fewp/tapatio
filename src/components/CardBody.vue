<template>
  <div class="flex w-full justify-between items-end">
    <card-progress :count="progress" />
    <div class="flex">
      <card-rewards />
      <button
        class="bg-primary shadow-[0_4px_4px_0px_rgba(0,0,0,0.25)] -skew-x-10 relative w-44 py-8 rounded-lg outline outline-black/20 -outline-offset-1 bg-[url(/assets/svg/buttonPattern.svg)] bg-bottom-right bg-contain bg-no-repeat"
      >
        <span
          class="absolute skew-x-10 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 font-montserrat text-2xl font-bold text-secondary"
        >
          Complete
        </span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardProps = {
  progress: Task["progress"];
};

const props = defineProps<CardProps>();
</script>
