import { defineComponent, PropType, ref } from "vue";
import { Message } from "../../models/message";
import { Chat } from "../../models/chat";
import Header from "./Header";
import History from "./History";
import UserInput from "./UserInput";
import { listen } from "../../utils/api";
import { isTauri } from "../../utils/env";

export default defineComponent({
  name: "Chat",
  props: {
    defaultTitle: {
      type: String,
    },
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    onMessage: {
      type: Function as PropType<(message: Message) => void>,
    },
    draggable: {
      type: Boolean,
    },
  },
  setup(props, { expose, slots }) {
    const historyRef = ref<InstanceType<typeof History>>();
    const userInputRef = ref<InstanceType<typeof UserInput>>();

    if (isTauri) {
      listen("tauri://focus", () => {
        // TODO: sync latest chat logs
        // reload();
      });
    }

    const publicExpose = {
      focusInput,
      reload,
    };
    expose(publicExpose);

    function reload() {
      historyRef.value?.reload();
    }

    function sendMessage(message: string) {
      props.chat.sendMessage(message, {
        onFinish: () => historyRef.value?.stopAutoScroll(),
      });
      historyRef.value?.startAutoScroll();
    }

    function resendMessage(id: string) {
      props.chat.resendMessage(id, {
        onFinish: () => historyRef.value?.stopAutoScroll(),
      });
      historyRef.value?.startAutoScroll();
    }

    function updateMessage(id: string, content: string) {
      props.chat.updateLog(id, content);
    }

    function focusInput() {
      userInputRef.value?.focus();
    }

    return (() => (
      <div
        class="h-full flex flex-col"
        style="background-color: var(--body-color)"
      >
        <Header
          chat={props.chat}
          defaultTitle={props.defaultTitle}
          draggable={props.draggable}
        >
          {{
            left: slots.headerLeft,
          }}
        </Header>

        <History
          ref={historyRef}
          chat={props.chat}
          resendMessage={resendMessage}
          updateMessage={updateMessage}
          deleteMessage={props.chat.deleteLog.bind(props.chat)}
          stopReply={props.chat.stopReply.bind(props.chat)}
        ></History>

        <UserInput
          ref={userInputRef}
          chat={props.chat}
          sendMessage={sendMessage}
          onMessage={props.onMessage}
        ></UserInput>
      </div>
    )) as unknown as typeof publicExpose;
  },
});
