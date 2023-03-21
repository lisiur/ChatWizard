import { computed, defineComponent, PropType } from "vue";
import { Chat } from "../models/chat";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
  },
  setup(props) {
    const cost = computed(() => props.chat.cost.value.toFixed(6));
    return () => <span>${cost.value}</span>;
  },
});
