import type { Task } from "@/core/entities/Task";
import type { TasksRepository } from "@/core/ports/repositories/TasksRepository";
import type { ResultTuple } from "@/core/helpers/result";

import { Result } from "@/core/helpers/result";

type UseCaseDependencies = {
  tasksRepository: TasksRepository;
};

export type GetTasksUseCase = Task[];

type GetTasksUseCaseFunctionParams = {
  category: string;
  maxQuantity?: number;
  offset?: number;
};

export type GetTasksUseCaseReturn = {
  tasks: Task[];
  filteredCount: number;
  totalCount: number;
};

type GetTasksUseCaseFunction = (
  params: GetTasksUseCaseFunctionParams
) => Promise<ResultTuple<GetTasksUseCaseReturn>>;

export function GetTasksUseCase({
  tasksRepository,
}: UseCaseDependencies): GetTasksUseCaseFunction {
  return async ({ category = "", maxQuantity = 9, offset = 0 }) => {
    try {
      const [tasksResult, tasksRepositoryError] =
        await tasksRepository.getTasks({
          category,
          maxQuantity,
          offset,
        });

      if (tasksRepositoryError) {
        return Result.error(tasksRepositoryError);
      }

      if (!tasksResult) {
        return Result.error({
          message: "Error while fetching tasks",
          source: "getTasksUseCase",
          originalErrorObject: null,
        });
      }

      return Result.ok(tasksResult);
    } catch (error) {
      return Result.error({
        message: "Error while fetching tasks",
        source: "getTasksUseCase",
        originalErrorObject: error as Error,
      });
    }
  };
}
