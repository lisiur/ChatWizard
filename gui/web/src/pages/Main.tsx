import { defineComponent, ref } from "vue";
import ChatComp from "../components/Chat";
import ExplorerComp from "../components/Explorer";
import * as api from "../api";
import { Chat } from "../models/chat";
import { AssistantMessage, Message, UserMessage } from "../models/message";
import { prompt } from "../utils/prompt";
import { Plus as PlusIcon } from "@vicons/fa";
import { NIcon } from "naive-ui";
import Version from "../components/Version";

export default defineComponent({
  setup() {
    const chatRef = ref<InstanceType<typeof ChatComp>>();

    const chatMetaList = ref<Array<{ id: string; title: string }>>([]);
    const chats = new Map<string, Chat>();
    const currentChat = ref<Chat>();
    const currentChatMeta = ref<{ id: string; title: string }>();
    refreshChatMetaList();

    function refreshChatMetaList() {
      api.allChats().then((list) => {
        chatMetaList.value = list;
      });
    }

    async function createChat() {
      prompt("Please input chat title:", {
        async okHandler(title) {
          const chatId = await api.createChat({
            title,
          });
          const chat = new Chat(chatId);
          refreshChatMetaList();
          currentChat.value = chat;
          setTimeout(() => {
            chatRef.value?.focusInput();
          });
        },
      });
    }

    async function explorerHandler(
      action: "delete" | "select",
      chatId: string
    ) {
      switch (action) {
        case "delete": {
          await deleteChatHandler(chatId);
          return;
        }
        case "select": {
          await selectChatHandler(chatId);
          return;
        }
      }
    }

    async function deleteChatHandler(chatId: string) {
      if (currentChat.value?.id === chatId) {
        currentChat.value = undefined;
      }
      await api.deleteChat(chatId);
      chats.delete(chatId);
      refreshChatMetaList();
    }

    async function selectChatHandler(chatId: string) {
      const _messages = await api.readChat(chatId);
      const messages = _messages.map((m) => {
        switch (m.message.role) {
          case "user": {
            const msg = new UserMessage(m.message.content);
            msg.setId(m.id);
            msg.markHistory();
            return msg;
          }
          case "assistant": {
            const msg = new AssistantMessage(m.message.content);
            msg.markHistory();
            return msg;
          }
        }
      }) as Message[];

      const chat = new Chat(chatId, messages);
      chats.set(chatId, chat);
      currentChat.value = chat;

      const chatMetaData = chatMetaList.value.find((m) => m.id === chatId)!;
      currentChatMeta.value = chatMetaData;
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
            onClick={createChat}
          >
            <NIcon class="mr-1">
              <PlusIcon />
            </NIcon>
            <span> New Chat </span>
          </div>
          <div class="p-2 text-gray-400">Conversations</div>
          <ExplorerComp
            class="flex-1 overflow-auto"
            active={currentChat.value?.id}
            list={chatMetaList.value}
            onAction={explorerHandler}
          ></ExplorerComp>
          <Version></Version>
        </div>
        <div
          class="flex-1 overflow-hidden"
          style="background-color: var(--body-color)"
        >
          {currentChat.value ? (
            <ChatComp
              ref={chatRef}
              chat={currentChat.value}
              chatMetaData={currentChatMeta.value!}
            ></ChatComp>
          ) : null}
        </div>
      </div>
    );
  },
});
