import { defineComponent, ref } from "vue";
import ChatComp from "../../components/Chat";
import ExplorerComp from "../../components/ChatExplorer";
import * as api from "../../api";
import { Chat } from "../../models/chat";
import { AssistantMessage, Message, UserMessage } from "../../models/message";
import { Plus as PlusIcon } from "@vicons/fa";
import { NIcon } from "naive-ui";
import { useRoute } from "vue-router";
import { useI18n } from "../../hooks/i18n";
import { prompt } from "../../utils/prompt";

export default defineComponent({
  setup() {
    const { t } = useI18n();
    const route = useRoute();

    const chatRef = ref<InstanceType<typeof ChatComp>>();

    const chatMetaList = ref<Array<{ id: string; title: string }>>([]);
    const chats = new Map<string, Chat>();
    const currentChat = ref<Chat>();
    const currentChatMeta = ref<{ id: string; title: string }>();
    refreshChatMetaList();

    if (route.query.id) {
      selectHandler(route.query.id as string);
    }

    async function refreshChatMetaList() {
      await api.allChats().then((list) => {
        chatMetaList.value = list;
      });
    }

    async function createChat() {
      const chatId = await api.createChat({
        title: "New Chat",
      });
      const chat = new Chat(chatId);
      await refreshChatMetaList();
      currentChatMeta.value = chatMetaList.value.find((m) => m.id === chatId)!;
      currentChat.value = chat;
      setTimeout(() => {
        chatRef.value?.focusInput();
      });
    }

    async function explorerHandler(
      action: "delete" | "select" | "rename",
      metadata: api.ChatMetadata
    ) {
      switch (action) {
        case "delete": {
          await deleteHandler(metadata.id);
          return;
        }
        case "select": {
          await selectHandler(metadata.id);
          return;
        }
        case "rename": {
          await renameHandler(metadata);
        }
      }
    }

    async function renameHandler(metadata: api.ChatMetadata) {
      prompt(t("prompt.inputNameHint"), {
        defaultValue: metadata.title,
        async okHandler(title) {
          await api.updateChat({
            id: metadata.id,
            title,
          });
          await refreshChatMetaList();
        },
      });
    }

    async function deleteHandler(id: string) {
      if (currentChat.value?.id === id) {
        currentChat.value = undefined;
      }
      await api.deleteChat(id);
      chats.delete(id);
      refreshChatMetaList();
    }

    async function selectHandler(id: string) {
      const chatData = await api.readChat(id);
      const messages = chatData.logs.map((m) => {
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

      const chat = new Chat(id, messages);
      chats.set(id, chat);
      currentChat.value = chat;

      const chatMetaData = chatMetaList.value.find((m) => m.id === id)!;
      currentChatMeta.value = chatMetaData;

      setTimeout(() => {
        chatRef.value?.focusInput();
      });
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
            onClick={createChat}
          >
            <NIcon class="mr-1">
              <PlusIcon />
            </NIcon>
            <span> {t("chat.new")} </span>
          </div>
          <div class="p-2 text-gray-400">{t("chat.conversations")}</div>
          <ExplorerComp
            class="flex-1 overflow-auto"
            active={currentChat.value?.id}
            list={chatMetaList.value}
            onAction={explorerHandler}
          ></ExplorerComp>
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
          ) : (
            <div class="h-full" data-tauri-drag-region></div>
          )}
        </div>
      </div>
    );
  },
});
