import { computed, defineComponent } from "vue";
import { RouterView, useRoute, useRouter } from "vue-router";
import {
  Chat20Regular as ChatIcon,
  Chat20Filled as ChatActiveIcon,
  CommentLightning20Regular as PromptIcon,
  CommentLightning20Filled as PromptActiveIcon,
  Apps20Regular as StoreIcon,
  Apps20Filled as StoreActiveIcon,
  Lightbulb20Regular as CasualChatIcon,
  LightbulbFilament20Filled as CasualActiveIcon,
  BoxMultiple20Regular as ModelIcon,
  BoxMultiple20Filled as ModelActiveIcon,
} from "@vicons/fluent";
import {
  ExtensionPuzzle as PluginActiveIcon,
  ExtensionPuzzleOutline as PluginIcon,
} from "@vicons/ionicons5";

import { SettingsAdjust as SettingIcon } from "@vicons/carbon";
import { NBadge, NDropdown, NIcon } from "naive-ui";
import { useAsyncData } from "../../hooks/asyncData";
import { useVersion } from "../../hooks/version";
import Version from "../../components/Version";
import { i18n } from "../../hooks/i18n";
import { getPlatform } from "../../utils/api";

export default defineComponent({
  setup() {
    const route = useRoute();
    const router = useRouter();
    const { hasNewVersion } = useVersion();
    const { t } = i18n.global;

    const topMenus = [
      {
        routeName: "casual-chat",
        icon: CasualChatIcon,
        activeIcon: CasualActiveIcon,
        size: "2.0rem",
      },
      {
        routeName: "chat",
        icon: ChatIcon,
        activeIcon: ChatActiveIcon,
        size: "1.8rem",
      },
      {
        routeName: "prompt",
        icon: PromptIcon,
        activeIcon: PromptActiveIcon,
        size: "1.8rem",
      },
      {
        routeName: "promptMarket",
        icon: StoreIcon,
        activeIcon: StoreActiveIcon,
        size: "1.8rem",
      },
      {
        routeName: "model",
        icon: ModelIcon,
        activeIcon: ModelActiveIcon,
        size: "1.8rem",
      },
      {
        routeName: "plugin",
        icon: PluginIcon,
        activeIcon: PluginActiveIcon,
        size: "1.6rem",
      },
    ];

    const currentPlatform = useAsyncData(getPlatform);

    const isMacos = computed(() => currentPlatform.value === "darwin");

    function settingActionHandler(key: string) {
      switch (key) {
        case "setting": {
          router.push({
            name: "setting",
          });
          break;
        }
      }
    }

    const Setting = () => (
      <NBadge dot show={hasNewVersion.value}>
        <NIcon size="1.6rem" color="var(--switcher-color)">
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
