import { defineComponent, reactive } from "vue";
import { NButton, NImage, NTag, NText } from "naive-ui";
import { version } from "../../../package.json";
import { openUrl } from "../../utils/api";
import Logo from "../../assets/logo.png";

export default defineComponent({
  setup() {
    const info = reactive({
      appName: "ChatWizard",
      repoUrl: "https://github.com/lisiur/ChatWizard",
      bugReportUrl:
        "https://github.com/lisiur/ChatWizard/issues/new?assignees=&labels=&template=bug_report.md&title=",
      featureRequestUrl:
        "https://github.com/lisiur/ChatWizard/issues/new?assignees=&labels=&template=feature_request.md&title=",
      author: {
        name: "Lisiur Day",
        email: "lisiurday@gmail.com",
        homepage: "https://github.com/lisiur",
      },
      maintainers: [],
      contributors: [],
    });

    return () => (
      <div class="grid place-items-center h-full" data-tauri-drag-region>
        <div
          class="grid grid-cols-1 place-items-center gap-2"
          data-tauri-drag-region
        >
          <NImage src={Logo} width="80" height="80" previewDisabled></NImage>
          <NButton
            text
            type="default"
            onClick={() => openUrl(info.repoUrl)}
            class="text-2xl"
          >
            {info.appName}
          </NButton>
          <NTag round type="primary">
            {version}
          </NTag>
          <div class="grid grid-cols-2 gap-2 mt-4">
            <NButton
              size="small"
              ghost
              type="error"
              onClick={() => openUrl(info.bugReportUrl)}
              class="flex-1"
            >
              Bug report
            </NButton>
            <NButton
              size="small"
              ghost
              type="primary"
              onClick={() => openUrl(info.featureRequestUrl)}
              class="flex-1"
            >
              Feature request
            </NButton>
          </div>
          <div class="mt-8">
            <NButton type="primary" text onClick={() => openUrl(info.repoUrl)}>
              {info.appName}
            </NButton>
            <NText class="mx-1">© 2023 created by</NText>
            <NButton
              type="default"
              text
              onClick={() => openUrl(info.author.homepage)}
            >
              {info.author.name}
            </NButton>
          </div>
        </div>
      </div>
    );
  },
});
