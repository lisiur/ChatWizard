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

    const promptMetaList = ref<Array<{ act: string }>>([]);
    const prompts = new Map<string, api.Prompt>();
    const currentPrompt = ref<api.Prompt>();
    const currentPromptInitial = ref<string>();
    const currentPromptMeta = ref<{ act: string }>();
    refreshMetaList();

    function refreshMetaList() {
      api.allPrompts().then((list) => {
        promptMetaList.value = list;
      });
    }

    async function createPrompt() {
      prompt(t("prompt.inputNameHint"), {
        async okHandler(title) {
          const prompt: api.Prompt = {
            act: title,
            prompt: "",
          };
          await api.createPrompt(prompt);
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
      action: "delete" | "select" | "newChat",
      act: string
    ) {
      switch (action) {
        case "delete": {
          await deleteHandler(act);
          return;
        }
        case "select": {
          await selectHandler(act);
          return;
        }
        case "newChat": {
          await newChatHandler(act);
        }
      }
    }

    async function newChatHandler(act: string) {
      const chatId = await api.createChat({
        title: act,
        act,
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

    async function deleteHandler(act: string) {
      if (currentPrompt.value?.act === act) {
        currentPrompt.value = undefined;
        currentPromptInitial.value = undefined;
      }
      await api.deletePrompt(act);
      prompts.delete(act);
      refreshMetaList();
    }

    async function selectHandler(act: string) {
      const promptData = await api.loadPrompt(act);
      prompts.set(act, promptData);
      currentPrompt.value = promptData;
      currentPromptInitial.value = promptData.prompt;

      const promptMetaData = promptMetaList.value.find((m) => m.act === act)!;
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
            class="border-b flex justify-center m-2 items-center p-2 bg-primary cursor-pointer"
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
            active={currentPrompt.value?.act}
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
          ) : null}
        </div>
      </div>
    );
  },
});
