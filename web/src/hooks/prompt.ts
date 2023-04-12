import { Ref, computed, isRef, ref, toRef, watch } from "vue";
import { loadPrompt } from "../api";
import { listen } from "../utils/api";
import { useTask } from "./task";

export function usePrompt(id: string | Ref<string | null>) {
  if (!isRef(id)) {
    id = ref(id);
  }

  const promptId = ref(id.value);
  watch(id, () => {
    promptId.value = (id as Ref<string>).value;
  });

  const loadPromptTask = useTask(async () => {
    if (promptId.value) {
      return loadPrompt(promptId.value!);
    } else {
      return null;
    }
  });

  watch(() => promptId.value, loadPromptTask.exec, {
    immediate: true,
  });

  listen("prompt-updated", (event) => {
    const updatedId = (event.payload as any).id;
    if (updatedId === promptId.value) {
      loadPromptTask.exec();
    }
  });

  return toRef(loadPromptTask, "result");
}
