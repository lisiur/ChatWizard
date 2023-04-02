import { NDropdown, NIcon, NScrollbar } from "naive-ui";
import { DropdownMixedOption } from "naive-ui/es/dropdown/src/interface";
import {
  computed,
  defineComponent,
  nextTick,
  PropType,
  Ref,
  ref,
  watch,
} from "vue";
import Draggable from "vuedraggable";
import { Archive as ArchiveIcon } from "@vicons/fa";
import { Pin as StickIcon } from '@vicons/tabler'

export type ExplorerMenuItem = DropdownMixedOption & {
  visible?: (data: ExplorerItem) => boolean;
  disabled?: ((data: ExplorerItem) => boolean) | boolean;
};
export interface ExplorerItem<T = any> {
  id: string;
  title: string;
  data: T;
}

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
        <div class="flex-1 overflow-hidden flex items-center select-none cursor-default">
          <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap">
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
    archivedList: {
      type: Array as PropType<ExplorerItem[]>,
      default: () => [],
    },
    menus: {
      type: Array as PropType<
        Array<
          | ExplorerMenuItem
          | null
          | ((item: ExplorerItem) => ExplorerMenuItem | null)
        >
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
    const stickVisible = ref(true);
    const archiveVisible = ref(false);
    const showStickIndicator = computed(() => props.stickList.length > 0);
    const showArchiveIndicator = computed(() => props.archivedList.length > 0);

    watch(
      () => props.active,
      (active) => {
        if (props.archivedList.find((item) => item.id === active)) {
          archiveVisible.value = true;
        }
        if (props.stickList.find((item) => item.id === active)) {
          stickVisible.value = true;
        }
      },
      {
        immediate: true,
      }
    );

    function renderColumn(data: ExplorerItem) {
      const menus = computed(() => {
        return props.menus
          .filter((item) => item !== null)
          .map((menu) => {
            if (typeof menu === "function") {
              return menu(data);
            } else {
              return menu;
            }
          })
          .filter((item) => item !== null)
          .filter((item) => {
            if (item?.visible) {
              return item?.visible(data);
            } else {
              return true;
            }
          })
          .map((item) => {
            if (typeof item?.disabled === "function") {
              item.disabled = item.disabled(data);
            }
            return item;
          });
      }) as Ref<DropdownMixedOption[]>;
      return (
        <Column
          active={props.active}
          item={data}
          menus={menus.value}
          onAction={(e) => props.onAction?.(e, data)}
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
      <div class="select-none cursor-default h-full">
        <NScrollbar>
          <div class="flex flex-col h-full">
            <div
              style={{
                backgroundColor: "var(--explorer-stick-bg-color)",
              }}
            >
              <Draggable
                v-show={stickVisible.value}
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
              <div
                v-show={showStickIndicator.value}
                onClick={() => (stickVisible.value = !stickVisible.value)}
                class="flex justify-center items-center p-1 text-gray-400"
                style={{
                  borderTopColor: "var(--border-color)",
                  borderTopStyle: "solid",
                  borderTopWidth: stickVisible.value ? "1px" : "0",
                }}
              >
                <NIcon>
                  <StickIcon />
                </NIcon>
                <span class="ml-1">
                  {stickVisible.value ? "Hide Pinned" : "Show Pinned"}
                </span>
              </div>
            </div>

            <div class="flex-1">
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
            </div>

            <div
              v-show={showArchiveIndicator.value}
              onClick={() => (archiveVisible.value = !archiveVisible.value)}
              style={{
                backgroundColor: "var(--explorer-archive-bg-color)",
              }}
            >
              <div
                class="flex justify-center items-center p-1 text-gray-400"
                style={{
                  borderBottomColor: "var(--border-color)",
                  borderBottomStyle: "solid",
                  borderBottomWidth: archiveVisible.value ? "1px" : "0",
                }}
              >
                <NIcon>
                  <ArchiveIcon />
                </NIcon>
                <span class="ml-1">
                  {archiveVisible.value ? "Hide Archived" : "Show Archived"}
                </span>
              </div>
              <div v-show={archiveVisible.value}>
                {props.archivedList.map((item) => renderColumn(item))}
              </div>
            </div>
          </div>
        </NScrollbar>
      </div>
    );
  },
});
