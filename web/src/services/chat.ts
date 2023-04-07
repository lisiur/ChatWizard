import { computed, ref } from "vue";
import * as api from "../api";
import { useTask } from "../hooks/task";

export function useChatService() {
  const loaded = ref(false);

  const loadAllChatsExceptCasualTask = useTask(
    async () => {
      return await api.allChatsExceptCasual();
    },
    {
      default: [],
    }
  );

  const allChats = computed(() => loadAllChatsExceptCasualTask.result ?? []);
  const allStickChats = computed(() =>
    (loadAllChatsExceptCasualTask.result ?? []).filter((it) => it.stick)
  );
  const allNonStickChats = computed(() =>
    (loadAllChatsExceptCasualTask.result ?? []).filter((it) => !it.stick)
  );
  const allArchiveChats = computed(() =>
    (loadAllChatsExceptCasualTask.result ?? []).filter((it) => it.archive)
  );

  async function reload() {
    const res = await loadAllChatsExceptCasualTask.exec();
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
