import { defineStore } from "pinia";
import { ref } from "vue";

const DEFAULT_DURATION = 3_000;

export type Msg = {
  id?: number;
  color: "success" | "info" | "warning" | "error";
  value: string;
};

export const useMsgStore = defineStore("msg-store", () => {
  const msgs = ref([] as Msg[]);

  function push(msg: Msg): void {
    if (msg.id === undefined) {
      msg.id = Math.random();
    }
    msgs.value.push(msg);

    setTimeout(() => {
      msgs.value = msgs.value.filter((t) => t.id !== msg.id);
    }, DEFAULT_DURATION);
  }

  return {
    msgs,
    push,
  };
});
