import type { InjectionKey } from "vue";
import type { TasksRepository } from "@/core/ports/repositories/TasksRepository";

export const TASKS_REPOSITORY: InjectionKey<TasksRepository> =
  Symbol("tasksRepository");
