import { NDropdown, NScrollbar } from "naive-ui";
import { DropdownMixedOption } from "naive-ui/es/dropdown/src/interface";
import { defineComponent, nextTick, PropType, ref } from "vue";

export type ExplorerMenuItem = DropdownMixedOption;
export interface ExplorerItem {
  id: string;
  title: string;
}

export default defineComponent({
  props: {
    active: {
      type: String,
    },
    list: {
      type: Array as PropType<ExplorerItem[]>,
      default: () => [],
    },
    menus: {
      type: Array as PropType<ExplorerMenuItem[]>,
      default: () => [],
    },
    onAction: {
      type: Function as PropType<
        (action: "select" | string, item: ExplorerItem) => void
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
              item={item}
              menus={props.menus}
              onAction={(e) => props.onAction?.(e, item)}
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
    item: {
      type: Object as PropType<ExplorerItem>,
      required: true,
    },
    menus: {
      type: Array as PropType<ExplorerMenuItem[]>,
      default: () => [],
    },
    onAction: {
      type: Function as PropType<(action: string) => void>,
    },
  },
  setup(props) {
    const x = ref(0);
    const y = ref(0);
    const showDropdown = ref(false);

    function clickOutsideHandler() {
      showDropdown.value = false;
    }

    function dropdownHandler(key: string) {
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
            props.active === props.item.id
              ? "var(--explorer-active-color)"
              : "",
          backgroundColor:
            props.active === props.item.id
              ? "var(--explorer-active-bg-color)"
              : "",
        }}
        onClick={() => props.onAction?.("select")}
        onContextmenu={contextMenuHandler}
      >
        <div class="flex-1 overflow-hidden flex items-center cursor-default">
          <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap ">
            {props.item.title}
          </div>
        </div>
        <NDropdown
          trigger="manual"
          placement="bottom-start"
          x={x.value}
          y={y.value}
          options={props.menus}
          show={showDropdown.value}
          onClickoutside={clickOutsideHandler}
          onSelect={dropdownHandler}
        ></NDropdown>
      </div>
    );
  },
});
