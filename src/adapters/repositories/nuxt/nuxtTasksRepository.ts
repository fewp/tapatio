import { Result } from "@/core/helpers/result";
import { invoke } from "@tauri-apps/api/core";
import type { TasksRepository } from "@/core/ports/repositories/TasksRepository";
import type { Task } from "@/core/entities/Task";

export const nuxtTasksRepository: TasksRepository = {
  async getTasks({ category, maxQuantity, offset }) {
    try {
      try {
        const result = await invoke("list_tasks");
        console.log("[rpc] tasks ->", result);
      } catch (e) {}

      return Result.ok({
        tasks: [],
        filteredCount: 0,
        totalCount: 0,
      });
    } catch (error) {
      return Result.error({
        message: `Error while fetching tasks`,
        source: "nuxtTasksRepository.getTasks",
        originalErrorObject: error as Error,
      });
    }
  },
};
