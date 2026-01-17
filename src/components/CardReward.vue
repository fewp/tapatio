<template>
  <div
    class="flex px-1 items-end justify-end rounded-lg border border-black/50 outline-2 outline-white -outline-offset-2 shadow-[0_3px_2px_0_rgba(0,0,0,0.4)] p-0 -skew-x-10 relative bg-offwhite bg-[url(/assets/svg/rewardPattern.svg)] bg-no-repeat bg-contain h-16 w-18"
  >
    <img
      src="/assets/images/pyroxene.webp"
      alt="Pyroxene Reward"
      class="absolute skew-x-10 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 object-contain w-13"
    />
    <span
      class="text-secondary -mb-0.5 z-1 font-montserrat font-bold tracking-tight text-shadow-[0_1px_0_rgb(255_255_255/1)]"
      >x30
    </span>
  </div>
</template>

<script setup lang="ts">
import type { RewardType } from "~/core/entities/Task";

type Reward = {
  path: string;
  alt: string;
};

const rewardTypesMap: Record<RewardType, Reward> = {
  credits: {
    path: "/assets/images/credit.webp",
    alt: "Credits Reward",
  },
  pyroxene: {
    path: "/assets/images/pyroxene.webp",
    alt: "Pyroxene Reward",
  },
  eleph: {
    path: "/assets/images/eleph.webp",
    alt: "Eleph Reward",
  },
};
</script>
