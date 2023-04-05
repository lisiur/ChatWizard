import { ref } from "vue";
import { sleep } from "../utils/sleep";

export interface UseWaitConfig {
  condition: () => boolean;
  interval?: number;
  maxWait?: number;
}

export function useWait(config: UseWaitConfig) {
  const ready = ref(false);
  const timeout = ref(false);
  async function wait() {
    const interval = setTimeout(() => {
      timeout.value = true;
    }, config.maxWait ?? 60 * 1000);

    while (!config.condition()) {
      if (timeout.value) {
        timeout.value = true;
        return false;
      }
      await sleep(config.interval ?? 16.67);
    }
    clearInterval(interval);
    ready.value = true;
    return true;
  }

  return {
    wait,
    ready,
    timeout,
  };
}
