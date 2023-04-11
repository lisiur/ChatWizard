import { NScrollbar } from "naive-ui";
import { PropType, defineComponent, nextTick, watch } from "vue";

export default defineComponent({
  props: {
    list: {
      type: Array as PropType<Array<{ value: string; label: string }>>,
      default: () => [],
    },
    selected: {
      type: Number,
      default: 0,
    },
  },
  setup(props, { expose }) {
    watch(
      () => props.selected,
      () => {
        setTimeout(() => {
          document
            .querySelector(`#prompt-${props.list[props.selected].value}`)
            ?.scrollIntoView({
              inline: "center",
              block: "center",
              behavior: "smooth",
            });
        });
      }
    );

    const publicInstance = {};

    expose(publicInstance);

    return (() => (
      <div class="border bg-[var(--command-panel-bg-color)] rounded-md overflow-hidden p-1">
        <NScrollbar class="max-h-[16rem]">
          <div class="h-full">
            {props.list.map((item, index) => {
              const isSelected = props.selected === index;
              return (
                <div
                  id={`prompt-${item.value}`}
                  class={[isSelected ? "bg-primary" : "", "py-1 px-2"]}
                >
                  {item.label}
                </div>
              );
            })}
          </div>
        </NScrollbar>
      </div>
    )) as unknown as typeof publicInstance;
  },
});
