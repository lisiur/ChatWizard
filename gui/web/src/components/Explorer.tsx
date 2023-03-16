import { NDropdown, NScrollbar } from "naive-ui";
import { computed, defineComponent, nextTick, PropType, ref } from "vue";

export default defineComponent({
  props: {
    active: {
      type: String,
    },
    list: {
      type: Array as PropType<{ id: string; title: string }[]>,
      default: () => [],
    },
    onAction: {
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
          {props.list?.map((item) => (
            <ChatColumn
              active={props.active}
              chat={item}
              onAction={(e) => props.onAction?.(e, item.id)}
            ></ChatColumn>
          ))}
        </NScrollbar>
      </div>
    );
  },
});

const ChatColumn = defineComponent({
  props: {
    active: String,
    chat: {
      type: Object as PropType<{ id: string; title: string }>,
      required: true,
    },
    onAction: {
      type: Function as PropType<(action: "select" | "delete") => void>,
    },
  },
  setup(props) {
    const x = ref(0);
    const y = ref(0);
    const showDropdown = ref(false);
    const options = computed(() => {
      return [
        {
          label: "Delete",
          key: "delete",
        },
      ];
    });

    function clickOutsideHandler() {
      showDropdown.value = false;
    }

    function dropdownHandler(key: "select" | "delete") {
      showDropdown.value = false;
      props.onAction?.(key);
    }

    function contextMenuHandler(e: MouseEvent) {
      e.preventDefault();
      showDropdown.value = false;
      nextTick().then(() => {
        showDropdown.value = true;
        x.value = e.clientX;
        y.value = e.clientY;
      });
    }

    return () => (
      <div
        class="flex items-center p-2"
        style={{
          color:
            props.active === props.chat.id
              ? "var(--explorer-active-color)"
              : "",
          backgroundColor:
            props.active === props.chat.id
              ? "var(--explorer-active-bg-color)"
              : "",
        }}
        onClick={() => props.onAction?.("select")}
        onContextmenu={contextMenuHandler}
      >
        <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap cursor-default">
          {props.chat.title}
        </div>
        <NDropdown
          trigger="manual"
          placement="bottom-start"
          x={x.value}
          y={y.value}
          options={options.value}
          show={showDropdown.value}
          onClickoutside={clickOutsideHandler}
          onSelect={dropdownHandler}
        ></NDropdown>
      </div>
    );
  },
});
