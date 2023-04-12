import { NTag, NTooltip } from "naive-ui";
import { defineComponent, PropType } from "vue";
import { useI18n } from "../../hooks/i18n";
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
              {props.chat.prompt?.name ? (
                <NTooltip contentStyle="max-width: 30rem">
                  {{
                    trigger: () => (
                      <NTag
                        size="small"
                        round
                        type="primary"
                        closable
                        onClose={() => {
                          props.chat.removePrompt();
                        }}
                      >
                        {props.chat.prompt?.name}
                      </NTag>
                    ),
                    default: () => props.chat.prompt?.content,
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
