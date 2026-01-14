<template>
  <button
    :to="id"
    class="p-4.5 min-w-47.5 active:scale-95 transition-all duration-50 flex items-center justify-center whitespace-nowrap leading-none"
    :class="[
      clipPathClass,
      activeClass,
      { 'pr-2.25': isLast, 'pl-2.25': isFirst },
    ]"
    @click="emit('update:activeItemId', id)"
  >
    {{ name }}
  </button>
</template>

<script setup lang="ts">
import type { CardTaskCategory } from "./CardsContainer.vue";

export type CardNavigatorItem = {
  id: CardTaskCategory;
  name: string;
  active?: boolean;
  isFirst?: boolean;
  isLast?: boolean;
  index?: number;
};

const props = withDefaults(defineProps<CardNavigatorItem>(), {
  active: false,
  isFirst: false,
  isLast: false,
});

type Emits = {
  (e: "update:activeItemId", id: CardTaskCategory): void;
};

const emit = defineEmits<Emits>();

const isActive = computed<boolean>(() => !!props.active);

const CLIP_TOP_RIGHT =
  "[clip-path:polygon(12px_0,100%_0,100%_100%,0_100%,0_100%)]";
const CLIP_BOTTOM_LEFT =
  "[clip-path:polygon(0_0,100%_0%,calc(100%-12px)100%,0%_100%)]";
const CLIP_BOTH_CORNERS =
  "[clip-path:polygon(12px_0,100%_0,calc(100%-12px)_100%,0_100%,0_100%)]";

const clipPathClass = computed<string>(() => {
  if (props.isFirst || props.isLast) {
    return props.isFirst ? CLIP_BOTTOM_LEFT : CLIP_TOP_RIGHT + " -ml-[11px]";
  }

  return CLIP_BOTH_CORNERS + " -ml-[11px]";
});

const activeBackgroundClasses: Record<CardTaskCategory, string> = {
  all: "bg-secondary",
  daily: "bg-daily",
  weekly: "bg-weekly",
  achievement: "bg-achievement",
  challenge: "bg-challenge",
};

const activeClass = computed<string>(() => {
  if (!isActive.value) return "bg-white text-secondary-light";

  return props.id === "all"
    ? "bg-secondary text-secondary-accent"
    : `text-white ${activeBackgroundClasses[props.id]}`;
});
</script>
