import { cloneDeep } from "lodash-es";
import { reactive } from "vue";

export function useState<T extends object>(state: T) {
  const innerState = reactive(state);
  const initState = cloneDeep(innerState);
  function reset() {
    Object.assign(innerState, cloneDeep(initState));
  }
  function resetField(filed: keyof typeof innerState) {
    innerState[filed] = cloneDeep(initState[filed]);
  }
  return {
    state: innerState,
    resetState: reset,
    resetField,
  };
}
