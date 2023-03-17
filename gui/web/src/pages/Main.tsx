import { defineComponent } from "vue";
import { RouterView, useRoute, useRouter } from "vue-router";
import Version from "../components/Version";
import {
  ChatbubbleEllipsesOutline as InactiveChatIcon,
  ChatbubbleEllipses as ActiveChatIcon,
} from "@vicons/ionicons5";

import { Prompt as PromptIcon } from "@vicons/tabler";
import { NIcon } from "naive-ui";

export default defineComponent({
  setup() {
    const route = useRoute();
    const router = useRouter();

    const menus = [
      {
        name: "chat",
        url: router.resolve({ name: "chat" }).href,
        icon: InactiveChatIcon,
        activeIcon: ActiveChatIcon,
      },
      {
        name: "prompt",
        url: router.resolve({ name: "prompt" }).href,
        icon: PromptIcon,
        activeIcon: PromptIcon,
      },
    ];

    return () => (
      <div class="h-full flex">
        <div
          class="w-16 border-r h-full flex flex-col"
          style="background-color: var(--switcher-bg-color); border-color: var(--border-color)"
        >
          <div class="grid gap-1 place-content-center">
            {menus.map((m) => {
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
          <div class="mt-auto flex justify-center">
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
