import { serverApi } from "./http";
import type { CategoryModel } from "./models";

export async function getCategoryList(): Promise<CategoryModel[]> {
  const resp = await serverApi.get("categories");
  const list = resp.data as CategoryModel[];

  return list;
}
