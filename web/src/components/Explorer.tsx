import { NCollapseTransition, NDropdown, NIcon, NScrollbar } from "naive-ui";
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
import Draggable from "./draggable/Draggable";
import { Archive as ArchiveIcon } from "@vicons/fa";
import { Pin as StickIcon } from "@vicons/tabler";
import { useI18n } from "../hooks/i18n";
import { SortableEvent } from "sortablejs";

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
    const { t } = useI18n();

    const stickVisible = ref(true);
    const unstickTransition = ref(true);

    watch(stickVisible, () => {
      unstickTransition.value = false;
      setTimeout(() => {
        unstickTransition.value = true;
      });
    });

    const archiveVisible = ref(false);
    const showStickIndicator = computed(() => props.stickList.length > 0);
    const showArchiveIndicator = computed(() => props.archivedList.length > 0);

    const stickList = ref<ExplorerItem[]>([]);
    watch(
      () => props.stickList,
      () => {
        stickList.value = [...props.stickList];
      },
      {
        immediate: true,
      }
    );
    const unstickList = ref<ExplorerItem[]>([]);
    watch(
      () => props.unstickList,
      () => {
        unstickList.value = [...props.unstickList];
      },
      {
        immediate: true,
      }
    );
    const archivedList = ref<ExplorerItem[]>([]);
    watch(
      () => props.archivedList,
      () => {
        archivedList.value = [...props.archivedList];
      },
      {
        immediate: true,
      }
    );

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
          data-id={`chat-${data.id}`}
          active={props.active}
          item={data}
          menus={menus.value}
          onAction={(e) => props.onAction?.(e, data)}
        ></Column>
      );
    }

    function stickListDragEndHandler(e: SortableEvent) {
      if (e.oldIndex === e.newIndex) return;
      props.onDarg?.(
        "stick",
        props.stickList[e.oldIndex!].id,
        props.stickList[e.newIndex!].id
      );
    }

    function unstickListDragEndHandler(e: SortableEvent) {
      if (e.oldIndex === e.newIndex) return;
      props.onDarg?.(
        "unstick",
        props.unstickList[e.oldIndex!].id,
        props.unstickList[e.newIndex!].id
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
              <NCollapseTransition show={stickVisible.value}>
                <Draggable
                  v-model={stickList.value}
                  options={{
                    group: "stickChats",
                    disabled: !props.onDarg,
                    onEnd: stickListDragEndHandler,
                  }}
                >
                  {{
                    item: ({ item }: any) => (
                      <div key={item.id}>{renderColumn(item)}</div>
                    ),
                  }}
                </Draggable>
              </NCollapseTransition>
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
                  {stickVisible.value
                    ? t("chat.explorer.hidePinned")
                    : t("chat.explorer.showPinned")}
                </span>
              </div>
            </div>

            <div class="flex-1">
              <Draggable
                v-model={unstickList.value}
                options={{
                  disabled: !props.onDarg,
                  onEnd: unstickListDragEndHandler,
                }}
                transition={unstickTransition.value}
              >
                {{
                  item: ({ item }: any) => renderColumn(item),
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
                  {archiveVisible.value
                    ? t("chat.explorer.hideArchived")
                    : t("chat.explorer.showArchived")}
                </span>
              </div>
            </div>
            <NCollapseTransition show={archiveVisible.value}>
              <Draggable
                v-model={archivedList.value}
                options={{
                  disabled: true,
                  onEnd: unstickListDragEndHandler,
                }}
              >
                {{
                  item: ({ item }: any) => renderColumn(item),
                }}
              </Draggable>
            </NCollapseTransition>
          </div>
        </NScrollbar>
      </div>
    );
  },
});
