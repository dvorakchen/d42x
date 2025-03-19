import axios, { AxiosHeaders, type AxiosResponse } from "axios";
import { decrypt } from "./cipher.ts";

const APPLICATION_JSON = "application/json;charset=UTF-8";

export const serverApi = axios.create({
  baseURL: import.meta.env.VITE_NET_BASE_URL,
  timeout: 10_000,
  validateStatus: function (status) {
    return status < 500;
  },
});

serverApi.interceptors.response.use(
  function (response: AxiosResponse<string, any>) {
    let data = response.data;
    if ((data?.length ?? 0) !== 0) {
      response.data = decrypt(data);
      response.data = JSON.parse(response.data);
    }

    (response.headers as AxiosHeaders).setContentType(APPLICATION_JSON, true);

    return response;
  },
  function (error) {
    return Promise.reject(error);
  }
);
