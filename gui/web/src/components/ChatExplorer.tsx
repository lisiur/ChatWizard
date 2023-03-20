import { NDropdown, NScrollbar } from "naive-ui";
import { computed, defineComponent, nextTick, PropType, ref } from "vue";
import { ChatMetadata } from "../api";
import { useI18n } from "../hooks/i18n";

export default defineComponent({
  props: {
    active: {
      type: String,
    },
    list: {
      type: Array as PropType<ChatMetadata[]>,
      default: () => [],
    },
    onAction: {
      type: Function as PropType<
        (action: "select" | "delete" | "rename", chat: ChatMetadata) => void
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
              chat={item}
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
    chat: {
      type: Object as PropType<ChatMetadata>,
      required: true,
    },
    onAction: {
      type: Function as PropType<
        (action: "select" | "delete" | "rename") => void
      >,
    },
  },
  setup(props) {
    const { t } = useI18n();

    const x = ref(0);
    const y = ref(0);
    const showDropdown = ref(false);
    const options = computed(() => {
      return [
        {
          label: t("chat.rename"),
          key: "rename",
        },
        {
          type: "divider",
        },
        {
          label: t("common.delete"),
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
        <div class="flex-1 overflow-hidden flex items-center cursor-default">
          <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap ">
            {props.chat.title || t("chat.new.defaultTitle")}
          </div>
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
