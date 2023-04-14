import {
  defineComponent,
  nextTick,
  onMounted,
  PropType,
  ref,
  watch,
} from "vue";
import { useComposition } from "../../hooks/composition";
import { useI18n } from "../../hooks/i18n";
import { Chat } from "../../models/chat";
import { Message, UserMessage } from "../../models/message";
import { message } from "../../utils/prompt";
import Backtrack from "./Backtrack";
import Cost from "./Cost";
import { NScrollbar } from "naive-ui";
import { autoGrowTextarea } from "../../utils/autoGrowTextarea";
import { usePromptService } from "../../services/prompt";
import { PromptIndex } from "../../api";
import CommandPanel from "./commandPanel";
import Export from "./Export";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    sendMessage: {
      type: Function as PropType<(message: string) => void>,
      required: true,
    },
    onMessage: {
      type: Function as PropType<(message: Message) => void>,
    },
  },
  setup(props, { expose }) {
    const { t } = useI18n();

    const inputRef = ref<HTMLTextAreaElement>();
    const { isComposition } = useComposition(inputRef);
    const userMessage = ref("");
    const inputStatus = ref<"normal" | "command" | "historyNavigation">(
      "normal"
    );
    let historyNavigationMessageId = null as string | null;
    const historyNavigationStack = [] as Message[];

    const { fuzzySearchPrompts } = usePromptService();
    const filteredPrompts = ref<Array<PromptIndex>>([]);
    const selectedPromptIndex = ref(0);

    const publicInstance = {
      focus,
    };
    expose(publicInstance);

    onMounted(() => {
      inputRef.value?.focus();
    });

    function setUserMessage(content: string) {
      userMessage.value = content;
      nextTick(() => {
        resizeInputHeight();
      });
    }

    function focus() {
      inputRef.value?.focus();
    }

    function resizeInputHeight() {
      autoGrowTextarea(inputRef.value as HTMLTextAreaElement, {
        minHeight: 100,
      });
    }

    watch(userMessage, (msg) => {
      if (!msg) {
        inputStatus.value = "normal";
        filteredPrompts.value = [];
      } else if (inputStatus.value === "command") {
        filteredPrompts.value = fuzzySearchPrompts(userMessage.value.slice(1));
        selectedPromptIndex.value = 0;

        if (filteredPrompts.value.length === 0) {
          inputStatus.value = "normal";
        }
      }
    });

    async function keydownHandler(e: KeyboardEvent) {
      if (inputStatus.value === "normal") {
        if (e.key === "/" && userMessage.value === "") {
          inputStatus.value = "command";
          return;
        } else if (["ArrowUp", "ArrowDown"].includes(e.key)) {
          inputStatus.value = "historyNavigation";
          historyNavigationStack.length = 0;
        } else if (e.key === "Tab") {
          // Expand tab to 4 spaces
          e.preventDefault();
          const start = inputRef.value?.selectionStart;
          const end = inputRef.value?.selectionEnd;
          if (start !== undefined && end !== undefined) {
            userMessage.value =
              userMessage.value.substring(0, start) +
              "  " +
              userMessage.value.substring(end);
            nextTick(() => {
              inputRef.value?.setSelectionRange(start + 4, start + 4);
            });
          }
        } else if (
          e.key === "Enter" &&
          !e.ctrlKey &&
          !e.altKey &&
          !e.shiftKey &&
          !isComposition.value
        ) {
          if (!userMessage.value) {
            return;
          }

          // Check if the reply is finished
          if (props.chat.busy) {
            message.warning(t("chat.busy"));
            e.preventDefault();
            return;
          }

          props.onMessage?.(new UserMessage(userMessage.value));
          props.sendMessage(userMessage.value);
          userMessage.value = "";

          e.preventDefault();
        }
      }

      if (inputStatus.value === "command") {
        if (e.key === "Escape") {
          inputStatus.value = "normal";
        } else if (e.key === "ArrowUp") {
          selectedPromptIndex.value = Math.max(
            0,
            selectedPromptIndex.value - 1
          );
          e.preventDefault();
          return;
        } else if (e.key === "ArrowDown") {
          selectedPromptIndex.value = Math.min(
            filteredPrompts.value.length - 1,
            selectedPromptIndex.value + 1
          );
          e.preventDefault();
          return;
        } else if (e.key === "Enter") {
          if (filteredPrompts.value.length > 0) {
            inputStatus.value = "normal";
            const prompt = filteredPrompts.value[selectedPromptIndex.value]!;
            props.chat.changePrompt(prompt.id);
            message.success(
              t("chat.prompt.changed", {
                name: prompt.name,
              })
            );
            userMessage.value = "";
            e.preventDefault();
            return;
          }
        }
      }

      if (inputStatus.value === "historyNavigation") {
        if (!["ArrowUp", "ArrowDown"].includes(e.key)) {
          inputStatus.value = "normal";
          historyNavigationMessageId = null;
          historyNavigationStack.length = 0;
        } else if (e.key === "ArrowUp") {
          let msg = await props.chat.getPreviousUserLog(
            historyNavigationMessageId ?? undefined
          );
          while (
            historyNavigationStack.find((item) => item.content === msg?.content)
          ) {
            msg = await props.chat.getPreviousUserLog(
              historyNavigationMessageId ?? undefined
            );
            if (msg) {
              historyNavigationMessageId = msg.id;
            } else {
              break;
            }
          }
          if (msg) {
            setUserMessage(msg.content);
            historyNavigationStack.push(msg);
          }
          e.preventDefault();
        } else if (e.key === "ArrowDown") {
          historyNavigationStack.pop();
          if (historyNavigationStack.length) {
            const msg =
              historyNavigationStack[historyNavigationStack.length - 1];
            historyNavigationMessageId = msg.id;
            setUserMessage(msg.content);
          } else {
            historyNavigationMessageId = null;
            setUserMessage("");
          }
          e.preventDefault();
        }
      }
    }

    return (() => (
      <div
        class="border-t flex flex-col"
        style="border-color: var(--border-color)"
      >
        <div class="flex items-center h-8">
          <Cost class="pl-2 text-xs" value={props.chat.index.cost}></Cost>
          <Backtrack class="ml-2" chat={props.chat}></Backtrack>
          <div class="flex-1 flex justify-end p-1">
            <Export chat={props.chat}></Export>
          </div>
        </div>
        <div class="h-[8rem] px-4 pt-2 pb-6 relative">
          <CommandPanel
            v-show={
              inputStatus.value === "command" &&
              filteredPrompts.value.length > 0
            }
            list={filteredPrompts.value.map((item) => ({
              label: item.name,
              value: item.name,
            }))}
            selected={selectedPromptIndex.value}
            class="absolute left-4 top-0 translate-y-[-100%]"
          ></CommandPanel>
          <NScrollbar class="h-full">
            <textarea
              ref={inputRef}
              v-model={userMessage.value}
              class="flex-1 resize-none w-full bg-transparent outline-none placeholder-slate-500"
              style="color: var(--input-msg-color)"
              rows="6"
              onKeydown={keydownHandler}
              onInput={resizeInputHeight}
              onFocus={resizeInputHeight}
            ></textarea>
          </NScrollbar>
        </div>
      </div>
    )) as unknown as typeof publicInstance;
  },
});
