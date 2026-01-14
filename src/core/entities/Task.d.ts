export type TaskCategory = "daily" | "weekly" | "achievement" | "challenge";

export type RewardType = "credits" | "pyroxene" | "eleph";

export type Reward = {
  type: RewardType;
  amount: number;
};

export type Task = {
  id: string;
  category: TaskCategory;
  title: string;
  progress: [number, number]; // [current, total]
  rewards: Reward[];
  updatedAt: string | null;
  deletedAt: string | null;
};
