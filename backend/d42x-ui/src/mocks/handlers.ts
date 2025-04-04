import { http, HttpResponse } from "msw";
import type {
  CategoryModel,
  MemeEntityModel,
  PaginatedModel,
} from "../net/models";
import { encrypt } from "../net/cipher";
import type { Interaction } from "../net/interactions";
import { AllowMemeFormats } from "../config";

const BASE_URL = import.meta.env.VITE_NET_BASE_URL;
export const handlers = [
  http.get(`${BASE_URL}categories`, () => {
    const mockRes: CategoryModel[] = [
      {
        id: "000",
        name: "meme",
        meme_count: 0,
      },
      {
        id: "001",
        name: "初音未来",
        meme_count: 0,
      },
    ];
    return HttpResponse.text(encrypt(JSON.stringify(mockRes)));
  }),
  http.get(`${BASE_URL}memes`, ({ request }) => {
    const qs = new URLSearchParams(new URL(request.url).searchParams);
    const page = parseInt(qs.get("page") ?? "1");
    const SIZE = 10;
    console.log(page);

    const mockRes: PaginatedModel<MemeEntityModel> = {
      page,
      total: Math.ceil(fakeMemes.length / SIZE),
      list: fakeMemes.slice((page - 1) * SIZE, (page - 1) * SIZE + SIZE),
    };
    return HttpResponse.text(encrypt(JSON.stringify(mockRes)));
  }),
  http.post(`${BASE_URL}memes/interactions`, () => {
    const mockRes: Interaction[] = [
      {
        id: "000",
        likes: 10,
        unlikes: 3,
      },
    ];

    return HttpResponse.text(encrypt(JSON.stringify(mockRes)));
  }),
];

const fakeMemes: MemeEntityModel[] = new Array(20).fill({
  id: "000",
  likes: 10,
  unlikes: 3,
  categories: ["meme", "初音未来"],
  nickname: "dvorak",
  show_date_time: "2025-01-05 23:52",
  list: [
    {
      id: "000",
      cover: "",
      url: "https://wallpaperm.cmcm.com/09108f0d7f193b31b010f4d0a01f4f34.jpg",
      format: AllowMemeFormats.JPG,
      sort: 0,
    },
    {
      id: "000",
      cover: "https://puui.qpic.cn/media_img/0/1732551538010396/0",
      url: "https://pic2.zhimg.com/v2-eb875fa60fe4366b908801e18e34788d_b.webp",
      format: AllowMemeFormats.WEBP,
      sort: 0,
    },
  ],
});
