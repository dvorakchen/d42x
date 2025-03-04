import { http } from "./http";
import type { MemeEntityModel, PaginatedModel } from "./models";

export async function getPaginatedMemeList(page: number, category: string) {
  const resp = await http.get<PaginatedModel<MemeEntityModel>>(
    `memes?page=${page}&category=${category}`
  );
  return resp.data;
}
