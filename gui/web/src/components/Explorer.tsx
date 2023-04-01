import { NDropdown, NScrollbar } from "naive-ui";
import { DropdownMixedOption } from "naive-ui/es/dropdown/src/interface";
import { computed, defineComponent, nextTick, PropType, ref } from "vue";
import Draggable from "vuedraggable";

export type ExplorerMenuItem = DropdownMixedOption;
export interface ExplorerItem<T = any> {
  id: string;
  title: string;
  data: T;
}

export default defineComponent({
  props: {
    active: {
      type: String,
    },
    unstickList: {
      type: Array as PropType<ExplorerItem[]>,
      default: () => [],
    },
    stickList: {
      type: Array as PropType<ExplorerItem[]>,
      default: () => [],
    },
    menus: {
      type: Array as PropType<
        Array<ExplorerMenuItem | ((item: ExplorerItem) => ExplorerMenuItem)>
      >,
      default: () => [],
    },
    onAction: {
      type: Function as PropType<
        (action: "select" | string, item: ExplorerItem) => void
      >,
    },
    onDarg: {
      type: Function as PropType<
        (group: "stick" | "unstick", from: string, to: string) => void
      >,
    },
  },
  setup(props) {
    function renderColumn(item: ExplorerItem) {
      const menus = computed(() => {
        return props.menus.map((menu) => {
          if (typeof menu === "function") {
            return menu(item);
          } else {
            return menu;
          }
        });
      });
      return (
        <Column
          active={props.active}
          item={item}
          menus={menus.value}
          onAction={(e) => props.onAction?.(e, item)}
        ></Column>
      );
    }

    function stickListDragEndHandler(e: {
      oldDraggableIndex: number;
      newDraggableIndex: number;
    }) {
      if (e.oldDraggableIndex === e.newDraggableIndex) return;
      props.onDarg?.(
        "stick",
        props.stickList[e.oldDraggableIndex].id,
        props.stickList[e.newDraggableIndex].id
      );
    }

    function unstickListDragEndHandler(e: {
      oldDraggableIndex: number;
      newDraggableIndex: number;
    }) {
      if (e.oldDraggableIndex === e.newDraggableIndex) return;
      props.onDarg?.(
        "unstick",
        props.unstickList[e.oldDraggableIndex].id,
        props.unstickList[e.newDraggableIndex].id
      );
    }
    return () => (
      <div class="select-none">
        <NScrollbar>
          <div
            style={{
              backgroundColor: "var(--explorer-stick-bg-color)",
            }}
          >
            <Draggable
              v-model={props.stickList}
              itemKey="id"
              //@ts-ignore
              disabled={!props.onDarg}
              //@ts-ignore
              onEnd={stickListDragEndHandler}
            >
              {{
                item: ({ element }: any) => renderColumn(element),
              }}
            </Draggable>
          </div>
          <Draggable
            v-model={props.unstickList}
            itemKey="id"
            //@ts-ignore
            disabled={!props.onDarg}
            //@ts-ignore
            onEnd={unstickListDragEndHandler}
          >
            {{
              item: ({ element }: any) => renderColumn(element),
            }}
          </Draggable>
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
          <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap select-none">
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
