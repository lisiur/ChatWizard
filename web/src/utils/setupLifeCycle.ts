import { onMounted, onBeforeUnmount } from "vue";

interface Context {
  onMounted: (fn: (ctx: Context) => any) => Context;
  onBeforeUnmount: (fn: (ctx: Context) => any) => Context;
  setup: () => any;
}

export function setupLifeCycle() {
  const onMountedCbs: any[] = [];
  const onBeforeUnmountCbs: any[] = [];

  const context: Context = {
    onMounted: _onMounted,
    onBeforeUnmount: _onBeforeUnmount,
    setup,
  };

  function triggerOnMounted() {
    onMountedCbs.forEach((item) => item(context));
  }
  function triggerOnBeforeUnmounted() {
    onBeforeUnmountCbs.forEach((item) => item(context));
  }

  function _onMounted(fn: (ctx: Context) => any) {
    onMountedCbs.push(fn);
    return context;
  }

  function _onBeforeUnmount(fn: (ctx: Context) => any) {
    onBeforeUnmountCbs.push(fn);
    return context;
  }

  function setup() {
    onMounted(triggerOnMounted);
    onBeforeUnmount(triggerOnBeforeUnmounted);
  }

  return context;
}
