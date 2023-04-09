import { defineComponent, onMounted, ref, watch, watchEffect } from "vue";

export default defineComponent({
  props: {
    title: String,
    disabled: Boolean,
  },
  setup(props, { slots }) {
    const dragRegion = ref<HTMLElement>();

    onMounted(() => {
      watchEffect(() => {
        if (!props.disabled) {
          dragRegion.value!.dataset["tauriDragRegion"] = "true";
        } else {
          delete dragRegion.value!.dataset["tauriDragRegion"];
        }
      });
    });
    return () => (
      <div class="border-b border-color flex items-center">
        <span
          ref={dragRegion}
          class="px-4 py-3 text-lg flex-1 overflow-hidden text-ellipsis whitespace-nowrap cursor-default"
        >
          {props.title}
        </span>
        <span class="pr-4 flex items-center">{slots["right-panel"]?.()}</span>
      </div>
    );
  },
});
