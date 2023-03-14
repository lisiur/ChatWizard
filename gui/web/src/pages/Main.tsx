import { defineComponent, Suspense } from "vue";
import ChatComp from "../components/Chat";
import * as api from "../api";
import { useAsyncData } from "../hooks/asyncData";
import { Chat } from "../models/chat";

export default defineComponent({
  setup() {
    const chat = useAsyncData(async () => {
      const chatId = await api.createChat();
      return new Chat(chatId);
    });
    return () => (
      <div class="h-full">
        {chat.value ? <ChatComp chat={chat.value}></ChatComp> : null}
      </div>
    );
  },
});
