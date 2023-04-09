import { NTag, NTooltip } from "naive-ui";
import { computed, defineComponent, PropType } from "vue";
import { useI18n } from "../../hooks/i18n";
import { usePrompt } from "../../hooks/prompt";
import { Chat } from "../../models/chat";
import DragBar from "../DragBar";
import ChatConfig from "./ChatConfig";

export default defineComponent({
  props: {
    defaultTitle: {
      type: String,
    },
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
    draggable: {
      type: Boolean,
      default: true,
    },
  },
  setup(props) {
    const { t } = useI18n();
    const prompt = usePrompt(computed(() => props.chat.index.promptId));

    return () => (
      <DragBar
        title={
          props.chat.index.title ||
          props.defaultTitle ||
          t("chat.new.defaultTitle")
        }
        disabled={!props.draggable}
      >
        {{
          "right-panel": () => (
            <>
              {prompt.value?.name ? (
                <NTooltip contentStyle="max-width: 30rem">
                  {{
                    trigger: () => (
                      <NTag size="small" round type="primary">
                        {prompt.value?.name}
                      </NTag>
                    ),
                    default: () => prompt.value?.content,
                  }}
                </NTooltip>
              ) : null}
              <ChatConfig class="ml-2" chat={props.chat}></ChatConfig>
            </>
          ),
        }}
      </DragBar>
    );
  },
});
