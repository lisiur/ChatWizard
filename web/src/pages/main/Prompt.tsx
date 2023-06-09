import { computed, defineComponent, ref, watch } from "vue";
import * as api from "../../api";
import { message, prompt } from "../../utils/prompt";
import { Plus as PlusIcon } from "@vicons/fa";
import { NIcon, NScrollbar } from "naive-ui";
import { useRouter } from "vue-router";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { autoGrowTextarea } from "../../utils/autoGrowTextarea";
import { usePromptService } from "../../services/prompt";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const router = useRouter();

    const { prompts, reload } = usePromptService();

    const promptRef = ref<HTMLInputElement>();

    const explorerList = computed(() => {
      return (prompts.value ?? []).map((m) => ({
        id: m.id,
        title: m.name,
        data: m,
      }));
    });

    const promptsMap = new Map<string, api.PromptData>();

    const currentId = ref<string>();

    const currentPromptIndex = computed(() =>
      prompts.value?.find((m) => m.id === currentId.value)
    );
    const currentPromptData = ref<api.PromptData>();
    watch(
      currentId,
      (value) => {
        if (!value) {
          currentPromptData.value = undefined;
          return;
        }

        if (promptsMap.has(value)) {
          currentPromptData.value = promptsMap.get(value);
          return;
        }
      },
      {
        immediate: true,
      }
    );
    const currentPromptInitialContent = ref<string>();

    async function createPrompt() {
      prompt(t("prompt.inputNameHint"), {
        async okHandler(title) {
          const id = await api.createPrompt({
            name: title,
            content: "",
          });
          await reload();
          selectHandler(id)

          setTimeout(() => {
            promptRef.value?.focus();
          });
        },
      });
    }

    async function explorerHandler(action: string, item: ExplorerItem) {
      switch (action) {
        case "delete": {
          await deleteHandler(item.id);
          return;
        }
        case "select": {
          await selectHandler(item.id);
          return;
        }
        case "newChat": {
          await newChatHandler(item.id, item.title);
          return;
        }
        case "rename": {
          await renameHandler(item.id, item.title);
          return;
        }
      }
    }

    async function renameHandler(id: string, act: string) {
      prompt(t("prompt.inputNameHint"), {
        defaultValue: act,
        async okHandler(title) {
          await api.updatePrompt({
            id: id,
            name: title,
          });
          reload();
        },
      });
    }

    async function newChatHandler(id: string, act: string) {
      const chatId = await api.newChat({
        promptId: id,
        title: act,
      });
      router.push({
        name: "chat",
        query: {
          id: chatId,
        },
      });
    }

    async function updateHandler() {
      if (!currentId.value) {
        return;
      }

      if (
        currentPromptInitialContent.value === currentPromptData.value?.content
      ) {
        return;
      }

      await api.updatePrompt({
        id: currentPromptIndex.value!.id,
        content: currentPromptData.value?.content ?? "",
      });
      currentPromptInitialContent.value =
        currentPromptData.value?.content ?? "";

      message.success(t("prompt.update.success"));
    }

    async function deleteHandler(id: string) {
      if (currentPromptIndex.value?.id === id) {
        currentId.value = undefined;
        currentPromptInitialContent.value = undefined;
      }
      await api.deletePrompt(id);
      promptsMap.delete(id);
      reload();
    }

    async function selectHandler(id: string) {
      const promptData = await api.loadPrompt(id);
      promptsMap.set(id, promptData);
      currentId.value = id;
      currentPromptInitialContent.value = promptData.content;

      const promptMetaData = prompts.value?.find((m) => m.id === id)!;
      currentId.value = promptMetaData.id;

      setTimeout(() => {
        promptRef.value?.focus();
      });
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div
            class="h-10 border-b flex justify-center m-2 mt-3 items-center bg-primary cursor-pointer"
            style="color: var(--base-color);border-color: var(--border-color)"
            onClick={createPrompt}
          >
            <NIcon class="mr-1">
              <PlusIcon />
            </NIcon>
            <span> {t("prompt.new")} </span>
          </div>
          <div class="p-2 text-gray-400">{t("prompt.prompts")}</div>
          <Explorer
            class="flex-1 overflow-auto"
            active={currentPromptIndex.value?.id}
            menus={[
              {
                label: t("prompt.newChat"),
                key: "newChat",
              },
              {
                label: t("prompt.rename"),
                key: "rename",
              },
              {
                type: "divider",
              },
              {
                label: t("common.delete"),
                key: "delete",
              },
            ]}
            unstickList={explorerList.value}
            onAction={explorerHandler}
          ></Explorer>
        </div>
        <div class="flex-1 overflow-hidden flex flex-col">
          {currentPromptIndex.value ? (
            <DragBar title={currentPromptIndex.value?.name}></DragBar>
          ) : null}
          <div
            class="flex-1 overflow-hidden p-4"
            style="background-color: var(--body-color)"
          >
            {currentPromptData.value ? (
              <NScrollbar class="h-full">
                <textarea
                  ref={promptRef}
                  v-model={currentPromptData.value.content}
                  class="p-4 h-full resize-none w-full rounded-lg outline-none placeholder-slate-500"
                  style="color: var(--input-msg-color); background-color: var(--input-bg-color)"
                  onFocusout={updateHandler}
                  onInput={(e) =>
                    autoGrowTextarea(e.target as HTMLTextAreaElement)
                  }
                  onFocus={(e) =>
                    autoGrowTextarea(e.target as HTMLTextAreaElement)
                  }
                ></textarea>
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
