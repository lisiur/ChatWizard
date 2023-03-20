import { defineComponent, ref } from "vue";
import * as api from "../../api";
import PromptExplorer from "../../components/PromptExplorer";
import { message, prompt } from "../../utils/prompt";
import { Plus as PlusIcon } from "@vicons/fa";
import { NIcon, NScrollbar } from "naive-ui";
import { useRouter } from "vue-router";
import { useI18n } from "../../hooks/i18n";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const router = useRouter();

    const promptRef = ref<HTMLInputElement>();

    const promptMetaList = ref<Array<api.PromptMetadata>>([]);
    const prompts = new Map<string, api.Prompt>();
    const currentPrompt = ref<api.Prompt>();
    const currentPromptInitial = ref<string>();
    const currentPromptMeta = ref<api.PromptMetadata>();
    refreshMetaList();

    function refreshMetaList() {
      api.allPrompts().then((list) => {
        promptMetaList.value = list;
      });
    }

    async function createPrompt() {
      prompt(t("prompt.inputNameHint"), {
        async okHandler(title) {
          const id = await api.createPrompt({
            act: title,
            prompt: "",
          });
          const prompt: api.Prompt = {
            id,
            act: title,
            prompt: "",
          };
          refreshMetaList();
          currentPrompt.value = prompt;
          currentPromptInitial.value = "";

          setTimeout(() => {
            promptRef.value?.focus();
          });
        },
      });
    }

    async function explorerHandler(
      action: "delete" | "select" | "newChat" | "rename",
      prompt: api.PromptMetadata
    ) {
      switch (action) {
        case "delete": {
          await deleteHandler(prompt);
          return;
        }
        case "select": {
          await selectHandler(prompt);
          return;
        }
        case "newChat": {
          await newChatHandler(prompt);
        }
        case "rename": {
          await renameHandler(prompt);
        }
      }
    }

    async function renameHandler(data: api.PromptMetadata) {
      prompt(t("prompt.inputNameHint"), {
        defaultValue: data.act,
        async okHandler(title) {
          await api.updatePrompt({
            id: data.id,
            act: title,
          });
          refreshMetaList();
        },
      });
    }

    async function newChatHandler(prompt: api.PromptMetadata) {
      const chatId = await api.createChat({
        promptId: prompt.id,
        title: prompt.act,
      });
      router.push({
        name: "chat",
        query: {
          id: chatId,
        },
      });
    }

    async function updateHandler() {
      if (!currentPrompt.value) {
        return;
      }

      if (currentPromptInitial.value === currentPrompt.value.prompt) {
        return;
      }

      await api.updatePrompt(currentPrompt.value);
      currentPromptInitial.value = currentPrompt.value.prompt;

      message.success("Prompt updated");
    }

    async function deleteHandler(prompt: api.PromptMetadata) {
      if (currentPrompt.value?.id === prompt.id) {
        currentPrompt.value = undefined;
        currentPromptInitial.value = undefined;
      }
      await api.deletePrompt(prompt.id);
      prompts.delete(prompt.id);
      refreshMetaList();
    }

    async function selectHandler(prompt: api.PromptMetadata) {
      const promptData = await api.loadPrompt(prompt.id);
      prompts.set(prompt.id, promptData);
      currentPrompt.value = promptData;
      currentPromptInitial.value = promptData.prompt;

      const promptMetaData = promptMetaList.value.find(
        (m) => m.id === prompt.id
      )!;
      currentPromptMeta.value = promptMetaData;

      setTimeout(() => {
        promptRef.value?.focus();
      });
    }

    function autoGrowHandler(element: HTMLTextAreaElement) {
      element.style.height = "5px";
      element.style.height = element.scrollHeight + "px";
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div
            class="border-b flex justify-center m-2 mt-3 items-center p-2 bg-primary cursor-pointer"
            style="color: var(--base-color);border-color: var(--border-color)"
            onClick={createPrompt}
          >
            <NIcon class="mr-1">
              <PlusIcon />
            </NIcon>
            <span> {t("prompt.new")} </span>
          </div>
          <div class="p-2 text-gray-400">{t("prompt.prompts")}</div>
          <PromptExplorer
            class="flex-1 overflow-auto"
            active={currentPrompt.value?.id}
            list={promptMetaList.value}
            onAction={explorerHandler}
          ></PromptExplorer>
        </div>
        <div
          class="flex-1 overflow-hidden"
          style="background-color: var(--body-color)"
        >
          {currentPrompt.value ? (
            <NScrollbar class="h-full">
              <textarea
                ref={promptRef}
                v-model={currentPrompt.value.prompt}
                class="p-4 h-full resize-none w-full outline-none placeholder-slate-500"
                style="color: var(--input-msg-color); background-color: var(--input-bg-color)"
                onFocusout={updateHandler}
                onInput={(e) =>
                  autoGrowHandler(e.target as HTMLTextAreaElement)
                }
                onFocus={(e) =>
                  autoGrowHandler(e.target as HTMLTextAreaElement)
                }
              ></textarea>
            </NScrollbar>
          ) : (
            <div class="h-full" data-tauri-drag-region></div>
          )}
        </div>
      </div>
    );
  },
});
