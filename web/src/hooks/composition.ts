import { Ref, ref, watch } from "vue";

export function useComposition(element?: Ref<HTMLElement | undefined>) {
  const isComposition = ref(false);
  let timer: NodeJS.Timeout;

  function compositionStartHandler(_e: CompositionEvent) {
    clearTimeout(timer);
    isComposition.value = true;
  }

  function compositionEndHandler(_e: CompositionEvent) {
    timer = setTimeout(() => {
      isComposition.value = false;
    }, 100);
  }

  if (element) {
    watch(
      element,
      () => {
        if (element?.value) {
          element.value.addEventListener(
            "compositionstart",
            compositionStartHandler
          );
          element.value.addEventListener(
            "compositionend",
            compositionEndHandler
          );
        }
      },
      {
        immediate: true,
      }
    );
  }

  return {
    isComposition,
    compositionStartHandler,
    compositionEndHandler,
  };
}
