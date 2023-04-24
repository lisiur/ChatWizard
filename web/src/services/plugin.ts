import { computed, toRef } from "vue";
import * as api from "../api";
import { useTask } from "../hooks/task";

export interface Plugin extends api.MarketPlugin {
  id?: string;
  installed: boolean;
  hasNewVersion: boolean;
  installedVersion?: string;
}

export function usePluginService() {
  const loadAllMarketPluginTask = useTask(
    async () => {
      return await api.getAllMarketPlugins();
    },
    {
      default: [],
      autoExecParams: [],
    }
  );

  const loadInstalledPluginsTask = useTask(
    async () => {
      return await api.getAllPlugins();
    },
    {
      default: [],
      autoExecParams: [],
    }
  );

  const marketPlugins = toRef(loadAllMarketPluginTask, "result");
  const installedPlugins = toRef(loadInstalledPluginsTask, "result");

  const plugins = computed<Array<Plugin>>(() => {
    return (
      marketPlugins.value?.map((plugin) => {
        const installedPlugin = installedPlugins.value?.find(
          (p) => p.name === plugin.name
        );
        return {
          ...plugin,
          id: installedPlugin?.id,
          installedVersion: installedPlugin?.version,
          installed: !!installedPlugin,
          hasNewVersion:
            installedPlugin && installedPlugin?.version !== plugin.version,
        } as Plugin;
      }) ?? []
    );
  });

  const loading = computed(() => {
    return loadAllMarketPluginTask.running;
  });

  const error = computed(() => {
    return (
      loadAllMarketPluginTask.error?.message ||
      loadInstalledPluginsTask.error?.message
    );
  });

  function reload() {
    return Promise.all([
      loadAllMarketPluginTask.exec(),
      loadInstalledPluginsTask.exec(),
    ]);
  }

  function reloadInstalled() {
    return loadInstalledPluginsTask.exec();
  }

  return {
    plugins,
    reload,
    loading,
    error,
    reloadInstalled,
  };
}
