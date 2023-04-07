import { EventCallback } from "@tauri-apps/api/event";
import { v4 as uuid } from "uuid";
import { execCommand } from "./api";
import { isWeb } from "./env";

export const clientId = uuid();

const eventCallbacks: Record<string, EventCallback<any>[]> = {};

export function listen<T>(id: string, eventHandler: EventCallback<T>) {
  if (!eventCallbacks[id]) {
    eventCallbacks[id] = [];
  }
  eventCallbacks[id].push(eventHandler);

  return () => {
    eventCallbacks[id] = eventCallbacks[id].filter(
      (item) => item !== eventHandler
    );
  };
}

function init() {
  const websocket = new WebSocket(
    `ws://${window.location.host}/api/ws?clientId=${clientId}`
  );

  websocket.send(JSON.stringify({ type: "connect" }));

  websocket.onmessage = (event) => {
    const { id, payload } = JSON.parse(event.data);
    if (eventCallbacks[id]) {
      eventCallbacks[id].forEach((callback) =>
        callback({
          event: '',
          windowLabel: '',
          id,
          payload,
        })
      );
    }
  };
}

if (isWeb) {
  init();
}
