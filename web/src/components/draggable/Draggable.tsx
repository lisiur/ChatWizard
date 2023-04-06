import "./style.css";
import { PropType, defineComponent, onMounted, ref, watch } from "vue";
import Sortable from "sortablejs";
import ListTransition from "../listTransition/listTransition";

export default defineComponent({
  props: {
    modelValue: {
      type: Array as PropType<{ id: string; [key: string]: any }[]>,
      default: () => [],
    },
    options: {
      type: Object as PropType<Sortable.Options>,
      default: () => ({}),
    },
    transition: {
      type: Boolean,
      default: true,
    },
  },
  emits: ["update:modelValue"],
  setup(props, { expose, slots }) {
    let sortable!: Sortable;
    const wrapperRef = ref<InstanceType<typeof ListTransition>>();

    const publicInstance = {};
    expose(publicInstance);

    onMounted(init);

    watch(() => props.options, init);

    function init() {
      sortable?.destroy();
      sortable = Sortable.create(wrapperRef.value!.$el, {
        animation: 150,
        handle: ".sortable-drag-handle",
        draggable: ".sortable-item",
        ghostClass: "sortable-ghost",
        chosenClass: "sortable-chosen",
        dragClass: "sortable-dragging",
        ...props.options,
      });
    }

    return (() => (
      <ListTransition ref={wrapperRef} disabled={!props.transition}>
        {props.modelValue.map((item, index) => (
          <div class="sortable-drag-handle sortable-item" key={item.id}>
            {slots.item?.({ item, index })}
          </div>
        ))}
      </ListTransition>
    )) as unknown as typeof publicInstance;
  },
});
