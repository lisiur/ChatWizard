import { computed, defineComponent, ref, watch } from "vue";
import * as api from "../../api";
import { NIcon, NScrollbar, NSelect, NSpin } from "naive-ui";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { BagAdd as InstallIcon } from "@vicons/ionicons5";
import { message } from "../../utils/prompt";
import { useTask } from "../../hooks/task";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const repos = ref<Array<api.PromptMarketRepo>>([]);
    const currentRepo = ref<string>();
    const repoOptions = computed(() => {
      return repos.value.map((item) => {
        return {
          value: item.name,
          label: item.name,
        };
      });
    });

    const indexList = ref<Array<api.MarketPromptIndex>>([]);
    const currentId = ref<string>();
    const currentIndex = computed(() =>
      indexList.value.find((m) => m.id === currentId.value)
    );
    const currentPrompt = ref<api.MarketPrompt>();

    const explorerList = computed(() => {
      return indexList.value.map((m) => ({
        id: m.id,
        title: m.act,
      }));
    });

    const loadReposTask = useTask(async () => {
      await api.allRepos().then((list) => {
        repos.value = list;
        if (!currentRepo.value && list.length > 0) {
          currentRepo.value = list[0].name;
        }
      });
    });

    const loadIndexTask = useTask(async () => {
      await api.repoIndexList(currentRepo.value!).then((list) => {
        indexList.value = list;
      });
    });

    const loadDataTask = useTask(async (id: string) => {
      currentPrompt.value = undefined;
      const marketPrompt = await api.loadMarketPrompt(id, currentRepo.value!);
      currentPrompt.value = marketPrompt;
    });

    watch(
      currentRepo,
      () => {
        if (currentRepo.value) {
          loadIndexTask.exec();
        }
      },
      {
        immediate: true,
      }
    );

    loadReposTask.exec();

    function explorerHandler(action: string, item: ExplorerItem) {
      if (action === "select") {
        selectHandler({
          id: item.id,
          act: item.title,
        });
      }
    }

    async function installHandler(prompt: api.MarketPrompt) {
      await api.installPrompt(prompt);
      message.success(t("prompt.market.install.success"));
    }

    async function selectHandler(prompt: api.MarketPromptIndex) {
      currentId.value = prompt.id;
      loadDataTask.exec(prompt.id);
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div
            class="h-10 m-2 mt-3"
            style="color: var(--base-color);border-color: var(--border-color)"
          >
            <NSelect
              v-model:value={currentRepo.value}
              options={repoOptions.value}
            ></NSelect>
          </div>
          <div class="p-2 text-gray-400">{t("prompt.market.prompts")}</div>
          {loadIndexTask.running ? (
            <NSpin class="mt-4"></NSpin>
          ) : (
            <Explorer
              class="flex-1 overflow-auto"
              active={currentIndex.value?.id}
              list={explorerList.value}
              onAction={explorerHandler}
            ></Explorer>
          )}
        </div>
        <div class="flex-1 overflow-hidden flex flex-col">
          {currentPrompt.value ? (
            <DragBar title={currentPrompt.value?.act}>
              {{
                "right-panel": () =>
                  currentPrompt.value ? (
                    <span onClick={() => installHandler(currentPrompt.value!)}>
                      <NIcon size="1.4rem" color="var(--primary-color)">
                        <InstallIcon></InstallIcon>
                      </NIcon>
                    </span>
                  ) : null,
              }}
            </DragBar>
          ) : null}
          <div
            class="flex-1 overflow-hidden p-4"
            style="background-color: var(--body-color)"
          >
            {currentId.value ? (
              loadDataTask.running ? (
                <div class="flex justify-center">
                  <NSpin class="mt-4"></NSpin>
                </div>
              ) : (
                <NScrollbar class="h-full">
                  {currentPrompt.value?.prompt}
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
