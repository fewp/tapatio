<template>
  <div
    class="grid grid-cols-5 rounded-t-lg overflow-hidden text-xl relative w-full min-h-14"
  >
    <cards-navigator-item
      v-for="(item, index) in items"
      :key="item.id"
      v-bind="item"
      :is-first="index === 0"
      :is-last="index === items.length - 1"
      :active="item.id === activeItemId"
      :index="index"
      @update:active-item-id="emit('update:activeItemId', $event)"
    />
  </div>
</template>
<script setup lang="ts">
import type { CardNavigatorItem, CardItemId } from "./CardsNavigatorItem.vue";

const items: CardNavigatorItem[] = [
  { id: "all", name: "All", active: true },
  { id: "daily", name: "Daily" },
  { id: "weekly", name: "Weekly" },
  { id: "achievement", name: "Achievement" },
  { id: "challenge", name: "Challenges" },
];

const props = defineProps<{
  activeItemId: CardItemId;
}>();

type Emits = {
  (e: "update:activeItemId", id: CardItemId): void;
};
const emit = defineEmits<Emits>();
</script>
