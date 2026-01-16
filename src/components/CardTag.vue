<template>
  <span
    class="px-4 py-1 rounded-md text-white h-min select-none outline outline-black/5 -outline-offset-1"
    :class="backgroundClasses[category]"
  >
    {{ name }}
  </span>
</template>

<script setup lang="ts">
import type { CardTaskCategory } from "./CardsContainer.vue";

const backgroundClasses: Record<CardTaskCategory, string> = {
  all: "bg-secondary",
  daily: "bg-daily",
  weekly: "bg-weekly",
  achievement: "bg-achievement",
  challenge: "bg-challenge",
};

const props = defineProps<{
  category: CardTaskCategory;
}>();

const nameMap: Record<CardTaskCategory, string> = {
  all: "All",
  daily: "Daily",
  weekly: "Weekly",
  achievement: "Achievement",
  challenge: "Challenges",
};

const name = computed<string>(() => nameMap[props.category]);
</script>
