import { defineComponent } from "vue";

export default defineComponent({
  props: {
    title: String,
  },
  setup(props, { slots }) {
    return () => (
      <div
        class="px-4 py-3 border-b border-color flex items-center"
        data-tauri-drag-region
      >
        <span
          class="text-lg flex-1 overflow-hidden text-ellipsis whitespace-nowrap"
          data-tauri-drag-region
        >
          {props.title}
        </span>
        {slots["right-panel"]?.()}
      </div>
    );
  },
});
