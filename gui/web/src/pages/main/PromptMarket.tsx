import { computed, defineComponent, ref } from "vue";
import * as api from "../../api";
import { NIcon, NScrollbar, NSpin } from "naive-ui";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { BagAdd as InstallIcon } from "@vicons/ionicons5";
import { message } from "../../utils/prompt";
import { useTask } from "../../hooks/task";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const marketPromptIndexList = ref<Array<api.MarketPromptIndex>>([]);

    const currentAct = ref<string>();

    const currentMarketPromptIndex = computed(() =>
      marketPromptIndexList.value.find((m) => m.act === currentAct.value)
    );
    const currentMarketPrompt = ref<api.MarketPrompt>();
    const explorerList = computed(() => {
      return marketPromptIndexList.value.map((m) => ({
        id: m.act,
        title: m.act,
      }));
    });

    const loadListTask = useTask(async () => {
      await api.allMarketPrompts().then((list) => {
        marketPromptIndexList.value = list;
      });
    });

    const loadDataTask = useTask(async (act: string) => {
      currentMarketPrompt.value = undefined;
      const marketPrompt = await api.loadMarketPrompt(act);
      currentMarketPrompt.value = marketPrompt;
    });

    loadListTask.exec();

    function explorerHandler(action: string, item: ExplorerItem) {
      if (action === "select") {
        selectHandler({
          act: item.id,
        });
      }
    }

    async function installHandler(prompt: api.MarketPrompt) {
      await api.installPrompt(prompt);
      message.success(t("prompt.market.install.success"));
    }

    async function selectHandler(prompt: api.MarketPromptIndex) {
      currentAct.value = prompt.act;
      loadDataTask.exec(prompt.act);
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div class="p-2 text-gray-400">{t("prompt.market.prompts")}</div>
          {loadListTask.running ? (
            <NSpin class="mt-4"></NSpin>
          ) : (
            <Explorer
              class="flex-1 overflow-auto"
              active={currentMarketPromptIndex.value?.act}
              list={explorerList.value}
              onAction={explorerHandler}
            ></Explorer>
          )}
        </div>
        <div class="flex-1 overflow-hidden flex flex-col">
          {currentAct.value ? (
            <DragBar title={currentAct.value}>
              {{
                "right-panel": () =>
                  currentMarketPrompt.value ? (
                    <span
                      onClick={() => installHandler(currentMarketPrompt.value!)}
                    >
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
            {currentAct.value ? (
              loadDataTask.running ? (
                <div class="flex justify-center">
                  <NSpin class="mt-4"></NSpin>
                </div>
              ) : (
                <NScrollbar class="h-full">
                  {currentMarketPrompt.value?.prompt}
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
