<template>
  <div class="w-full flex flex-col items-end justify-between h-full">
    <cards-navigator
      :active-item-id="activeNavigatorItemId"
      @update:activeItemId="activeNavigatorItemId = $event"
    />

    <cards-list />

    <cards-footer />
  </div>
</template>

<script setup lang="ts">
import type { TaskCategory } from "@/core/entities/Task";

export type CardTaskCategory = TaskCategory | "all";

const activeNavigatorItemId = ref<CardTaskCategory>("all");
</script>
