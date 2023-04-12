import { defineComponent } from "vue";
import ChatComp from "../../components/chat/Chat";
import { Chat } from "../../models/chat";
import * as api from "../../api";
import { useAsyncData, useAsyncDataReactive } from "../../hooks/asyncData";
import { useI18n } from "../../hooks/i18n";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const chatIndex = useAsyncData(async () => {
      return api.casualChat();
    });

    const chat = useAsyncDataReactive(async () => {
      if (!chatIndex.value) {
        return undefined;
      }
      return new Chat(chatIndex.value);
    }, chatIndex);

    return () => (
      <div class="h-full">
        {chat.value ? (
          <ChatComp
            chat={chat.value}
            defaultTitle={t("chat.casual.title")}
            draggable
          ></ChatComp>
        ) : null}
      </div>
    );
  },
});
