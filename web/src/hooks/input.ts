import { Ref, UnwrapRef, ref, watch } from "vue";
import { useComposition } from "./composition";
import { ensure } from "../utils/ensure";

export interface Input {
  composition: boolean;
  key: string;
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  event: KeyboardEvent;
}

export interface InputOptions<State = any> {
  dom: Ref<HTMLElement | undefined>;
  inputWatcher?: (input: string) => void;
  defaultState?: State;
  stateTransition?: (
    state: UnwrapRef<State>,
    input: Input
  ) => UnwrapRef<State | undefined> | Promise<UnwrapRef<State | undefined>>;
}

export function useInput<State = any>(options: InputOptions<State>) {
  const { isComposition } = useComposition(options.dom);

  const input = ref("");
  const state = ref(options.defaultState);

  watch(input, (newInput) => options.inputWatcher?.(newInput));

  ensure(() => {
    const dom = options.dom.value!;
    dom.addEventListener("keydown", handleInput);
  }, [options.dom]);

  async function handleInput(e: KeyboardEvent) {
    const charInput = {
      composition: isComposition.value,
      key: e.key,
      ctrl: e.ctrlKey,
      alt: e.altKey,
      shift: e.shiftKey,
      event: e,
    };
    if (state.value) {
      const newState = await options.stateTransition?.(state.value, charInput);
      if (newState) {
        state.value = newState;
      }
    }
  }

  function setState(newState: UnwrapRef<State>) {
    state.value = newState;
  }

  function focus() {
    options.dom.value?.focus();
  }

  return {
    input,
    state,
    setState,
    focus,
  };
}
