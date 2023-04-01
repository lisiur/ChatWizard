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
import { useChatService } from "../../services/chat";

export default defineComponent({
  setup() {
    const { t } = useI18n();
    const route = useRoute();

    const chatRef = ref<InstanceType<typeof ChatComp>>();

    const {
      load,
      reload,
      allChats,
      allStickChats,
      allNonStickChats,
      moveNonStickChat,
      moveStickChat,
      setChatStick,
    } = useChatService();

    const nonStickExplorerList = computed(() => {
      return allNonStickChats.value.map((m) => ({
        id: m.id,
        title: m.title || t("chat.new.defaultTitle"),
        data: m,
      }));
    });

    const stickExplorerList = computed(() => {
      return allStickChats.value.map((m) => ({
        id: m.id,
        title: m.title || t("chat.new.defaultTitle"),
        data: m,
      }));
    });

    const currentChat = ref<Chat>();
    const currentChatIndex = computed(
      () => allChats.value.find((m) => m.id === currentChat.value?.index.id)!
    );

    load().then(() => {
      if (route.query.id) {
        selectHandler(route.query.id as string);
      } else {
        if (allChats.value.length > 0) {
          selectHandler(allChats.value[0].id);
        }
      }
    });

    async function createChat() {
      const chatId = await api.newChat({
        title: "",
      });
      await reload();

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
          return;
        }
        case "stick": {
          await setChatStick(item.id, true);
          return;
        }
        case "unstick": {
          await setChatStick(item.id, false);
          return;
        }
      }
    }

    async function explorerDragHandler(
      group: "stick" | "unstick",
      from: string,
      to: string
    ) {
      if (group === "stick") {
        await moveStickChat(from, to);
      } else {
        await moveNonStickChat(from, to);
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
          await reload();
        },
      });
    }

    async function deleteHandler(id: string) {
      if (currentChat.value?.index.id === id) {
        currentChat.value = undefined;
      }
      await api.deleteChat(id);
      reload();
    }

    async function selectHandler(id: string) {
      const index = allChats.value.find((m) => m.id === id)!;
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
        await reload();
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
              (item) => {
                const data = item.data as api.ChatIndex;
                if (data.stick) {
                  return {
                    label: t("chat.unstick"),
                    key: "unstick",
                  };
                } else {
                  return {
                    label: t("chat.stick"),
                    key: "stick",
                  };
                }
              },
              {
                type: "divider",
              },
              {
                label: t("common.delete"),
                key: "delete",
              },
            ]}
            stickList={stickExplorerList.value}
            unstickList={nonStickExplorerList.value}
            onAction={explorerHandler}
            onDarg={explorerDragHandler}
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
