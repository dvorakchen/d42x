import type { AllowMemeFormats } from "../config";

export type CategoryModel = {
  id: string;
  name: string;
};

export type PaginatedModel<T> = {
  page: number;
  total: number;
  list: T[];
};

export type MemeEntityModel = {
  id: string;
  likes: number;
  unlikes: number;
  categories: string[];
  nickname: string;
  show_date_time: string;
  list: MemeUrlEntityModel[];
};

export type MemeUrlEntityModel = {
  id: string;
  cover: string;
  url: string;
  format: AllowMemeFormats;
};
