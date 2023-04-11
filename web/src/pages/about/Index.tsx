import { computed, defineComponent, reactive } from "vue";
import { NButton, NImage, NTag, NText } from "naive-ui";
import { version } from "../../../package.json";
import { appInfo, openUrl, writeToClipboard } from "../../utils/api";
import Logo from "../../assets/logo.png";
import { useAsyncData } from "../../hooks/asyncData";

export default defineComponent({
  setup() {
    const APP_INFO = useAsyncData(appInfo);

    const info = computed(() => ({
      appName: "ChatWizard",
      repoUrl: "https://github.com/lisiur/ChatWizard",
      bugReportUrl: `https://github.com/lisiur/ChatWizard/issues/new?assignees=&labels=${APP_INFO.value?.osType},bug&template=bug_report.md&title=`,
      featureRequestUrl:
        "https://github.com/lisiur/ChatWizard/issues/new?assignees=&labels=enhancement&template=feature_request.md&title=",
      author: {
        name: "Lisiur Day",
        email: "lisiurday@gmail.com",
        homepage: "https://github.com/lisiur",
      },
      maintainers: [],
      contributors: [],
    }));

    async function copyAppInfoHandler() {
      const appInfo = APP_INFO.value;
      const text = ` - App Version: ${appInfo.appVersion}
 - Platform: ${appInfo.platform}
 - OS Type: ${appInfo.osType}
 - OS Version: ${appInfo.osVersion}
 - OS Arch: ${appInfo.osArch}`;
      await writeToClipboard(text);
    }

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
            onClick={() => openUrl(info.value.repoUrl)}
            class="text-2xl"
          >
            {info.value.appName}
          </NButton>
          <NTag round type="primary">
            {version}
          </NTag>
          <NButton
            size="tiny"
            type="tertiary"
            dashed
            ghost
            onClick={copyAppInfoHandler}
          >
            copy app info
          </NButton>
          <div class="grid grid-cols-2 gap-2 mt-4">
            <NButton
              size="small"
              ghost
              type="error"
              onClick={() => openUrl(info.value.bugReportUrl)}
              class="flex-1"
            >
              Bug report
            </NButton>
            <NButton
              size="small"
              ghost
              type="primary"
              onClick={() => openUrl(info.value.featureRequestUrl)}
              class="flex-1"
            >
              Feature request
            </NButton>
          </div>
          <div class="mt-8">
            <NButton
              type="primary"
              text
              onClick={() => openUrl(info.value.repoUrl)}
            >
              {info.value.appName}
            </NButton>
            <NText class="mx-1">© 2023 created by</NText>
            <NButton
              type="default"
              text
              onClick={() => openUrl(info.value.author.homepage)}
            >
              {info.value.author.name}
            </NButton>
          </div>
        </div>
      </div>
    );
  },
});
