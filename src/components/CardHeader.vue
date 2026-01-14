<template>
  <div
    class="flex items-center justify-start w-full gap-4 border-b-2 border-[#B7BDC2] pb-2"
  >
    <card-tag :category="category" />
    <span class="text-xl text-secondary font-semibold">{{ title }}</span>
  </div>
</template>

<script setup lang="ts">
import type { Task } from "./CardsList.vue";

type CardHeaderProps = {
  category: Task["category"];
  title: Task["title"];
};

defineProps<CardHeaderProps>();
</script>
