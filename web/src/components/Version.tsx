import { NBadge, NButton, NSpace } from "naive-ui";
import { defineComponent, ref } from "vue";
import { useI18n } from "../hooks/i18n";
import { useVersion } from "../hooks/version";
import mdRender from "../utils/mdRender";
import { dialog, message } from "../utils/prompt";

export default defineComponent({
  setup() {
    const { hasNewVersion, installNewVersion, newVersion, relaunch } =
      useVersion();
    const { t } = useI18n();

    function showUpdateHandler() {
      const releaseContent = (newVersion.value?.body ?? "").replaceAll(
        /%0A/g,
        "\n"
      );
      const renderContent = mdRender(releaseContent);
      const loading = ref(false);
      const dl = dialog.create({
        showIcon: false,
        title: t("setting.upgrade.newVersion"),
        content: () => {
          return <div class="markdown-root" v-html={renderContent}></div>;
        },
        action: () => {
          const positiveText = ref(t("setting.upgrade.upgrade"));
          return (
            <NSpace>
              <NButton onClick={() => dl.destroy()}>Cancel</NButton>
              <NButton
                type="primary"
                loading={loading.value}
                onClick={() => {
                  positiveText.value = t("setting.upgrade.downloading");
                  loading.value = true;
                  installNewVersion()
                    .then(() => {
                      dl.destroy();
                      dialog.success({
                        title: t("setting.upgrade.download.success"),
                        content: t("setting.upgrade.restart.hint"),
                        positiveText: t("setting.upgrade.relaunch"),
                        negativeText: t("setting.upgrade.later"),
                        onPositiveClick: relaunch,
                      });
                    })
                    .catch((err) => {
                      message.error(err);
                    })
                    .finally(() => {
                      positiveText.value = t("setting.upgrade.upgrade");
                      loading.value = false;
                    });
                }}
              >
                {positiveText.value}
              </NButton>
            </NSpace>
          );
        },
      });
    }

    return () => (
      <div
        class="flex justify-center p-2"
        onClick={hasNewVersion.value ? showUpdateHandler : () => null}
      >
        <NBadge dot show={hasNewVersion.value}>
          <span class="select-none cursor-default">
            {t("common.newVersion")}
          </span>
        </NBadge>
      </div>
    );
  },
});
