import { defineComponent, TransitionGroup } from "vue";
import "./style.css";

export default defineComponent({
  props: {
    disabled: {
      type: Boolean,
      default: false,
    },
    absolute: {
      type: Boolean,
      default: true,
    },
  },
  setup(props, { slots }) {
    return () => (
      <TransitionGroup
        tag="div"
        type="transition"
        name={
          props.disabled
            ? ""
            : props.absolute
            ? "list-transition"
            : "list-transition-without-absolute"
        }
      >
        {slots.default?.()}
      </TransitionGroup>
    );
  },
});
