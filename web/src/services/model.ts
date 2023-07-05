import { toRef } from "vue";
import * as api from "../api";
import { useTask } from "../hooks/task";

export function useModelService() {
  const loadAllModelsTask = useTask(
    async () => {
      return await api.getChatModels();
    },
    {
      default: [],
      autoExecParams: [],
    }
  );

  const models = toRef(loadAllModelsTask, "result");

  return {
    loadAllModelsTask,
    models,
    reload: loadAllModelsTask.exec,
  };
}
