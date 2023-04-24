import { computed, defineComponent, ref } from "vue";
import * as api from "../../api";
import { NButton, NScrollbar, NSpace, NSpin } from "naive-ui";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { useRouter } from "vue-router";
import { usePluginService, Plugin } from "../../services/plugin";
import { useTask } from "../../hooks/task";

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
                          Install
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
                          Update
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
                          Uninstall
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
              <NScrollbar class="h-full">
                {currentPlugin.value?.description}
              </NScrollbar>
            ) : (
              <div class="h-full" data-tauri-drag-region></div>
            )}
          </div>
        </div>
      </div>
    );
  },
});
