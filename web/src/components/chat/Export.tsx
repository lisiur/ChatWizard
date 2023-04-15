import { defineComponent, PropType } from "vue";
import { Share20Filled as ExportIcon } from "@vicons/fluent";
import { NButton, NIcon } from "naive-ui";
import { Chat } from "../../models/chat";
import { showOrCreateWindow } from "../../api";
import { useI18n } from "../../hooks/i18n";
import { isWeb } from "../../utils/env";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
  },
  setup(props) {
    const { t } = useI18n();

    function exportHandler() {
      const url = "index.html/#/export?chatId=" + props.chat.index.id;
      if (isWeb) {
        window.open(url);
      } else {
        showOrCreateWindow("export", {
          width: 960,
          height: 720,
          resizable: true,
          title: "Export",
          url,
          visible: false,
          alwaysOnTop: false,
        });
      }
    }

    return () => (
      <NButton tertiary size="tiny" onClick={exportHandler}>
        <NIcon size={14}>
          <ExportIcon></ExportIcon>
        </NIcon>
        <span class="ml-[.1rem]">{t("chat.export")}</span>
      </NButton>
    );
  },
});
