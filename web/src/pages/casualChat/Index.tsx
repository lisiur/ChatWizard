import "./style.css";
import { defineComponent, ref } from "vue";
import ChatComp from "../../components/chat/Chat";
import { Chat } from "../../models/chat";
import * as api from "../../api";
import { useAsyncData, useAsyncDataReactive } from "../../hooks/asyncData";
import { useI18n } from "../../hooks/i18n";
import { useHideWindowWhenBlur } from "../../hooks/hideWindowWhenBlur";
import { Pin20Filled as PinIcon } from "@vicons/fluent";
import { NIcon } from "naive-ui";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const chatRef = ref<InstanceType<typeof ChatComp>>();

    const chatIndex = useAsyncData(async () => {
      return api.casualChat();
    });

    const chat = useAsyncDataReactive(async () => {
      if (!chatIndex.value) {
        return undefined;
      }
      return new Chat(chatIndex.value);
    }, chatIndex);

    const {
      enabled: autoHideWindowEnabled,
      toggleEnable: toggleEnableAutoHideWindow,
    } = useHideWindowWhenBlur({
      onFocus() {
        chatRef.value?.focusInput();
      },
    });

    return () => (
      <div class="h-full">
        {chat.value ? (
          <ChatComp
            ref={chatRef}
            chat={chat.value}
            defaultTitle={t("chat.casual.title")}
          >
            {{
              headerLeft: () => (
                <span onClick={toggleEnableAutoHideWindow} class="select-none">
                  <NIcon size={16} class="relative top-[.2rem]">
                    <PinIcon
                      class={[
                        "transition-transform",
                        !autoHideWindowEnabled.value
                          ? "text-primary -rotate-45"
                          : "text-gray-500",
                      ]}
                    ></PinIcon>
                  </NIcon>
                </span>
              ),
            }}
          </ChatComp>
        ) : null}
      </div>
    );
  },
});
