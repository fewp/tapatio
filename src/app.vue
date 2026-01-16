<template>
  <NuxtRouteAnnouncer />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { nuxtTasksRepository } from "@/adapters/repositories/nuxt/nuxtTasksRepository";

provide(TASKS_REPOSITORY, nuxtTasksRepository);
</script>
