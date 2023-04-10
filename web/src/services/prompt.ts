import * as api from "../api";
import { useTask } from "../hooks/task";

function fuzzyMatch(text: string, keyword: string) {
  let regex = new RegExp(keyword, "gi");
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

  function fuzzySearchPrompts(keyword: string) {
    return (
      loadAllPromptsTask.result?.filter((it) => fuzzyMatch(it.name, keyword)) ??
      []
    );
  }

  return {
    loadAllPromptsTask,
    fuzzySearchPrompts,
  };
}
