import { computed, Ref } from "vue";
import { loadPrompt } from "../api";
import { useAsyncData, useAsyncDataReactive } from "./asyncData";

export function usePrompt(id: string | Ref<string | undefined>) {
  if (typeof id === "string") {
    const res = useAsyncData(async () => {
      return loadPrompt(id);
    });
    return {
      prompt: {
        act: computed(() => res.value?.[0].act),
        prompt: computed(() => res.value?.[1].prompt),
      }
    };
  } else {
    const res = useAsyncDataReactive(async () => {
      if (id.value) {
        return loadPrompt(id.value);
      }
    }, [id]);
    return {
      prompt: {
        act: computed(() => res.value?.[0].act),
        prompt: computed(() => res.value?.[1].prompt),
      }
    }
  }
}
