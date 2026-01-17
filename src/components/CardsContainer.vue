<template>
  <div class="w-full flex flex-col items-end h-full">
    <cards-navigator
      :active-item-id="activeNavigatorItemId"
      @update:activeItemId="activeNavigatorItemId = $event"
    />

    <div class="flex flex-col w-full overflow-hidden">
      <div class="w-full y-2 bg-black/15 h-2"></div>

      <cards-list />

      <div class="w-full y-2 bg-black/15 h-2 rounded-b-lg"></div>
    </div>
    <cards-footer />
  </div>
</template>

<script setup lang="ts">
import type { TaskCategory } from "@/core/entities/Task";

export type CardTaskCategory = TaskCategory | "all";

const activeNavigatorItemId = ref<CardTaskCategory>("all");
</script>
