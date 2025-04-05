import axios, { AxiosHeaders, type AxiosResponse } from "axios";
import { decrypt, encrypt } from "./cipher.ts";

const TEXT_PLAIN = "text/plain;charset=UTF-8";
const APPLICATION_JSON = "application/json;charset=UTF-8";

export const serverApi = axios.create({
  baseURL: import.meta.env.VITE_NET_BASE_URL,
  timeout: 10_000,
  validateStatus: function (status) {
    return status < 500;
  },
});

serverApi.interceptors.request.use(
  function (config) {
    let data = JSON.stringify(config.data);
    if ((data?.length ?? 0) !== 0) {
      config.data = encrypt(data);
    }

    config.headers.setContentType(TEXT_PLAIN, true);

    return config;
  },
  function (error) {
    return Promise.reject(error);
  }
);

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
