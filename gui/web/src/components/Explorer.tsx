import { NIcon, NScrollbar } from "naive-ui";
import { Trash } from "@vicons/fa";
import { defineComponent, PropType } from "vue";

export default defineComponent({
  props: {
    active: {
      type: String,
    },
    list: {
      type: Array as PropType<{ id: string; title: string }[]>,
      default: () => [],
    },
    handler: {
      type: Function as PropType<
        (action: "select" | "delete", chatId: string) => void
      >,
    },
  },
  setup(props) {
    return () => (
      <div
        class="select-none"
        style="background-color: var(--explorer-bg-color); color: var(--explorer-color)"
      >
        <NScrollbar>
          {props.list?.map((chat) => (
            <div
              class="flex items-center p-2"
              style={{
                color:
                  props.active === chat.id
                    ? "var(--explorer-active-color)"
                    : "",
                backgroundColor:
                  props.active === chat.id
                    ? "var(--explorer-active-bg-color)"
                    : "",
              }}
            >
              <div
                class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap cursor-default"
                onClick={() => props.handler?.("select", chat.id)}
              >
                {chat.title}
              </div>
              <span
                class="ml-auto"
                onClick={() => props.handler?.("delete", chat.id)}
              >
                <NIcon color="var(--error-color)">
                  <Trash />
                </NIcon>
              </span>
            </div>
          ))}
        </NScrollbar>
      </div>
    );
  },
});
