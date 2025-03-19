import { defineStore } from "pinia";
import { ref } from "vue";

export type Interaction = {
  like: boolean;
  unlike: boolean;
};

export type MemeId = string;

const STORE_KEY = "KEY_MEME_INTERACTIONS";

export const useInteractStore = defineStore("interaction-store", () => {
  const records = ref(new Map<MemeId, Interaction>());
  records.value = retrieve();

  function getRecord(id: MemeId): Interaction | null {
    const record = records.value.get(id);
    if (record === undefined) {
      return null;
    }

    return record;
  }

  function like(id: MemeId, value: boolean): void {
    let record =
      records.value.get(id) ?? ({ like: false, unlike: false } as Interaction);
    record.like = value;
    records.value.set(id, record);

    save();
  }

  function unlike(id: MemeId, value: boolean): void {
    let record =
      records.value.get(id) ?? ({ like: false, unlike: false } as Interaction);
    record.unlike = value;
    records.value.set(id, record);

    save();
  }

  async function fresh(): Promise<void> {}

  function save(): void {
    const strValue = JSON.stringify(Array.from(records.value));
    localStorage.setItem(STORE_KEY, strValue);
  }

  function retrieve(): Map<MemeId, Interaction> {
    let value: [] = JSON.parse(localStorage.getItem(STORE_KEY) ?? "[]");

    return new Map(value);
  }

  return {
    getRecord,
    like,
    unlike,
    fresh,
  };
});
