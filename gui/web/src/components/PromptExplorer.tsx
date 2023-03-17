import { NDropdown, NScrollbar } from "naive-ui";
import { computed, defineComponent, nextTick, PropType, ref } from "vue";

export default defineComponent({
  props: {
    active: {
      type: String,
    },
    list: {
      type: Array as PropType<{ act: string }[]>,
      default: () => [],
    },
    onAction: {
      type: Function as PropType<
        (action: "select" | "delete" | "newChat", chatId: string) => void
      >,
    },
  },
  setup(props) {
    return () => (
      <div class="select-none">
        <NScrollbar>
          {props.list?.map((item) => (
            <Column
              active={props.active}
              prompt={item}
              onAction={(e) => props.onAction?.(e, item.act)}
            ></Column>
          ))}
        </NScrollbar>
      </div>
    );
  },
});

const Column = defineComponent({
  props: {
    active: String,
    prompt: {
      type: Object as PropType<{ act: string }>,
      required: true,
    },
    onAction: {
      type: Function as PropType<(action: "select" | "delete" | "newChat") => void>,
    },
  },
  setup(props) {
    const x = ref(0);
    const y = ref(0);
    const showDropdown = ref(false);
    const options = computed(() => {
      return [
        {
          label: "New Chat",
          key: "newChat",
        },
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
        class="flex items-center py-2 px-4"
        style={{
          color:
            props.active === props.prompt.act
              ? "var(--explorer-active-color)"
              : "",
          backgroundColor:
            props.active === props.prompt.act
              ? "var(--explorer-active-bg-color)"
              : "",
        }}
        onClick={() => props.onAction?.("select")}
        onContextmenu={contextMenuHandler}
      >
        <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap cursor-default">
          {props.prompt.act}
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
