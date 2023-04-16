import { toRef } from "vue";
import * as api from "../api";
import { useTask } from "../hooks/task";
import { listen } from "../utils/api";

function fuzzyMatch(text: string, keyword: string) {
  const pattern = keyword.split("").join(".*?");
  let regex = new RegExp(pattern, "gi");
  return text.match(regex) !== null;
}

export function usePromptService() {
  const loadAllPromptsTask = useTask(
    async () => {
      return await api.allPrompts();
    },
    {
      default: [],
      autoExecParams: [],
    }
  );

  const prompts = toRef(loadAllPromptsTask, "result");

  listen("prompt-created", () => {
    loadAllPromptsTask.exec();
  });

  listen("prompt-updated", () => {
    loadAllPromptsTask.exec();
  });

  listen("prompt-deleted", () => {
    loadAllPromptsTask.exec();
  });

  function fuzzySearchPrompts(keyword: string) {
    return loadAllPromptsTask.result?.filter((it) => {
      return fuzzyMatch(it.name, keyword) ?? [];
    });
  }

  return {
    loadAllPromptsTask,
    fuzzySearchPrompts,
    prompts,
    reload: loadAllPromptsTask.exec,
  };
}
