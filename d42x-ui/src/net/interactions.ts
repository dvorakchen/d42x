import { serverApi } from "./http";

export type Interaction = {
  id: string;
  likes: number;
  unlikes: number;
};

export async function getInteractions(ids: string[]): Promise<Interaction[]> {
  const resp = await serverApi.post("memes/interactions", ids);
  if (resp.status === 200) {
    return resp.data;
  }

  return [];
}
