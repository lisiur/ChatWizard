import { computed, defineComponent, nextTick, ref } from "vue";
import { useRoute } from "vue-router";
import { useAsyncData } from "../../hooks/asyncData";
import { ChatLog, getChat, saveFile } from "../../api";
import { Chat } from "../../models/chat";
import {
  NButton,
  NCheckbox,
  NCheckboxGroup,
  NScrollbar,
  NSpin,
} from "naive-ui";
import { useAutoScroll, useScroll } from "../../hooks/scroll";
import { useLazyLoad } from "../../hooks/lazyLoad";
import { AssistantMessage, Message, UserMessage } from "../../models/message";
import mdRender from "../../utils/mdRender";
import { toBlob } from "html-to-image";
import { useI18n } from "../../hooks/i18n";
import dayjs from "dayjs";
import { isWeb } from "../../utils/env";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const route = useRoute();

    const chatId = route.query.chatId as string;

    const chat = useAsyncData(async () => {
      const chatIndex = await getChat(chatId);
      return new Chat(chatIndex);
    });

    const loadingRef = ref<HTMLElement | null>(null);
    const scrollRef = ref<InstanceType<typeof NScrollbar>>();
    const scrollEle = computed(() => {
      return scrollRef.value?.$el.nextSibling.children[0] as HTMLElement;
    });
    const { scrollToBottom } = useAutoScroll(scrollEle);
    const { savePosition, recoverPosition } = useScroll(scrollEle);
    const { hasMore, reset, firstBatchLoad } = useLazyLoad<ChatLog>(
      async () => {
        savePosition();
        const res = await chat.value.loadPrevLogs();
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
    const selectedIds = ref<Array<string>>([]);
    const selectedMsgs = computed(() => {
      return (chat.value?.messages ?? []).filter((msg) =>
        selectedIds.value.includes(msg.id)
      );
    });

    function renderMessage(msg: Message) {
      const color =
        msg instanceof UserMessage
          ? "var(--user-msg-color)"
          : "var(--assistant-msg-color)";
      const bgColor =
        msg instanceof UserMessage
          ? "var(--user-msg-bg-color)"
          : "var(--assistant-msg-bg-color)";

      return (
        <div class="flex items-center px-4 py-1">
          <NCheckbox value={msg.id}>
            <div
              class="relative top-[-.4rem] px-4 max-h-[2.15rem] overflow-hidden inline-block rounded-t-xl rounded-r-xl"
              style={{
                color,
                backgroundColor: bgColor,
              }}
            >
              {(() => {
                if (msg instanceof UserMessage) {
                  return (
                    <div class="markdown-root">
                      <p>{msg.content}</p>
                    </div>
                  );
                } else if (msg instanceof AssistantMessage) {
                  const content = msg.content;
                  const html = mdRender(content);
                  return <div class="markdown-root" v-html={html}></div>;
                }
              })()}
            </div>
          </NCheckbox>
        </div>
      );
    }

    function renderPreviewMessage(msg: Message) {
      const color =
        msg instanceof UserMessage
          ? "var(--user-msg-color)"
          : "var(--assistant-msg-color)";
      const bgColor =
        msg instanceof UserMessage
          ? "var(--user-msg-bg-color)"
          : "var(--assistant-msg-bg-color)";
      return (
        <div
          class="px-4 inline-block rounded-t-xl rounded-r-xl"
          style={{
            color,
            backgroundColor: bgColor,
          }}
        >
          {(() => {
            if (msg instanceof UserMessage) {
              return (
                <div class="markdown-root">
                  <p>{msg.content}</p>
                </div>
              );
            } else if (msg instanceof AssistantMessage) {
              const html = mdRender(msg.content);
              return <div class="markdown-root" v-html={html}></div>;
            }
          })()}
        </div>
      );
    }

    async function exportHandler() {
      const exportPreview = document.getElementById(
        "export-preview"
      ) as HTMLElement;

      const fileName = `${chat.value?.index.title || "chat"}-${dayjs().format(
        "YYYY-MM-DD_HH-mm-ss"
      )}.png`;

      toBlob(exportPreview).then((blob) => {
        if (blob) {
          if (isWeb) {
            const url = URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = fileName;
            a.click();
            URL.revokeObjectURL(url);
          } else {
            saveFile(fileName, blob);
          }
        }
      });
    }

    return () => (
      <div class="h-full flex">
        {chat.value ? (
          <div class="h-full flex-[2] overflow-hidden">
            <NScrollbar ref={scrollRef} class="py-4">
              <div class="relative">
                <div
                  ref={loadingRef}
                  class="absolute top-1 left-1/2 translate-x-[-50%] flex justify-center items-center"
                >
                  <NSpin size={12} v-show={hasMore.value}></NSpin>
                </div>
                <NCheckboxGroup v-model:value={selectedIds.value}>
                  {chat.value.messages.map((message) => (
                    <div key={message.id}>{renderMessage(message)}</div>
                  ))}
                </NCheckboxGroup>
              </div>
            </NScrollbar>
          </div>
        ) : null}
        <div class="h-full flex flex-col flex-[3] overflow-hidden border-l border-[var(--border-color)]">
          <div class="flex-1 overflow-hidden">
            <NScrollbar>
              <div id="export-preview" class="p-4 bg-[var(--body-color)]">
                {selectedMsgs.value.map((msg) => (
                  <div key={msg.id} class="p-2">
                    {renderPreviewMessage(msg)}
                  </div>
                ))}
              </div>
            </NScrollbar>
          </div>
          <div class="p-2 border-t border-[var(--border-color)] flex justify-end">
            <NButton type="primary" secondary onClick={exportHandler}>
              {t("chat.export")}
            </NButton>
          </div>
        </div>
      </div>
    );
  },
});
