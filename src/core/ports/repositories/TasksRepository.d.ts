import type { Task } from "@/core/entities/Task";
import type { ResultTuple } from "@/core/helpers/result";
import type { GetTasksUseCaseReturn } from "@/core/useCases/getTasks";

type GetTasksOptions = {
  category: string;
  maxQuantity: number;
  offset: number;
};

export type TasksRepository = {
  getTasks(
    options: GetTasksOptions
  ): Promise<ResultTuple<GetTasksUseCaseReturn>>;
};
