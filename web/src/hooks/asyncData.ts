import {
  computed,
  ComputedRef,
  Ref,
  ref,
  UnwrapRef,
  watch,
} from "vue";

export function useAsyncData<T>(
  asyncGetter: () => Promise<UnwrapRef<T>>,
  initValue?: T
): Ref<UnwrapRef<T>> {
  const value = ref<T | undefined>(initValue);

  asyncGetter().then((newVal) => {
    value.value = newVal;
  });

  return value as Ref<UnwrapRef<T>>;
}

export function useAsyncDataReactiveWithDefault<T>(
  asyncGetter: () => Promise<T> | undefined,
  deps: Parameters<typeof watch>[0],
  defaultValue: T
): Ref<T> {
  return useAsyncDataReactive(asyncGetter, deps, defaultValue) as Ref<T>;
}

export function useAsyncDataReactive<T>(
  asyncGetter: () => Promise<T> | undefined,
  deps: Parameters<typeof watch>[0],
  defaultValue?: T
): Ref<T | undefined> {
  const value = ref(defaultValue) as Ref<T | undefined>;

  watch(
    deps,
    () => {
      const res = asyncGetter();
      if (res) {
        res.then((newVal) => {
          value.value = newVal;
        });
      } else {
        value.value = defaultValue;
      }
    },
    {
      immediate: true,
    }
  );

  return value;
}

export function useAsyncDataLazy<T>(
  asyncGetter: () => Promise<UnwrapRef<T>>,
  initValue?: T
) {
  let loaded = false;
  const fetched = ref(false);
  const value = ref<T | undefined>(initValue);

  function loadData() {
    if (loaded) {
      return;
    } else {
      loaded = true;
    }

    asyncGetter().then((newVal) => {
      value.value = newVal;
      fetched.value = true;
    });
  }

  function reloadData() {
    loaded = true;
    asyncGetter().then((newVal) => {
      value.value = newVal;
      fetched.value = true;
    });
  }

  async function fetchData(): Promise<UnwrapRef<T>> {
    return new Promise((resolve) => {
      const unwatch = watch(
        fetched,
        () => {
          if (fetched.value) {
            unwatch();
            resolve(value.value as UnwrapRef<T>);
          }
        },
        {
          immediate: true,
        }
      );
    });
  }

  return {
    data: value as Ref<UnwrapRef<T>>,
    loadData,
    reloadData,
    fetchData,
  };
}

export function useAsyncArrayData<T>(
  asyncGetter: () => Promise<UnwrapRef<Array<T>>>,
  initValue?: Array<T>
): { data: Ref<UnwrapRef<Array<T>>>; empty: ComputedRef<boolean> } {
  const loaded = ref(false);
  const value = ref<Array<T>>(initValue ?? []);
  const empty = computed(() => {
    return loaded.value && value.value.length === 0;
  });

  asyncGetter().then((newVal) => {
    value.value = newVal;
    loaded.value = true;
  });

  return {
    data: value as Ref<UnwrapRef<Array<T>>>,
    empty,
  };
}
