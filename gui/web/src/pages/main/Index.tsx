import { defineComponent } from "vue";
import { RouterView, useRoute, useRouter } from "vue-router";
import Version from "../../components/Version";
import {
  ChatbubbleEllipsesOutline as InactiveChatIcon,
  ChatbubbleEllipses as ActiveChatIcon,
  Settings as SettingIcon,
} from "@vicons/ionicons5";

import { Prompt as PromptIcon } from "@vicons/tabler";
import { NIcon } from "naive-ui";
import { WebviewWindow } from "@tauri-apps/api/window";
import { message } from "../../utils/prompt";
import { showWindow } from "../../api";

export default defineComponent({
  setup() {
    const route = useRoute();
    const router = useRouter();

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

    const bottomMenus = [
      {
        title: "Setting",
        name: "setting",
        url: router.resolve({ name: "setting" }).path,
        icon: SettingIcon,
      },
    ];

    function actionHandler(action: typeof bottomMenus[0]) {
      showWindow("setting", {
        title: action.title,
        url: `/#${action.url}`,
        width: 500,
        height: 400,
      });
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-16 border-r h-full flex flex-col"
          style="background-color: var(--switcher-bg-color); border-color: var(--border-color)"
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
          <div class="mt-auto flex justify-center items-center flex-col">
            <div class="grid gap-1 place-content-center">
              {bottomMenus.map((m) => {
                const Icon = m.icon;
                return (
                  <div class="mt-4" onClick={() => actionHandler(m)}>
                    <NIcon size="2rem" color="var(--switcher-color)">
                      <Icon></Icon>
                    </NIcon>
                  </div>
                );
              })}
            </div>
            <Version></Version>
          </div>
        </div>
        <div class="flex-1 overflow-hidden">
          <RouterView />
        </div>
      </div>
    );
  },
});
