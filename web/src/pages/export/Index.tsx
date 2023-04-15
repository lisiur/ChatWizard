import { computed, defineComponent, nextTick, ref } from "vue";
import { useRoute } from "vue-router";
import { useAsyncData } from "../../hooks/asyncData";
import { ChatLog, getChat } from "../../api";
import { Chat } from "../../models/chat";
import { NCheckbox, NScrollbar, NSpin } from "naive-ui";
import { useAutoScroll, useScroll } from "../../hooks/scroll";
import { useLazyLoad } from "../../hooks/lazyLoad";
import { AssistantMessage, Message, UserMessage } from "../../models/message";
import mdRender from "../../utils/mdRender";

export default defineComponent({
  setup() {
    const route = useRoute();

    const chatId = route.params.chatId as string;

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

    function renderMessage(msg: Message) {
      if (msg instanceof UserMessage) {
        return (
          <div
            key={msg.id}
            class="flex justify-end items-start pr-4 pl-24 pb-4 group relative"
          >
            <NCheckbox>
              <div
                class="inline-block py-2 px-3 mr-1 rounded-l-xl rounded-t-xl"
                style="background-color: var(--user-msg-bg-color); color: var(--user-msg-color)"
              >
                <div class="break-words whitespace-pre-line">{msg.content}</div>
              </div>
            </NCheckbox>
          </div>
        );
      } else if (msg instanceof AssistantMessage) {
        const content = msg.content;
        const html = mdRender(content);
        return (
          <div key={msg.id} class="flex">
            <NCheckbox>
              <div
                class="markdown-root inline-block px-3 py-2 ml-2 rounded-t-xl rounded-r-xl z-1"
                style="background-color: var(--assistant-msg-bg-color); color: var(--assistant-msg-color)"
                v-html={html}
              ></div>
            </NCheckbox>
          </div>
        );
      }
    }

    return () => (
      <div>
        <div>
          <NScrollbar ref={scrollRef} class="py-4">
            <div class="relative">
              <div
                ref={loadingRef}
                class="absolute top-1 left-1/2 translate-x-[-50%] flex justify-center items-center"
              >
                <NSpin size={12} v-show={hasMore.value}></NSpin>
              </div>
              {chat.value.messages.map((message) => (
                <div key={message.tmpId || message.id}>
                  {renderMessage(message)}{" "}
                </div>
              ))}
            </div>
          </NScrollbar>
        </div>
        <div></div>
      </div>
    );
  },
});
