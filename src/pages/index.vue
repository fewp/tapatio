<template>
  <div
    class="flex justify-between pt-12 p-8 bg-no-repeat bg-cover bg-center bg-[url(/assets/images/bg.png)] w-full h-screen"
  >
    <aside></aside>
    <main class="w-3/5 flex flex-col">
      <cards-container />
    </main>
  </div>
</template>

<script lang="ts" setup>
const tasksRepository = inject(TASKS_REPOSITORY)!;

const { data: tasks } = useAsyncData("tasks", () =>
  tasksRepository.getTasks({
    category: "daily",
    maxQuantity: 5,
    offset: 0,
  })
);

// watch(
//   tasks,
//   (newTasks) => {
//     console.log("Fetched tasks:", newTasks);
//   },
//   { immediate: true }
// );

onMounted(() => {
  console.log("hi", tasksRepository);
});
</script>
