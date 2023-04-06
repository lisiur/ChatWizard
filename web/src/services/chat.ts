import { computed, ref } from "vue";
import * as api from "../api";
import { useTask } from "../hooks/task";

export function useChatService() {
  const loaded = ref(false);

  const loadAllStickChatsTask = useTask(
    async () => {
      return await api.allStickChats();
    },
    {
      default: [],
    }
  );

  const loadAllNonStickChatsTask = useTask(
    async () => {
      return await api.allNonStickChats();
    },
    {
      default: [],
    }
  );

  const loadAllArchiveChatsTask = useTask(
    async () => {
      return await api.allArchiveChats();
    },
    {
      default: [],
    }
  );

  const allStickChats = computed(() => loadAllStickChatsTask.result ?? []);
  const allNonStickChats = computed(
    () => loadAllNonStickChatsTask.result ?? []
  );
  const allArchiveChats = computed(() => loadAllArchiveChatsTask.result ?? []);

  const allChats = computed(() => {
    return allStickChats.value
      .concat(allNonStickChats.value)
      .concat(allArchiveChats.value);
  });

  async function reload() {
    const res = await Promise.all([
      loadAllNonStickChatsTask.exec(),
      loadAllStickChatsTask.exec(),
      loadAllArchiveChatsTask.exec(),
    ]);
    loaded.value = true;
    return res;
  }

  async function setChatStick(chatId: string, stick: boolean) {
    await api.setChatStick(chatId, stick);
    await reload();
  }

  async function setChatArchive(chatId: string) {
    await api.setChatArchive(chatId);
    await reload();
  }

  async function moveStickChat(from: string, to: string) {
    await api.moveStickChat(from, to);
    await reload();
  }

  async function moveNonStickChat(from: string, to: string) {
    await api.moveNonStickChat(from, to);
    await reload();
  }

  return {
    loaded,
    load: reload,
    reload,
    allChats,
    allStickChats,
    allNonStickChats,
    allArchiveChats,
    setChatStick,
    setChatArchive,
    moveStickChat,
    moveNonStickChat,
  };
}
