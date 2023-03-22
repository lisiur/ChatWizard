import { computed, defineComponent, ref, watch } from "vue";
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

    const promptIndexList = ref<Array<api.PromptIndex>>([]);
    const prompts = new Map<string, [api.PromptMetadata, api.PromptData]>();

    const currentId = ref<string>();

    const currentPromptIndex = computed(() =>
      promptIndexList.value.find((m) => m.id === currentId.value)
    );
    const currentPromptContent = ref<string>()
    watch(currentPromptIndex, (value) => {
      if (!value) {
        currentPromptContent.value = "";
        return
      }

      if (prompts.has(value.id)) {
        currentPromptContent.value = prompts.get(value.id)![1].prompt;
        return;
      }
    }, {
      immediate: true,
    })
    const currentPromptInitialContent = ref<string>();
    refreshMetaList();

    function refreshMetaList() {
      api.allPrompts().then((list) => {
        promptIndexList.value = list;
      });
    }

    async function createPrompt() {
      prompt(t("prompt.inputNameHint"), {
        async okHandler(title) {
          const id = await api.createPrompt({
            act: title,
            prompt: "",
          });
          refreshMetaList();
          currentId.value = id;
          currentPromptInitialContent.value = "";

          setTimeout(() => {
            promptRef.value?.focus();
          });
        },
      });
    }

    async function explorerHandler(
      action: "delete" | "select" | "newChat" | "rename",
      prompt: api.PromptIndex
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
          return;
        }
        case "rename": {
          await renameHandler(prompt);
          return;
        }
      }
    }

    async function renameHandler(data: api.PromptIndex) {
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

    async function newChatHandler(prompt: api.PromptIndex) {
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
      if (!currentId.value) {
        return;
      }

      if (currentPromptInitialContent.value === currentPromptContent.value) {
        return;
      }

      await api.updatePrompt({
        id: currentPromptIndex.value!.id,
        prompt: currentPromptContent.value ?? "",
      });
      currentPromptInitialContent.value = currentPromptContent.value;

      message.success(t("prompt.update.success"));
    }

    async function deleteHandler(prompt: api.PromptIndex) {
      if (currentPromptIndex.value?.id === prompt.id) {
        currentId.value = undefined;
        currentPromptInitialContent.value = undefined;
      }
      await api.deletePrompt(prompt.id);
      prompts.delete(prompt.id);
      refreshMetaList();
    }

    async function selectHandler(prompt: api.PromptIndex) {
      const [metadata, data] = await api.loadPrompt(prompt.id);
      prompts.set(prompt.id, [metadata, data]);
      currentId.value = prompt.id;
      currentPromptInitialContent.value = data.prompt;

      const promptMetaData = promptIndexList.value.find(
        (m) => m.id === prompt.id
      )!;
      currentId.value = promptMetaData.id;

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
            active={currentPromptIndex.value?.id}
            list={promptIndexList.value}
            onAction={explorerHandler}
          ></PromptExplorer>
        </div>
        <div
          class="flex-1 overflow-hidden p-4"
          style="background-color: var(--body-color)"
        >
          {currentId.value ? (
            <NScrollbar class="h-full">
              <textarea
                ref={promptRef}
                v-model={currentPromptContent.value}
                class="p-4 h-full resize-none w-full rounded-lg outline-none placeholder-slate-500"
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
