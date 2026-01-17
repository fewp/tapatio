<template>
  <div
    class="w-full flex items-center justify-between pr-6 gap-6 py-4 h-min pt-8 min-h-24"
  >
    <div
      class="justify-between items-center gap-3 w-full shadow-[0_4px_4px_0px_rgba(0,0,0,0.25)] outline outline-white px-4 py-3 flex h-22 bg-[#F0F2F5]/95 rounded-lg"
    >
      <div class="flex flex-col justify-center flex-1 gap-2">
        <div class="flex justify-between items-center">
          <span class="text-lg text-secondary font-semibold"
            >Complete at least 8 Daily Tasks</span
          >
          <task-counter :count="dailyCount" />
        </div>

        <task-progress-bar :count="dailyCount" />
      </div>

      <div class="flex items-center gap-2">
        <card-reward />

        <button
          class="bg-emphasis-primary shadow-[0_4px_4px_0px_rgba(0,0,0,0.25)] -skew-x-10 relative w-32 py-8 rounded-lg outline outline-black/20 -outline-offset-1 bg-[url(/assets/svg/emphasisButtonPattern.svg)] bg-cover bg-no-repeat"
        >
          <span
            class="absolute skew-x-10 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 font-montserrat text-2xl font-bold text-emphasis-tertiary"
          >
            Claim
          </span>
        </button>
      </div>
    </div>

    <button
      class="shadow-[0_4px_4px_0px_rgba(0,0,0,0.25)] -skew-x-6 relative rounded-lg outline outline-black/20 -outline-offset-1 min-w-64 bg-[url(/assets/svg/disabledButtonPattern.svg)] bg-no-repeat bg-contain bg-disabled-secondary h-22"
    >
      <span
        class="absolute skew-x-6 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 font-montserrat text-[26px] font-bold text-disabled-tertiary"
      >
        Claim All
      </span>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

const dailyCount = ref<Task["progress"]>([8, 8]);
</script>
