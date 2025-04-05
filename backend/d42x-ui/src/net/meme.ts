import { serverApi } from "./http";
import type { MemeEntityModel, PaginatedModel } from "./models";
import placeholderImg from "../assets/placeholder-img.png";
import { AllowMemeFormats } from "../config";

export async function getPaginatedMemeList(page: number, category: string) {
  const resp = await serverApi.get<PaginatedModel<MemeEntityModel>>(
    `memes?page=${page}&category=${category}`
  );
  return resp.data;
}

export async function getMemeDetail(shortId: string): Promise<MemeEntityModel> {
  const resp = await serverApi.get<MemeEntityModel>(`memes/${shortId}`);
  return resp.data;
}

export const skeletonMemeList = [
  {
    id: "0",
    show_date_time: Date(),
    list: [
      {
        id: "00",
        url: placeholderImg,
        format: AllowMemeFormats.PNG,
      },
    ],
  },
  {
    id: "1",
    show_date_time: Date(),
    list: [{ id: "11", url: placeholderImg, format: AllowMemeFormats.PNG }],
  },
  {
    id: "2",
    show_date_time: Date(),
    list: [{ id: "22", url: placeholderImg, format: AllowMemeFormats.PNG }],
  },
] as MemeEntityModel[];
