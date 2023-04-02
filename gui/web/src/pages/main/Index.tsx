import { computed, defineComponent } from "vue";
import { RouterView, useRoute, useRouter } from "vue-router";
import {
  ChatbubbleEllipsesOutline as InactiveChatIcon,
  ChatbubbleEllipses as ActiveChatIcon,
  Settings as SettingIcon,
  TerminalOutline as PromptIcon,
  Terminal as PromptActiveIcon,
  StorefrontOutline as StoreIcon,
  Storefront as StoreActiveIcon,
} from "@vicons/ionicons5";
import { NBadge, NDropdown, NIcon } from "naive-ui";
import { showOrCreateWindow } from "../../api";
import { os } from "@tauri-apps/api";
import { useAsyncData } from "../../hooks/asyncData";
import { useVersion } from "../../hooks/version";
import Version from "../../components/Version";
import { i18n } from "../../hooks/i18n";

export default defineComponent({
  setup() {
    const route = useRoute();
    const router = useRouter();
    const { hasNewVersion } = useVersion();
    const { t } = i18n.global;

    const topMenus = [
      {
        routeName: "chat",
        icon: InactiveChatIcon,
        activeIcon: ActiveChatIcon,
        size: "2rem",
      },
      {
        routeName: "prompt",
        icon: PromptIcon,
        activeIcon: PromptActiveIcon,
        size: "1.7rem",
      },
      {
        routeName: "promptMarket",
        icon: StoreIcon,
        activeIcon: StoreActiveIcon,
        size: "1.7rem",
      },
    ];

    const platform = useAsyncData(async () => {
      return await os.platform();
    });

    const isMacos = computed(() => platform.value === "darwin");

    function settingActionHandler(key: string) {
      switch (key) {
        case "setting": {
          showOrCreateWindow("setting", {
            title: t("window.setting"),
            url: `/#${router.resolve({ name: "setting" }).path}`,
            width: 520,
            height: 400,
            resizable: false,
            alwaysOnTop: false,
            visible: false,
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
              const isActive = route.name === m.routeName;
              const color = isActive
                ? "var(--primary-color)"
                : "var(--switcher-color)";
              const Icon = isActive ? m.activeIcon : m.icon;
              return (
                <div
                  class="mt-6 flex justify-center"
                  onClick={() => router.push({ name: m.routeName })}
                >
                  <NIcon size={m.size} color={color}>
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
                      label: t("config.setting"),
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
