import {
  NButton,
  NButtonGroup,
  NIcon,
  NInput,
  NPopconfirm,
  NScrollbar,
  NSpace,
  NSpin,
  NTooltip,
} from "naive-ui";
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
import { ChatLog } from "../../api";
import { useI18n } from "../../hooks/i18n";
import { useLazyLoad } from "../../hooks/lazyLoad";
import { useAutoScroll, useScroll } from "../../hooks/scroll";
import { Chat } from "../../models/chat";
import {
  AssistantMessage,
  ErrorMessage,
  Message,
  UserMessage,
} from "../../models/message";
import { interceptLink } from "../../utils/interceptLink";
import mdRender from "../../utils/mdRender";
import { writeToClipboard } from "../../utils/api";
import {
  Clipboard20Regular as CopyIcon,
  ClipboardCheckmark20Regular as CopySuccessIcon,
  Delete20Regular as DeleteIcon,
  Checkmark20Regular as ConfirmIcon,
  Send20Regular as ResendIcon,
  DocumentEdit20Regular as EditIcon,
} from "@vicons/fluent";
import { dialog } from "../../utils/prompt";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    resendMessage: {
      type: Function as PropType<(messageId: string) => void>,
    },
    deleteMessage: {
      type: Function as PropType<(messageId: string) => void>,
    },
    updateMessage: {
      type: Function as PropType<(messageId: string, content: string) => void>,
    },
  },
  setup(props, { expose }) {
    const { t } = useI18n();

    const loadingRef = ref<HTMLElement | null>(null);
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

    const { savePosition, recoverPosition } = useScroll(scrollEle);

    const { hasMore, reset, firstBatchLoad } = useLazyLoad<ChatLog>(
      async (cursor?: string) => {
        const res = await props.chat.loadLogByCursor(cursor);
        savePosition();
        props.chat.addPreviousLogs(res.records);
        await nextTick();
        if (firstBatchLoad.value) {
          scrollToBottom();
        } else {
          recoverPosition();
        }
        return res;
      },
      loadingRef
    );

    watch(() => props.chat, reset);

    const hijackLink = (message: Message) => {
      if (message instanceof AssistantMessage) {
        const dom = document.querySelector(
          `#assistant-${message.id}`
        ) as HTMLElement;
        if (dom && dom.dataset.intercepted) {
          return false;
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
    };
    watch(
      () => props.chat.messages,
      (messages) => {
        setTimeout(() => {
          for (let i = messages.length - 1; i >= 0; i--) {
            let needHijack = hijackLink(messages[i]);
            if (needHijack === false) {
              break;
            }
          }
          for (let i = 0; i < messages.length; i++) {
            let needHijack = hijackLink(messages[i]);
            if (needHijack === false) {
              break;
            }
          }
        }, 200);
      },
      {
        immediate: true,
      }
    );

    watch(
      () => props.chat.messages,
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
          key={msg.id}
          class="relative flex justify-start items-start pl-4 pr-24 pb-4 group"
          id={`assistant-${msg.id}`}
        >
          <div
            class="markdown-root inline-block px-3 py-2 ml-2 rounded-t-xl rounded-r-xl z-1"
            style="background-color: var(--assistant-msg-bg-color); color: var(--assistant-msg-color)"
            v-html={html}
          ></div>
          {msg.done ? (
            <div class="group-hover:block hidden gap-1 absolute bottom-[-.6rem] left-5 text-xs">
              <NButtonGroup>
                {renderCopyMessageButton(msg.content)}
                {renderEditMessageButton(msg)}
                {renderDeleteMessageButton(msg.id)}
              </NButtonGroup>
            </div>
          ) : null}
        </div>
      );
    }

    function renderUserMessage(msg: UserMessage) {
      return (
        <div
          key={msg.id}
          class="flex justify-end items-start pr-4 pl-24 pb-4 group relative hover:shadow-md"
        >
          <div
            class="inline-block py-2 px-3 mr-1 rounded-l-xl rounded-t-xl"
            style="background-color: var(--user-msg-bg-color); color: var(--user-msg-color)"
          >
            <div class="break-words whitespace-pre-line">{msg.content}</div>
          </div>
          <div class="group-hover:block hidden absolute bottom-[-.6rem] right-5 text-xs">
            {renderDeleteMessageButton(msg.id)}
            {renderEditMessageButton(msg)}
            {renderCopyMessageButton(msg.content)}
            {msg.finished === false ? renderResendMessageButton(msg.id) : null}
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
                  return error.message ?? error.type;
                }
              }
            })()}
          </div>
        </div>
      );
    }

    function renderDeleteMessageButton(messageId: string) {
      const needConfirm = ref(false);
      const buttonRef = ref<InstanceType<typeof NButton>>();
      const unwatch = watch(
        buttonRef,
        (button) => {
          if (button) {
            unwatch();
            button.$el.addEventListener("mouseleave", () => {
              setTimeout(() => {
                needConfirm.value = false;
              }, 500);
            });
          }
        },
        {
          immediate: true,
        }
      );
      const clickHandler = () => {
        if (needConfirm.value) {
          props.deleteMessage?.(messageId);
          needConfirm.value = false;
        } else {
          needConfirm.value = true;
        }
      };
      return (
        <NButton
          ref={buttonRef}
          text
          type="error"
          size="tiny"
          onClick={clickHandler}
        >
          <NIcon size="1.2rem">
            {needConfirm.value ? <ConfirmIcon /> : <DeleteIcon />}
          </NIcon>
        </NButton>
      );
    }

    function renderCopyMessageButton(content: string) {
      const success = ref(false);
      const buttonRef = ref<InstanceType<typeof NButton>>();
      const unwatch = watch(
        buttonRef,
        (button) => {
          if (button) {
            unwatch();
            button.$el.addEventListener("mouseleave", () => {
              setTimeout(() => {
                success.value = false;
              }, 500);
            });
          }
        },
        {
          immediate: true,
        }
      );
      return (
        <NTooltip placement="bottom" delay={1000} disabled={true}>
          {{
            trigger: () => (
              <NButton
                ref={buttonRef}
                type={success.value ? "success" : "default"}
                text
                size="tiny"
                class="text-gray-500"
                onClick={async () => {
                  await writeToClipboard(content);
                  success.value = true;
                }}
              >
                <NIcon size="1.2rem">
                  {success.value ? <CopySuccessIcon /> : <CopyIcon />}
                </NIcon>
              </NButton>
            ),
            default: () => t("common.copy"),
          }}
        </NTooltip>
      );
    }

    function editMessageHandler(msg: Message) {
      const value = ref(msg.content);
      const dl = dialog.create({
        style: {
          width: "80%",
          height: "80%",
        },
        closable: false,
        closeOnEsc: false,
        maskClosable: false,
        showIcon: false,
        content() {
          return (
            <NInput
              type="textarea"
              class="h-full"
              v-model:value={value.value}
            ></NInput>
          );
        },
        positiveText: t("common.ok"),
        negativeText: t("common.cancel"),
        onPositiveClick() {
          props.updateMessage?.(msg.id, value.value);
        },
        onNegativeClick() {
          dl.destroy();
        },
      });
    }

    function renderEditMessageButton(msg: Message) {
      return (
        <NButton
          type="default"
          text
          size="tiny"
          class="text-gray-500"
          onClick={() => {
            editMessageHandler(msg);
          }}
        >
          <NIcon size="1.2rem">
            <EditIcon />
          </NIcon>
        </NButton>
      );
    }

    function renderResendMessageButton(id: string) {
      return (
        <NTooltip placement="bottom" delay={500}>
          {{
            trigger: () => (
              <NButton
                type="default"
                text
                size="tiny"
                class="text-gray-500"
                onClick={() => {
                  props.resendMessage?.(id);
                }}
              >
                <NIcon size="1.2rem">
                  <ResendIcon />
                </NIcon>
              </NButton>
            ),
            default: () => t("chat.message.resend"),
          }}
        </NTooltip>
      );
    }

    return (() => (
      <div
        class="flex-1 flex flex-col overflow-hidden"
        style={{
          opacity: firstBatchLoad.value ? 0 : 1,
        }}
      >
        <NScrollbar ref={scrollRef} class="py-4">
          <div class="relative">
            <div
              ref={loadingRef}
              class="absolute top-1 left-1/2 translate-x-[-50%] flex justify-center items-center"
            >
              <NSpin size={12} v-show={hasMore.value}></NSpin>
            </div>
            <div class="grid gap-4 pb-6">
              {props.chat.messages.map((message) => (
                <div key={message.id}>{renderMessage(message)} </div>
              ))}
            </div>
          </div>
        </NScrollbar>
      </div>
    )) as unknown as typeof publicInstance;
  },
});
