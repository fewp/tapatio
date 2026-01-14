import { Result } from "@/core/helpers/result";
import type { TasksRepository } from "@/core/ports/repositories/TasksRepository";
import type { Task } from "~/core/entities/Task";

export const nuxtTasksRepository: TasksRepository = {
  async getTasks({ category, maxQuantity, offset }) {
    try {
      const query = {
        category,
        offset,
        maxQuantity,
      };

      const tasks: Task[] = [];

      return Result.ok(
        tasks || {
          tasks: [],
          filteredCount: 0,
          totalCount: 0,
        }
      );
    } catch (error) {
      return Result.error({
        message: `Error while fetching tasks`,
        source: "nuxtTasksRepository.getTasks",
        originalErrorObject: error as Error,
      });
    }
  },
};
