import { defineComponent, PropType, ref, DefineComponent } from "vue";
import { Message } from "../../models/message";
import { NIcon, NTooltip } from "naive-ui";
import { Chat } from "../../models/chat";
import { save } from "@tauri-apps/api/dialog";
import Header from "./Header";
import History from "./History";
import UserInput from "./UserInput";

export default defineComponent({
  name: "Chat",
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    onMessage: {
      type: Function as PropType<(message: Message) => void>,
    },
  },
  setup(props, { expose }) {
    const historyRef = ref<InstanceType<typeof History>>();
    const userInputRef = ref<InstanceType<typeof UserInput>>();

    const publicExpose = {
      focusInput,
    };
    expose(publicExpose);

    function sendMessage(message: string) {
      props.chat.sendMessage(message, {
        onFinish: () => historyRef.value?.stopAutoScroll(),
      });
      setTimeout(() => {
        historyRef.value?.startAutoScroll();
      }, 20);
    }

    function resendMessage(id: string) {
      props.chat.resendMessage(id, {
        onFinish: () => historyRef.value?.stopAutoScroll(),
      });
      setTimeout(() => {
        historyRef.value?.startAutoScroll();
      }, 20);
    }

    async function exportMarkdown() {
      const filePath = await save({
        title: props.chat.index.title,
        filters: [
          {
            name: "Markdown",
            extensions: ["md"],
          },
        ],
      });
      if (filePath) {
        props.chat.exportMarkdown(filePath);
      }
    }

    function focusInput() {
      userInputRef.value?.focus();
    }

    function renderButton(props: {
      icon: DefineComponent<any, any, any>;
      tooltip: string;
      handler: () => void;
    }) {
      return (
        <NTooltip trigger="hover" delay={500}>
          {{
            trigger: () => (
              <button
                class="bg-transparent rounded px-2 py-1"
                onClick={props.handler}
              >
                <NIcon
                  size="1rem"
                  class="text-[var(--chat-btn-color)] hover:text-[var(--primary-color)]"
                >
                  <props.icon></props.icon>
                </NIcon>
              </button>
            ),
            default: () => props.tooltip,
          }}
        </NTooltip>
      );
    }

    return (() => (
      <div
        class="h-full flex flex-col"
        style="background-color: var(--body-color)"
      >
        <Header chat={props.chat}></Header>

        <History
          ref={historyRef}
          messages={props.chat.messages}
          resendMessage={resendMessage}
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
