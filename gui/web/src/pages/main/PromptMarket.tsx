import { computed, defineComponent, ref, watch } from "vue";
import * as api from "../../api";
import { NIcon, NScrollbar, NSelect, NSpin } from "naive-ui";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { BagAdd as InstallIcon } from "@vicons/ionicons5";
import { message } from "../../utils/prompt";
import { useTask } from "../../hooks/task";
import { useRouter } from "vue-router";

export default defineComponent({
  setup() {
    const { t } = useI18n();
    const router = useRouter();

    const sources = ref<Array<api.PromptMarketSource>>([]);
    const currentSourceId = ref<string>();
    const sourceOptions = computed(() => {
      return sources.value.map((item) => {
        return {
          value: item.id,
          label: item.name,
        };
      });
    });

    const marketPrompts = ref<Array<api.MarketPrompt>>([]);
    const currentPromptName = ref<string>();
    const currentPrompt = computed(() =>
      marketPrompts.value.find((m) => m.name === currentPromptName.value)
    );

    const explorerList = computed(() => {
      return marketPrompts.value.map((m) => ({
        id: m.name,
        title: m.name,
        data: m,
      }));
    });

    const loadSourcesTask = useTask(async () => {
      await api.getPromptSources().then((list) => {
        sources.value = list;
        if (!currentSourceId.value && list.length > 0) {
          currentSourceId.value = list[0].id;
        }
      });
    });

    const promptsCache = new Map();
    const loadMarketPromptsTask = useTask(async () => {
      const sourceId = currentSourceId.value!;
      if (!promptsCache.has(sourceId)) {
        const list = await api.getPromptSourcePrompts(sourceId);
        promptsCache.set(sourceId, list);
      }
      marketPrompts.value = promptsCache.get(currentSourceId.value!);
    });

    watch(
      currentSourceId,
      () => {
        if (currentSourceId.value) {
          loadMarketPromptsTask.exec();
        }
        currentPromptName.value = undefined;
      },
      {
        immediate: true,
      }
    );

    loadSourcesTask.exec();

    function explorerHandler(action: string, item: ExplorerItem) {
      let prompt = marketPrompts.value.find((m) => m.name === item.id)!;
      switch (action) {
        case "select": {
          selectHandler(prompt);
          break;
        }
        case "install": {
          installHandler(prompt);
          break;
        }
        case "newChat": {
          newChatHandler(prompt);
          break;
        }
      }
    }

    async function installHandler(prompt: api.MarketPrompt) {
      await api.installMarketPrompt(prompt);
      message.success(t("prompt.market.install.success"));
    }

    async function newChatHandler(prompt: api.MarketPrompt) {
      const chatId = await api.installMarketPromptAndCreateChat(prompt);
      router.push({
        name: "chat",
        query: {
          id: chatId,
        },
      });
    }

    async function selectHandler(prompt: api.MarketPrompt) {
      currentPromptName.value = prompt.name;
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
              v-model:value={currentSourceId.value}
              options={sourceOptions.value}
              loading={loadSourcesTask.running}
            ></NSelect>
          </div>
          <div class="p-2 text-gray-400">{t("prompt.market.prompts")}</div>
          {loadMarketPromptsTask.running ? (
            <NSpin class="mt-4"></NSpin>
          ) : (
            <Explorer
              class="flex-1 overflow-auto"
              active={currentPrompt.value?.name}
              menus={[
                {
                  label: t("prompt.market.actions.install"),
                  key: "install",
                },
                {
                  label: t("prompt.market.actions.newChat"),
                  key: "newChat",
                },
              ]}
              unstickList={explorerList.value}
              onAction={explorerHandler}
            ></Explorer>
          )}
        </div>
        <div class="flex-1 overflow-hidden flex flex-col">
          {currentPrompt.value ? (
            <DragBar title={currentPrompt.value?.name}>
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
            {currentPromptName.value ? (
              <NScrollbar class="h-full">
                {currentPrompt.value?.content}
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
