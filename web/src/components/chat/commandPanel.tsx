import { PropType, defineComponent } from "vue";

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
    const publicInstance = {};

    expose(publicInstance);

    return (() => (
      <div class="border bg-[var(--command-panel-bg-color)] rounded-md">
        {props.list.map((item, index) => {
          const isSelected = props.selected === index;
          return (
            <div class={[isSelected ? "bg-primary" : "", "py-1 px-2"]}>
              {item.label}
            </div>
          );
        })}
      </div>
    )) as unknown as typeof publicInstance;
  },
});
