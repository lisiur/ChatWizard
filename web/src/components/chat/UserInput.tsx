import { defineComponent, nextTick, onMounted, PropType, Ref, ref } from "vue";
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
import CommandPanel from "./CommandPanel";
import Export from "./Export";
import { useInput } from "../../hooks/input";

function useHistoryNavigation(chat: Chat) {
  let id = null as string | null;
  const stack = [] as Message[];

  async function getPrevious() {
    let msg = await chat.getPreviousUserLog(id ?? undefined);
    while (stack.find((item) => item.content === msg?.content)) {
      msg = await chat.getPreviousUserLog(id ?? undefined);
      if (msg) {
        id = msg.id;
      } else {
        return;
      }
    }
    if (msg) {
      stack.push(msg);
      return msg;
    }
  }

  function getNext() {
    stack.pop();
    if (stack.length) {
      const msg = stack[stack.length - 1];
      id = msg.id;
      return msg;
    } else {
      id = null;
    }
  }

  function reset() {
    id = null;
    stack.length = 0;
  }

  return {
    id: id,
    stack: stack,
    getPrevious,
    getNext,
    reset,
  };
}

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

    const { fuzzySearchPrompts } = usePromptService();
    const filteredPrompts = ref<Array<PromptIndex>>([]);
    const selectedPromptIndex = ref(0);

    const {
      getPrevious: getPrevHistory,
      getNext: getNextHistory,
      reset: resetHistory,
    } = useHistoryNavigation(props.chat);

    const {
      input: userMessage,
      state,
      setState,
      focus,
    } = useInput<"normal" | "command" | "historyNavigation">({
      dom: inputRef,
      defaultState: "normal",
      async stateTransition(state, input) {
        switch (state) {
          case "normal": {
            if (input.key === "/" && userMessage.value === "") {
              return "command";
            } else if (["ArrowUp", "ArrowDown"].includes(input.key)) {
              handleHistoryKey(input.event);
              return "historyNavigation";
            } else if (input.key === "Tab") {
              // Expand tab to 4 spaces
              input.event.preventDefault();
              const start = inputRef.value?.selectionStart;
              const end = inputRef.value?.selectionEnd;
              if (start !== undefined && end !== undefined) {
                setUserMessage(
                  userMessage.value.substring(0, start) +
                    "  " +
                    userMessage.value.substring(end)
                );
                nextTick(() => {
                  inputRef.value?.setSelectionRange(start + 4, start + 4);
                });
              }
            } else if (input.key === "Backspace") {
              if (userMessage.value.startsWith("/")) {
                return "command";
              }
            } else if (
              input.key === "Enter" &&
              !input.ctrl &&
              !input.alt &&
              !input.shift &&
              !input.composition
            ) {
              if (!userMessage.value) {
                return;
              }

              // Check if the reply is finished
              if (props.chat.busy) {
                message.warning(t("chat.busy"));
                input.event.preventDefault();
                return;
              }

              props.onMessage?.(new UserMessage(userMessage.value));
              props.sendMessage(userMessage.value);
              setUserMessage("");

              input.event.preventDefault();
            }
            return;
          }
          case "command": {
            if (input.key === "Escape") {
              return "normal";
            } else if (input.key === "ArrowUp") {
              selectedPromptIndex.value = Math.max(
                0,
                selectedPromptIndex.value - 1
              );
              input.event.preventDefault();
            } else if (input.key === "ArrowDown") {
              selectedPromptIndex.value = Math.min(
                filteredPrompts.value.length - 1,
                selectedPromptIndex.value + 1
              );
              input.event.preventDefault();
            } else if (input.key === "Enter") {
              if (filteredPrompts.value.length > 0) {
                const prompt =
                  filteredPrompts.value[selectedPromptIndex.value]!;
                props.chat.changePrompt(prompt.id);
                message.success(
                  t("chat.prompt.changed", {
                    name: prompt.name,
                  })
                );
                userMessage.value = "";
                input.event.preventDefault();
                return "normal";
              }
            }
            return;
          }
          case "historyNavigation": {
            if (!["ArrowUp", "ArrowDown"].includes(input.key)) {
              resetHistory();
              return "normal";
            } else {
              handleHistoryKey(input.event);
            }
            return;
          }
          default: {
            return "normal";
          }
        }
      },
      inputWatcher(input) {
        console.log(state.value);
        if (!input) {
          setState("normal");
        } else if (state.value === "command") {
          filteredPrompts.value =
            fuzzySearchPrompts(userMessage.value.slice(1)) ?? [];
          selectedPromptIndex.value = 0;

          if (filteredPrompts.value.length === 0) {
            setState("normal");
          }
        }
      },
    });

    const publicInstance = {
      focus,
    };
    expose(publicInstance);

    onMounted(focus);

    async function handleHistoryKey(event: KeyboardEvent) {
      if (event.key === "ArrowUp") {
        const msg = await getPrevHistory();
        if (msg) {
          setUserMessage(msg.content);
        }
      } else if (event.key === "ArrowDown") {
        const msg = getNextHistory();
        if (msg) {
          setUserMessage(msg.content);
        } else {
          setUserMessage("");
        }
      }
      event.preventDefault();
    }

    function setUserMessage(content: string) {
      userMessage.value = content;
      nextTick(() => {
        resizeInputHeight();
      });
    }

    function resizeInputHeight() {
      autoGrowTextarea(inputRef.value as HTMLTextAreaElement, {
        minHeight: 100,
      });
    }

    return (() => (
      <div
        class="border-t flex flex-col"
        style="border-color: var(--border-color)"
      >
        <div class="flex items-center h-8">
          <Cost class="pl-2 text-xs" value={props.chat.index.cost}></Cost>
          <Backtrack class="ml-2" chat={props.chat}></Backtrack>
          <div class="flex-1 flex justify-end py-1 px-2">
            <Export chat={props.chat}></Export>
          </div>
        </div>
        <div class="h-[8rem] px-4 pt-2 pb-6 relative">
          <CommandPanel
            v-show={
              state.value === "command" && filteredPrompts.value.length > 0
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
              onInput={resizeInputHeight}
              onFocus={resizeInputHeight}
            ></textarea>
          </NScrollbar>
        </div>
      </div>
    )) as unknown as typeof publicInstance;
  },
});
