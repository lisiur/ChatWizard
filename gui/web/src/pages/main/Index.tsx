import { computed, defineComponent } from "vue";
import { RouterView, useRoute, useRouter } from "vue-router";
import {
  ChatbubbleEllipsesOutline as InactiveChatIcon,
  ChatbubbleEllipses as ActiveChatIcon,
  Settings as SettingIcon,
} from "@vicons/ionicons5";

import { Prompt as PromptIcon } from "@vicons/tabler";
import { NBadge, NDropdown, NIcon } from "naive-ui";
import { showWindow } from "../../api";
import { os } from "@tauri-apps/api";
import { useAsyncData } from "../../hooks/asyncData";
import { useVersion } from "../../hooks/version";
import Version from "../../components/Version";

export default defineComponent({
  setup() {
    const route = useRoute();
    const router = useRouter();
    const { hasNewVersion } = useVersion();

    const topMenus = [
      {
        name: "chat",
        url: router.resolve({ name: "chat" }).path,
        icon: InactiveChatIcon,
        activeIcon: ActiveChatIcon,
      },
      {
        name: "prompt",
        url: router.resolve({ name: "prompt" }).path,
        icon: PromptIcon,
        activeIcon: PromptIcon,
      },
    ];

    const platform = useAsyncData(async () => {
      return await os.platform();
    });

    const isMacos = computed(() => platform.value === "darwin");

    function settingActionHandler(key: string) {
      switch (key) {
        case "setting": {
          showWindow("setting", {
            title: "Setting",
            url: `/#${router.resolve({ name: "setting" }).path}`,
            width: 520,
            height: 400,
          });
          break;
        }
      }
    }

    const Setting = () => (
      <NBadge dot show={hasNewVersion.value}>
        <NIcon size="2rem" color="var(--switcher-color)">
          <SettingIcon />
        </NIcon>
      </NBadge>
    );

    return () => (
      <div class="h-full flex">
        <div
          data-tauri-drag-region
          class="w-16 border-r h-full flex flex-col"
          style={{
            backgroundColor: "var(--switcher-bg-color)",
            borderColor: "var(--border-color)",
            paddingTop: isMacos.value ? "22px" : "0",
          }}
        >
          <div class="grid gap-1 place-content-center">
            {topMenus.map((m) => {
              const isActive = route.name === m.name;
              const color = isActive
                ? "var(--primary-color)"
                : "var(--switcher-color)";
              const Icon = isActive ? m.activeIcon : m.icon;
              return (
                <div class="mt-4" onClick={() => router.push({ name: m.name })}>
                  <NIcon size="2rem" color={color}>
                    <Icon></Icon>
                  </NIcon>
                </div>
              );
            })}
          </div>
          <div class="mt-auto mb-4 flex justify-center items-center flex-col">
            <div class="grid gap-1 place-content-center">
              {hasNewVersion.value ? (
                <NDropdown
                  trigger="click"
                  placement="right-end"
                  options={[
                    {
                      label: "Setting",
                      key: "setting",
                    },
                    {
                      type: "divider",
                    },
                    {
                      type: "render",
                      render: () => <Version></Version>,
                    },
                  ]}
                  onSelect={settingActionHandler}
                >
                  <Setting></Setting>
                </NDropdown>
              ) : (
                <div onClick={() => settingActionHandler("setting")}>
                  <Setting></Setting>
                </div>
              )}
            </div>
          </div>
        </div>
        <div class="flex-1 overflow-hidden">
          <RouterView />
        </div>
      </div>
    );
  },
});
