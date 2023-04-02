import { defineComponent, nextTick, PropType, ref } from "vue";
import { useComposition } from "../../hooks/composition";
import { useI18n } from "../../hooks/i18n";
import { Chat } from "../../models/chat";
import { Message, UserMessage } from "../../models/message";
import { message } from "../../utils/prompt";
import Backtrack from "./Backtrack";
import Cost from "./Cost";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    sendMessage: {
      type: Function as PropType<(message: string) => void>,
      required: true,
    },
    onMessage: {
      type: Function as PropType<(message: Message) => void>,
    },
  },
  setup(props, { expose }) {
    const { t } = useI18n();

    const inputRef = ref<HTMLTextAreaElement>();
    const { isComposition } = useComposition(inputRef);
    const userMessage = ref("");

    const publicInstance = {
      focus,
    };
    expose(publicInstance);

    function focus() {
      inputRef.value?.focus();
    }

    function keydownHandler(e: KeyboardEvent) {
      if (e.key === "Tab") {
        e.preventDefault();
        const start = inputRef.value?.selectionStart;
        const end = inputRef.value?.selectionEnd;
        if (start !== undefined && end !== undefined) {
          userMessage.value =
            userMessage.value.substring(0, start) +
            "  " +
            userMessage.value.substring(end);
          nextTick(() => {
            inputRef.value?.setSelectionRange(start + 4, start + 4);
          });
        }
      }
      if (
        e.key === "Enter" &&
        !e.ctrlKey &&
        !e.altKey &&
        !e.shiftKey &&
        !isComposition.value
      ) {
        if (props.chat.busy.value) {
          message.warning(t("chat.busy"));
          e.preventDefault();
          return;
        }

        const msg = userMessage.value;

        props.onMessage?.(new UserMessage(msg));

        userMessage.value = "";
        props.sendMessage(msg);
        e.preventDefault();
      }
    }

    return (() => (
      <div class="border-t" style="border-color: var(--border-color)">
        <div class="flex items-center h-8">
          <Cost class="pl-2 text-xs" value={props.chat.index.cost}></Cost>
          <Backtrack class="ml-2" chat={props.chat}></Backtrack>
          <div class="flex-1 flex justify-end p-1">
            {/* {renderButton({
                handler: exportMarkdown,
                icon: MarkdownIcon,
                tooltip: t("chat.exportMarkdown"),
              })} */}
          </div>
        </div>
        <textarea
          ref={inputRef}
          v-model={userMessage.value}
          class="p-2 resize-none w-full bg-transparent outline-none placeholder-slate-500"
          style="color: var(--input-msg-color)"
          rows="5"
          onKeydown={keydownHandler}
        ></textarea>
      </div>
    )) as unknown as typeof publicInstance;
  },
});
