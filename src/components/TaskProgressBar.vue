<template>
  <div class="bg-[#353B41] h-5 rounded-sm p-0.5 -skew-x-16">
    <div
      class="bg-[#01BEFF] h-full rounded-xs transition-all duration-500 ease-in-out"
      :style="`width: ${(count[0] / count[1]) * 100}%`"
    ></div>
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardProps = {
  count: Task["progress"];
};

defineProps<CardProps>();
</script>
