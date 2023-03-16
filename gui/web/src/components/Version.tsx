import { AngleDoubleUp } from "@vicons/fa";
import { NButton, NIcon, NSpace } from "naive-ui";
import { defineComponent, ref } from "vue";
import { useVersion } from "../hooks/version";
import mdRender from "../utils/mdRender";
import { dialog, message } from "../utils/prompt";

export default defineComponent({
  setup() {
    const { version, hasNewVersion, installNewVersion, newVersion, relaunch } =
      useVersion();

    function showUpdateHandler() {
      const releaseContent = (newVersion.value?.body ?? "").replaceAll(
        /%0A/g,
        "\n"
      );
      const renderContent = mdRender(releaseContent);
      const loading = ref(false);
      const dl = dialog.create({
        showIcon: false,
        title: "New version is available!",
        content: () => {
          return <div class="markdown-root" v-html={renderContent}></div>;
        },
        action: () => {
          const positiveText = ref("Upgrade");
          return (
            <NSpace>
              <NButton onClick={() => dl.destroy()}>Cancel</NButton>
              <NButton
                type="primary"
                loading={loading.value}
                onClick={() => {
                  positiveText.value = "Downloading...";
                  loading.value = true;
                  installNewVersion()
                    .then(() => {
                      dl.destroy();
                      dialog.success({
                        title: "Download Success",
                        content: "Please restart the app to apply the update.",
                        positiveText: "Restart",
                        negativeText: "Later",
                        onPositiveClick: relaunch,
                      });
                    })
                    .catch((err) => {
                      message.error(err);
                    })
                    .finally(() => {
                      positiveText.value = "Upgrade";
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
      <div class="flex justify-center text-gray-500 p-2">
        <span> v{version.value} </span>
        {hasNewVersion.value ? (
          <span onClick={showUpdateHandler} class="cursor-pointer">
            <NIcon color="var(--primary-color)" class="ml-1">
              <AngleDoubleUp />
            </NIcon>
          </span>
        ) : null}
      </div>
    );
  },
});
