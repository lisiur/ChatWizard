import { Ref, ref, isRef, computed } from "vue";
import { useWait } from "./wait";

export function useLazyLoad<T>(
  load: (cursor?: string) => Promise<{
    records: T[];
    nextCursor: string | null;
  }>,
  indicator?: Element | Ref<Element | null>
) {
  const firstLoad = ref(false);
  const firstBatchLoad = ref(true);
  const records = ref<Array<T>>([]);
  const loading = ref(false);
  const cursor = ref<string>();
  const hasMore = computed(() => {
    return firstLoad.value === false || cursor.value !== undefined;
  });
  let destroy = null as null | (() => void);

  init();

  function init() {
    firstLoad.value = false;
    firstBatchLoad.value = true;
    cursor.value = undefined;
    records.value = [];
    destroy?.();
    if (indicator) {
      const { wait } = useWait({
        condition: () => (isRef(indicator) ? indicator.value !== null : true),
      });

      wait().then((done) => {
        if (done) {
          const ele = isRef(indicator)
            ? (indicator.value as Element)
            : indicator;
          const observer = new IntersectionObserver(async (entries) => {
            if (entries[0].intersectionRatio > 0) {
              await tryLoadNext(ele);
              firstBatchLoad.value = false;
            }
          });

          observer.observe(ele);
          destroy = () => observer.unobserve(ele);
        }
      });
    }
  }

  async function loadNext() {
    loading.value = true;
    try {
      const res = await load(cursor.value === "" ? undefined : cursor.value);
      cursor.value = res.nextCursor ?? undefined;
      records.value = ref(res.records).value.concat(records.value);

      if (cursor.value === undefined) {
        destroy?.();
      }
    } finally {
      loading.value = false;
    }
  }

  async function tryLoadNext(element: Element) {
    if (hasMore.value && elementVisible(element)) {
      firstLoad.value = true;
      await loadNext();
      await tryLoadNext(element);
    }
  }

  return {
    hasMore,
    loading,
    cursor,
    records,
    loadNext,
    firstBatchLoad,
    reset: init,
  };
}

function elementVisible(ele: Element) {
  const { top, bottom } = ele.getBoundingClientRect();
  return top <= window.innerHeight && bottom >= 0;
}
