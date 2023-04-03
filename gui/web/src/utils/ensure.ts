import { isRef, watch } from "vue";

export async function ensure<T extends () => any>(
  fn: T,
  deps: Array<any>
): Promise<ReturnType<T>> {
  return new Promise((resolve) => {
    let waiting = [...deps.filter((item) => isRef(item))];
    deps.forEach((dep) => {
      let unwatch: any;
      unwatch = watch(
        dep,
        () => {
          if (dep.value !== undefined && dep.value !== null) {
            let index = waiting.findIndex((item) => item === dep);
            if (index !== -1) {
              waiting.splice(index, 1);
              if (unwatch) {
                unwatch();
              }
            } else {
              if (unwatch) {
                unwatch();
              }
            }

            if (waiting.length === 0) {
              resolve(fn());
            }
          }
        },
        {
          immediate: true,
        }
      );
    });
  });
}
