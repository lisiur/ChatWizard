import { computed, defineComponent, ref, watch } from "vue";
import * as api from "../../api";
import { NButton, NScrollbar, NSpace, NSpin } from "naive-ui";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { useRouter } from "vue-router";
import { usePluginService, Plugin } from "../../services/plugin";
import { useTask } from "../../hooks/task";
import { useAsyncDataReactive } from "../../hooks/asyncData";
import mdRender from "../../utils/mdRender";

export default defineComponent({
  setup() {
    const { t } = useI18n();
    const router = useRouter();

    const { plugins, loading, error, reloadInstalled } = usePluginService();
    const installTask = useTask(async (plugin: Plugin) => {
      await api.installPlugin(plugin);
      reloadInstalled();
    });
    const uninstallTask = useTask(async (plugin: Plugin) => {
      await api.uninstallPlugin(plugin.id!);
      reloadInstalled();
    });
    const updateTask = useTask(async (plugin: Plugin) => {
      await api.uninstallPlugin(plugin.id!);
      await api.installPlugin(plugin);
      reloadInstalled();
    });
    const installLoading = computed(() => installTask.running);
    const uninstallLoading = computed(() => uninstallTask.running);
    const updateLoading = computed(() => updateTask.running);

    const currentPluginName = ref<string>();
    const currentPlugin = computed(() =>
      plugins.value?.find((m) => m.name === currentPluginName.value)
    );

    const queryReadmeTask = useTask(async () => {
      if (currentPlugin.value) {
        return await api.getMarketPluginReadme(currentPlugin.value?.readme);
      } else {
        return "";
      }
    });
    watch(currentPluginName, queryReadmeTask.exec, {
      immediate: true,
    });

    const explorerList = computed(() => {
      return (
        plugins.value?.map((m) => ({
          id: m.name,
          title: m.name,
          data: m,
        })) ?? []
      );
    });

    function explorerHandler(action: string, item: ExplorerItem) {
      let plugin = plugins.value?.find((m) => m.name === item.id)!;
      switch (action) {
        case "select": {
          selectHandler(plugin);
          break;
        }
      }
    }

    async function selectHandler(plugin: api.MarketPlugin) {
      currentPluginName.value = plugin.name;
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div class="p-2 text-gray-400">{t("plugin.market.plugins")}</div>
          {loading.value ? (
            <NSpin class="mt-4"></NSpin>
          ) : error.value ? (
            <div class="mt-4 flex justify-center text-error">
              {error.toString()}
            </div>
          ) : (
            <Explorer
              class="flex-1 overflow-auto"
              active={currentPlugin.value?.name}
              unstickList={explorerList.value}
              onAction={explorerHandler}
            ></Explorer>
          )}
        </div>
        <div class="flex-1 overflow-hidden flex flex-col">
          {currentPlugin.value ? (
            <DragBar title={currentPlugin.value?.name}>
              {{
                "right-panel": () =>
                  currentPlugin.value ? (
                    <NSpace>
                      {!currentPlugin.value?.installed ? (
                        <NButton
                          size="tiny"
                          type="primary"
                          secondary
                          loading={installLoading.value}
                          onClick={() => installTask.exec(currentPlugin.value!)}
                        >
                          {t("plugin.market.actions.install")}
                        </NButton>
                      ) : null}
                      {currentPlugin.value?.hasNewVersion ? (
                        <NButton
                          size="tiny"
                          type="primary"
                          secondary
                          loading={updateLoading.value}
                          onClick={() => updateTask.exec(currentPlugin.value!)}
                        >
                          {t("plugin.market.actions.update")}
                        </NButton>
                      ) : null}
                      {currentPlugin.value?.installed ? (
                        <NButton
                          size="tiny"
                          type="error"
                          secondary
                          loading={uninstallLoading.value}
                          onClick={() =>
                            uninstallTask.exec(currentPlugin.value!)
                          }
                        >
                          {t("plugin.market.actions.uninstall")}
                        </NButton>
                      ) : null}
                    </NSpace>
                  ) : null,
              }}
            </DragBar>
          ) : null}
          <div
            class="flex-1 overflow-hidden p-4"
            style="background-color: var(--body-color)"
          >
            {currentPluginName.value ? (
              queryReadmeTask.running ? (
                <div class="flex justify-center">
                  <NSpin></NSpin>
                </div>
              ) : (
                <NScrollbar class="h-full">
                  <div class="markdown-root" v-html={mdRender(queryReadmeTask.result ?? "")}></div>
                </NScrollbar>
              )
            ) : (
              <div class="h-full" data-tauri-drag-region></div>
            )}
          </div>
        </div>
      </div>
    );
  },
});
