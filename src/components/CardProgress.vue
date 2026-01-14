<template>
  <div class="flex flex-col gap-2 items-start justify-start w-3/5">
    <div class="flex gap-4">
      <span class="text-md text-secondary font-regular"> Count </span>
      <div class="flex">
        <span class="text-md text-secondary font-bold">{{
          formattedCount
        }}</span>
      </div>
    </div>

    <div class="w-4/5 bg-[#353B41] h-5 rounded-sm p-0.5 -skew-x-16">
      <div
        class="bg-[#01BEFF] h-full rounded-xs transition-all duration-500 ease-in-out"
        :style="`width: ${(props.count[0] / props.count[1]) * 100}%`"
      ></div>
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
