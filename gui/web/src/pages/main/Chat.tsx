import { computed, defineComponent, ref, shallowReactive } from "vue";
import ChatComp from "../../components/Chat";
import * as api from "../../api";
import { Chat } from "../../models/chat";
import { Message, UserMessage } from "../../models/message";
import { Plus as PlusIcon } from "@vicons/fa";
import { NIcon } from "naive-ui";
import { useRoute } from "vue-router";
import { useI18n } from "../../hooks/i18n";
import { prompt } from "../../utils/prompt";
import Explorer, { ExplorerItem } from "../../components/Explorer";

export default defineComponent({
  setup() {
    const { t } = useI18n();
    const route = useRoute();

    const chatRef = ref<InstanceType<typeof ChatComp>>();

    const chatIndexList = ref<Array<api.ChatIndex>>([]);
    const explorerList = computed(() => {
      return chatIndexList.value.map((m) => ({
        id: m.id,
        title: m.title || t("chat.new.defaultTitle"),
      }));
    });

    const currentChat = ref<Chat>();
    const currentChatIndex = computed(
      () =>
        chatIndexList.value.find((m) => m.id === currentChat.value?.index.id)!
    );

    refreshChatMetaList().then(() => {
      if (route.query.id) {
        selectHandler(route.query.id as string);
      }
    });

    async function refreshChatMetaList() {
      await api.allChats().then((list) => {
        chatIndexList.value = list;
      });
    }

    async function createChat() {
      const chatId = await api.createChat({
        title: "",
      });
      await refreshChatMetaList();

      await selectHandler(chatId);

      setTimeout(() => {
        chatRef.value?.focusInput();
      });
    }

    async function explorerHandler(action: string, item: ExplorerItem) {
      switch (action) {
        case "delete": {
          await deleteHandler(item.id);
          return;
        }
        case "select": {
          await selectHandler(item.id);
          return;
        }
        case "rename": {
          await renameHandler(item.id, item.title);
        }
      }
    }

    async function renameHandler(id: string, title: string) {
      prompt(t("chat.inputNameHint"), {
        defaultValue: title,
        async okHandler(title) {
          await api.updateChat({
            id,
            title,
          });
          if (currentChat.value && currentChat.value.index.id === id) {
            currentChat.value.index.title = title;
          }
          await refreshChatMetaList();
        },
      });
    }

    async function deleteHandler(id: string) {
      if (currentChat.value?.index.id === id) {
        currentChat.value = undefined;
      }
      await api.deleteChat(id);
      refreshChatMetaList();
    }

    async function selectHandler(id: string) {
      const index = chatIndexList.value.find((m) => m.id === id)!;
      const logs = await api.loadChat(id);
      const chat = Chat.init(index, logs);
      currentChat.value = shallowReactive(chat);

      setTimeout(() => {
        chatRef.value?.focusInput();
      });
    }

    async function messageHandler(message: Message) {
      const chat = currentChat.value!;
      const chatMetaData = currentChatIndex.value!;

      // If the chat is empty, set the title to the first message.
      if (
        message instanceof UserMessage &&
        chat.messages.length === 0 &&
        chatMetaData.title === ""
      ) {
        await api.updateChat({
          id: chatMetaData.id,
          title: message.content,
        });
        currentChatIndex.value!.title = message.content;
        currentChat.value!.index.title = message.content;
        await refreshChatMetaList();
      }
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div
            class="h-10 border-b flex justify-center m-2 mt-3 items-center bg-primary cursor-pointer"
            style="color: var(--base-color);border-color: var(--border-color)"
            onClick={createChat}
          >
            <NIcon class="mr-1">
              <PlusIcon />
            </NIcon>
            <span> {t("chat.new")} </span>
          </div>
          <div class="p-2 text-gray-400">{t("chat.conversations")}</div>
          <Explorer
            class="flex-1 overflow-auto"
            active={currentChat.value?.index.id}
            menus={[
              {
                label: t("chat.rename"),
                key: "rename",
              },
              {
                type: "divider",
              },
              {
                label: t("common.delete"),
                key: "delete",
              },
            ]}
            list={explorerList.value}
            onAction={explorerHandler}
          ></Explorer>
        </div>
        <div
          class="flex-1 overflow-hidden"
          style="background-color: var(--body-color)"
        >
          {currentChat.value ? (
            <ChatComp
              ref={chatRef}
              chat={currentChat.value}
              onMessage={messageHandler}
            ></ChatComp>
          ) : (
            <div class="h-full" data-tauri-drag-region></div>
          )}
        </div>
      </div>
    );
  },
});
