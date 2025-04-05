import { serverApi } from "./http";
import type { CategoryModel } from "./models";

export async function getCategoryList(): Promise<CategoryModel[]> {
  const resp = await serverApi.get("categories");
  const list = resp.data as CategoryModel[];

  return list;
}

export async function categorySuggest(memeId: string, list: string[]) {
  // todo: apply_user_id, needs logged in
  const resp = await serverApi.post("suggests", {
    meme_id: memeId,
    list: list,
    apply_user_id: null,
  });

  if (resp.status !== 200) {
    console.error("create suggestion failed");
  }
}
