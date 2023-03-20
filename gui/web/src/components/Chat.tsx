import {
  computed,
  defineComponent,
  onBeforeUnmount,
  nextTick,
  PropType,
  ref,
  onMounted,
  watch,
  DefineComponent,
} from "vue";
import mdRender from "../utils/mdRender";
import assistantAvatar from "../assets/assistant_avatar.png";
import userAvatar from "../assets/user_avatar.png";
import {
  AssistantMessage,
  ErrorMessage,
  Message,
  UserMessage,
} from "../models/message";
import { useConfig } from "../hooks/config";
import { NButton, NIcon, NScrollbar, NTooltip } from "naive-ui";
import { writeToClipboard } from "../utils/clipboard";
import { useComposition } from "../hooks/composition";
import { Markdown } from "@vicons/fa";
import { Chat } from "../models/chat";
import { useAutoScroll } from "../hooks/scroll";
import { save } from "@tauri-apps/api/dialog";
import { ChatMetadata, updateChat } from "../api";
import { useI18n } from "../hooks/i18n";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    chatMetaData: {
      type: Object as PropType<ChatMetadata>,
      required: true,
    },
    onMessage: {
      type: Function as PropType<(message: Message) => void>,
    }
  },
  setup(props, { expose }) {
    const { t } = useI18n();

    const scrollRef = ref<InstanceType<typeof NScrollbar>>();
    const inputRef = ref<HTMLTextAreaElement>();
    const { isComposition } = useComposition(inputRef);

    const scrollEle = computed(() => {
      return scrollRef.value?.$el.nextSibling.children[0] as HTMLElement;
    });

    const {
      start: startAutoScroll,
      stop: stopAutoScroll,
      destroy: destroyAutoScroll,
      scrollToBottom,
    } = useAutoScroll(scrollEle);

    watch(
      () => props.chat,
      () => {
        nextTick(scrollToBottom);
      }
    );
    onMounted(() => {
      nextTick(scrollToBottom);
    });
    onBeforeUnmount(() => {
      destroyAutoScroll();
    });

    const { checkApiKey: check_api_key } = useConfig();

    check_api_key();

    const prompt = ref("");

    function keydownHandler(e: KeyboardEvent) {
      if (e.key === "Enter" && !e.ctrlKey && !isComposition.value) {
        const message = prompt.value;

        props.onMessage?.(new UserMessage(message));

        prompt.value = "";
        props.chat.sendMessage(message, {
          onFinish: stopAutoScroll,
        });
        setTimeout(() => {
          startAutoScroll();
        }, 20);
        e.preventDefault();
      }
    }

    async function exportMarkdown() {
      const filePath = await save({
        title: props.chatMetaData.title,
        filters: [
          {
            name: "Markdown",
            extensions: ["md"],
          },
        ],
      });
      if (filePath) {
        props.chat.exportMarkdown(filePath);
      }
    }

    function focusInput() {
      inputRef.value?.focus();
    }

    function renderButton(props: {
      icon: DefineComponent<any, any, any>;
      tooltip: string;
      handler: () => void;
    }) {
      return (
        <NTooltip trigger="hover" delay={500}>
          {{
            trigger: () => (
              <button
                class="bg-transparent rounded px-2 py-1"
                onClick={props.handler}
              >
                <NIcon
                  size="1rem"
                  class="text-[var(--chat-btn-color)] hover:text-[var(--primary-color)]"
                >
                  <props.icon></props.icon>
                </NIcon>
              </button>
            ),
            default: () => props.tooltip,
          }}
        </NTooltip>
      );
    }

    function renderAvatar(avatar: string) {
      return <img src={avatar} class="w-8 h-8 "></img>;
    }

    function renderTriangle(
      direction: "left" | "right",
      style?: {
        color: string;
        size: string;
      }
    ) {
      if (direction === "left") {
        return (
          <div
            class={"border-solid border-y-transparent border-l-0"}
            style={{
              borderRightColor: style?.color,
              borderRightWidth: style?.size,
              borderTopWidth: style?.size,
              borderBottomWidth: style?.size,
            }}
          ></div>
        );
      } else if (direction === "right") {
        return (
          <div
            class={"border-solid border-y-transparent border-r-0"}
            style={{
              borderLeftColor: style?.color,
              borderLeftWidth: style?.size,
              borderTopWidth: style?.size,
              borderBottomWidth: style?.size,
            }}
          ></div>
        );
      }
    }

    function renderMessage(
      message: Message,
      chat: Chat,
      params?: { onFinish?: () => void }
    ) {
      if (message instanceof AssistantMessage) {
        return renderAssistantMessage(message);
      } else if (message instanceof UserMessage) {
        return renderUserMessage(message, chat, params);
      } else if (message instanceof ErrorMessage) {
        return renderErrorMessage(message);
      }
    }

    function renderAssistantMessage(message: AssistantMessage) {
      const html = mdRender(message.content);
      return (
        <div class="flex relative justify-start items-start pl-4 pr-24">
          {renderAvatar(assistantAvatar)}
          <div class="relative ml-2 flex-1 overflow-hidden">
            <div class="absolute left-[-.2rem] top-1">
              {renderTriangle("left", {
                color: "var(--assistant-msg-bg-color)",
                size: ".5rem",
              })}
            </div>
            <div
              class="markdown-root inline-block px-3 ml-1 rounded-md z-1"
              style="background-color: var(--assistant-msg-bg-color); color: var(--assistant-msg-color)"
              v-html={html}
            ></div>
          </div>
          {message.done ? (
            <div class="absolute bottom-[-1.2rem] left-14 text-xs">
              <NButton
                type="default"
                text
                size="tiny"
                class="ml-2 text-gray-500"
                onClick={() => writeToClipboard(message.content)}
              >
                {t("common.copy")}
              </NButton>
            </div>
          ) : null}
        </div>
      );
    }

    function renderUserMessage(
      message: UserMessage,
      chat: Chat,
      params?: { onFinish?: () => void }
    ) {
      return (
        <div class="flex justify-end items-start pr-4 pl-24">
          <div class="relative mr-2">
            <div
              class="inline-block py-2 px-3 mr-1 rounded-md"
              style="background-color: var(--user-msg-bg-color); color: var(--user-msg-color)"
            >
              {message.content}
            </div>
            <div class="absolute right-[-.2rem] top-1">
              {renderTriangle("right", {
                color: "var(--user-msg-bg-color)",
                size: ".5rem",
              })}
            </div>
            <div class="absolute bottom-[-1.1rem] right-0 text-xs">
              {(() => {
                switch (message.delivered) {
                  case null: {
                    break;
                  }
                  case true: {
                    return (
                      <span class="text-gray-600">{t("chat.delivered")}</span>
                    );
                  }
                  case false: {
                    return (
                      <NButton
                        type="error"
                        text
                        size="tiny"
                        class="mr-2"
                        onClick={() => chat.resendMessage(message.id, params)}
                      >
                        resend
                      </NButton>
                    );
                  }
                }
              })()}
            </div>
          </div>
          {renderAvatar(userAvatar)}
        </div>
      );
    }

    function renderErrorMessage(message: ErrorMessage) {
      return (
        <div class="flex justify-center px-16">
          <div class="text-xs text-white bg-red-400 px-4 py-1 rounded">
            {(() => {
              switch (message.error.type) {
                case "network": {
                  const error = message.error.data;
                  switch (error.type) {
                    case "timeout": {
                      return "Timeout";
                    }
                    case "unknown": {
                      return error.message;
                    }
                  }
                }
                case "api": {
                  const error = message.error.data;
                  return error.message;
                }
              }
            })()}
          </div>
        </div>
      );
    }

    const publicExpose = {
      focusInput,
    };
    expose(publicExpose);

    return (() => (
      <div
        class="h-full flex flex-col"
        style="background-color: var(--body-color)"
      >
        {/* history */}
        <div class="flex-1 flex flex-col overflow-hidden">
          <div class="px-4 py-3 border-b border-color" data-tauri-drag-region>
            <span class="text-lg">
              {props.chatMetaData.title || t("chat.new.defaultTitle")}
            </span>
          </div>
          <div class="flex-1 overflow-hidden">
            <NScrollbar ref={scrollRef} class="py-4">
              <div class="grid gap-6">
                {props.chat.messages.map((message, index) => (
                  <div key={index}>
                    {renderMessage(message, props.chat, {
                      onFinish: stopAutoScroll,
                    })}
                  </div>
                ))}
              </div>
            </NScrollbar>
          </div>
        </div>

        {/* input */}
        <div class="border-t" style="border-color: var(--border-color)">
          <div class="flex items-center">
            <div class="flex-1 flex justify-end p-1">
              {renderButton({
                handler: exportMarkdown,
                icon: Markdown,
                tooltip: t("chat.exportMarkdown"),
              })}
            </div>
          </div>
          <textarea
            ref={inputRef}
            v-model={prompt.value}
            class="p-2 resize-none w-full bg-transparent outline-none placeholder-slate-500"
            style="color: var(--input-msg-color)"
            rows="5"
            onKeydown={keydownHandler}
          ></textarea>
        </div>
      </div>
    )) as unknown as typeof publicExpose;
  },
});
