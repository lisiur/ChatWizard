import { defineComponent, PropType } from "vue";
import { Share20Filled as ExportIcon } from "@vicons/fluent";
import { NButton, NIcon } from "naive-ui";
import { Chat } from "../../models/chat";
import { showOrCreateWindow } from "../../api";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
  },
  setup(props) {
    function exportHandler() {
      showOrCreateWindow("export", {
        alwaysOnTop: true,
        width: 800,
        height: 800,
        resizable: false,
        title: "Export",
        url: "/#/export?chatId=" + props.chat.index.id,
        visible: true,
      });
    }

    return () => (
      <NButton tertiary size="tiny" onClick={exportHandler}>
        <NIcon size={10}>
          <ExportIcon></ExportIcon>
        </NIcon>
        <span class="ml-[.1rem]">Export</span>
      </NButton>
    );
  },
});
