/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_AES_KEY: string;
  readonly VITE_AES_IV: string;
  readonly VITE_NET_BASE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
