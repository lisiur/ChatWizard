import { NButton, NScrollbar } from "naive-ui";
import {
  computed,
  defineComponent,
  nextTick,
  onBeforeUnmount,
  onMounted,
  PropType,
  ref,
  watch,
} from "vue";
import { useI18n } from "../../hooks/i18n";
import { useAutoScroll } from "../../hooks/scroll";
import {
  AssistantMessage,
  ErrorMessage,
  Message,
  UserMessage,
} from "../../models/message";
import { writeToClipboard } from "../../utils/clipboard";
import { interceptLink } from "../../utils/interceptLink";
import mdRender from "../../utils/mdRender";
import { message } from "../../utils/prompt";

export default defineComponent({
  props: {
    messages: {
      type: Array as PropType<Message[]>,
      default: () => [],
    },
    resendMessage: {
      type: Function as PropType<(messageId: string) => void>,
    },
  },
  setup(props, { expose }) {
    const { t } = useI18n();

    const scrollRef = ref<InstanceType<typeof NScrollbar>>();
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
      () => props.messages,
      (messages) => {
        nextTick(() => {
          for (let i = messages.length - 1; i >= 0; i--) {
            let message = messages[i];
            if (message instanceof AssistantMessage) {
              const dom = document.querySelector(
                `#assistant-${message.id}`
              ) as HTMLElement;
              if (dom && dom.dataset.intercepted) {
                break;
              }
              watch(
                () => (message as AssistantMessage).done,
                () => {
                  nextTick(() => {
                    interceptLink(dom);
                    dom.dataset.intercepted = "true";
                  });
                },
                {
                  immediate: true,
                }
              );
            }
          }
        });
      },
      {
        immediate: true,
      }
    );

    watch(
      () => props.messages,
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

    const publicInstance = {
      startAutoScroll,
      stopAutoScroll,
    };

    expose(publicInstance);

    function renderMessage(message: Message) {
      if (message instanceof AssistantMessage) {
        return renderAssistantMessage(message);
      } else if (message instanceof UserMessage) {
        return renderUserMessage(message);
      } else if (message instanceof ErrorMessage) {
        return renderErrorMessage(message);
      }
    }

    function renderAssistantMessage(msg: AssistantMessage) {
      let content = msg.content;
      const codeBlockAssignNum = msg.content.split("```").length - 1;
      if (codeBlockAssignNum % 2 === 1) {
        content += "\n```";
      }
      const html = mdRender(content);
      return (
        <div
          class="flex relative justify-start items-start pl-4 pr-24"
          id={`assistant-${msg.id}`}
        >
          <div class="relative flex-1 overflow-hidden">
            <div
              class="markdown-root inline-block px-3 ml-2 rounded-t-xl rounded-r-xl z-1"
              style="background-color: var(--assistant-msg-bg-color); color: var(--assistant-msg-color)"
              v-html={html}
            ></div>
          </div>
          {msg.done ? (
            <div class="absolute bottom-[-1.2rem] left-4 text-xs">
              <NButton
                type="default"
                text
                size="tiny"
                class="ml-2 text-gray-500"
                onClick={async () => {
                  await writeToClipboard(msg.content);
                  message.success(t("common.copy.success"));
                }}
              >
                {t("common.copy")}
              </NButton>
            </div>
          ) : null}
        </div>
      );
    }

    function renderUserMessage(message: UserMessage) {
      return (
        <div class="flex justify-end items-start pr-4 pl-24">
          <div class="relative">
            <div
              class="inline-block py-2 px-3 mr-1 rounded-l-xl rounded-t-xl"
              style="background-color: var(--user-msg-bg-color); color: var(--user-msg-color)"
            >
              <pre class="break-words whitespace-pre-line">
                {message.content}
              </pre>
            </div>
            <div class="absolute bottom-[-1.2rem] right-1 text-xs">
              {(() => {
                if (message.finished === false) {
                  return (
                    <NButton
                      type="error"
                      text
                      size="tiny"
                      class="mr-2"
                      onClick={() => props.resendMessage?.(message.id)}
                    >
                      resend
                    </NButton>
                  );
                }
              })()}
            </div>
          </div>
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
                  const error = message.error.error;
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
                  const error = message.error.error;
                  return error.message;
                }
              }
            })()}
          </div>
        </div>
      );
    }

    return (() => (
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-hidden">
          <NScrollbar ref={scrollRef} class="py-4">
            <div class="grid gap-6 pb-6">
              {props.messages.map((message, index) => (
                <div key={index}>{renderMessage(message)} </div>
              ))}
            </div>
          </NScrollbar>
        </div>
      </div>
    )) as unknown as typeof publicInstance;
  },
});
