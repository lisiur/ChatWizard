import { Ref, ref, isRef } from "vue";
import { useWait } from "./wait";

export function useLazyLoad<T>(
  load: (cursor: string | null) => Promise<{
    records: T[];
    nextCursor: string | null;
  }>,
  indicator?: Element | Ref<Element | null>
) {
  const records = ref<Array<T>>([]);
  const loading = ref(false);
  const cursor = ref<string | null>("");
  let destroy = null as null | (() => void);

  init();

  function init() {
    cursor.value = "";
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
              loading.value = true;
              setTimeout(() => {
                tryLoadNext(ele);
              }, 300);
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
      const res = await load(cursor.value);
      cursor.value = res.nextCursor;
      records.value = ref(res.records).value.concat(records.value);
    } finally {
      loading.value = false;
    }
  }

  async function tryLoadNext(element: Element) {
    if (cursor.value && elementVisible(element)) {
      await loadNext();

      // wait for the element to be rendered from the DOM
      await sleep();

      await tryLoadNext(element);
    }
  }

  return {
    hasMore: cursor,
    loading,
    records,
    loadNext,
    reload: init,
  };
}

async function sleep(ms = 0) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function elementVisible(ele: Element) {
  const { top, bottom } = ele.getBoundingClientRect();
  return top <= window.innerHeight && bottom >= 0;
}
