import { http } from "./http";
import type { MemeEntityModel, PaginatedModel } from "./models";
import placeholderImg from "../assets/placeholder-img.png";

export async function getPaginatedMemeList(page: number, category: string) {
  const resp = await http.get<PaginatedModel<MemeEntityModel>>(
    `memes?page=${page}&category=${category}`
  );
  return resp.data;
}

export const skeletonMemeList = [
  {
    id: "0",
    show_date_time: Date(),
    list: [{ id: "00", url: placeholderImg }],
  },
  {
    id: "1",
    show_date_time: Date(),
    list: [{ id: "11", url: placeholderImg }],
  },
  {
    id: "2",
    show_date_time: Date(),
    list: [{ id: "22", url: placeholderImg }],
  },
] as MemeEntityModel[];
