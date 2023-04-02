import { computed, defineComponent, PropType, ref, watch } from "vue";
import { History as HistoryIcon } from "@vicons/fa";
import { NButton, NIcon, NInputNumber, NPopover } from "naive-ui";
import { Chat } from "../../models/chat";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
  },
  setup(props) {
    const currentBacktrack = computed(() => props.chat.index.config.backtrack);
    const backtrack = ref(currentBacktrack.value);
    watch(
      () => props.chat.index.config.backtrack,
      (value) => {
        backtrack.value = value;
      }
    );

    function updateBacktrack(visible: boolean) {
      if (!visible) {
        props.chat.updateBacktrack(backtrack.value);
      }
    }

    return () => (
      <span>
        <NPopover class="w-28" trigger="click" onUpdateShow={updateBacktrack}>
          {{
            trigger: () => (
              <NButton tertiary size="tiny">
                <NIcon size={10}>
                  <HistoryIcon></HistoryIcon>
                </NIcon>
                <span class="ml-[.1rem]">{currentBacktrack.value}</span>
              </NButton>
            ),
            default: () => (
              <NInputNumber
                v-model:value={backtrack.value}
                buttonPlacement="both"
                size="tiny"
                min={0}
                class="text-center"
              ></NInputNumber>
            ),
          }}
        </NPopover>
      </span>
    );
  },
});
