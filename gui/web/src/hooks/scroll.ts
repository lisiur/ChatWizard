import { isRef, Ref, watch } from "vue";
import { ensure } from "../utils/ensure";

export function useAutoScroll(el: HTMLElement | Ref<HTMLElement>) {
  const interval = 20;
  let autoMode = true;
  let timer: NodeJS.Timer;
  let ele!: HTMLElement;
  if (isRef(el)) {
    let unwatch = watch(
      el,
      (e) => {
        if (e) {
          ele = e;
          ele.addEventListener("scroll", handleScroll);
          unwatch();
        }
      },
      {
        immediate: true,
      }
    );
  } else {
    ele = el;
    ele.addEventListener("scroll", handleScroll);
  }

  function resetAutoMode() {
    if (ele.scrollTop === ele.scrollHeight - ele.clientHeight) {
      autoMode = true;
    } else {
      autoMode = false;
    }
  }

  function handleScroll() {
    resetAutoMode();
  }

  function start() {
    autoMode = true;
    scrollToBottom();
    timer = setInterval(() => {
      if (autoMode) {
        scrollToBottom();
      }
    }, interval);
  }

  function stop() {
    clearTimeout(timer);
  }

  function destroy() {
    stop();
    ele.removeEventListener("scroll", handleScroll);
  }

  function scrollToBottom() {
    ele.scrollTop = ele.scrollHeight - ele.clientHeight;
  }

  return {
    start,
    stop,
    destroy,
    scrollToBottom,
  };
}

export function useScroll(el: HTMLElement | Ref<HTMLElement>) {
  let ele!: HTMLElement;
  ensure(() => {
    ele = isRef(el) ? el.value : el;
  }, [el]);

  let lastPosition = 0;

  function getCurrentPosition() {
    return ele.scrollHeight - ele.scrollTop;
  }

  function setPosition(position: number) {
    ele.scrollTop = ele.scrollHeight - position;
  }

  function savePosition() {
    lastPosition = getCurrentPosition();
  }

  function recoverPosition() {
    setPosition(lastPosition);
  }

  return {
    savePosition,
    recoverPosition,
  };
}
