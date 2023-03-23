import { computed, defineComponent } from "vue";

export default defineComponent({
  props: {
    value: {
      type: Number,
      default: 0,
    },
  },
  setup(props) {
    const cost = computed(() => props.value.toFixed(6));
    return () => <span>${cost.value}</span>;
  },
});
