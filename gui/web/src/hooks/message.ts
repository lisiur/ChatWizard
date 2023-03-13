import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { ref, reactive } from "vue";

export type Message = UserMessage | AssistantMessage | ErrorMessage;

export interface UserMessage {
  id: string;
  role: "user" | "assistant" | "system";
  content: string;
  delivered: boolean | null;
}

export interface AssistantMessage {
  role: "assistant";
  content: string;
  pending: boolean;
  done: boolean;
}

export type MessageChunk =
  | {
      type: "error";
      data: ErrorData;
    }
  | {
      type: "data";
      data: string;
    }
  | {
      type: "done";
    };

type ErrorData =
  | {
      type: "network";
      data: {
        type: "timeout" | "unknown";
        message: string;
      };
    }
  | {
      type: "api";
      data: {
        code: string;
        type: string;
        message: string;
      };
    };

export interface ErrorMessage {
  role: "error";
  error: ErrorData;
}

function useAssistantMessage() {
  let index = 0;
  const values = [".&nbsp;&nbsp;", "..&nbsp;", "..."];
  const message = reactive<AssistantMessage>({
    role: "assistant",
    content: values[index],
    pending: true,
    done: false,
  });

  const interval = setInterval(() => {
    index += 1;
    index = index % values.length;
    message.content = values[index];
  }, 500);

  function markReady() {
    if (!message.pending) {
      return;
    }
    message.pending = false;
    clearInterval(interval);
    message.content = "";
  }

  return {
    message,
    markReady,
  };
}

export function useTopic() {
  const idle = ref(true);

  const topic = reactive({
    messages: [] as Array<Message>,
  });

  const prompt = ref("");

  function handleAssistantMessageChunk(params: {
    userMessage: UserMessage;
    assistantMessage: AssistantMessage;
    chunk: MessageChunk;
    doneCallback: () => void;
  }) {
    switch (params.chunk.type) {
      case "error": {
        idle.value = true;
        topic.messages.pop();
        topic.messages.push({
          role: "error",
          error: params.chunk.data,
        });
        params.userMessage.delivered = false;
        break;
      }
      case "data": {
        params.assistantMessage.content += params.chunk.data;
        params.userMessage.delivered = true;
        break;
      }
      case "done": {
        idle.value = true;
        params.assistantMessage.done = true;
        params.doneCallback();
        break;
      }
    }
  }

  async function sendMessage() {
    if (!idle.value) {
      return;
    }

    const msg = prompt.value;
    prompt.value = "";

    const userMessage: UserMessage = {
      id: "",
      role: "user",
      content: msg,
      delivered: null,
    };
    topic.messages.push(userMessage);

    const { message: assistantMessage, markReady } = useAssistantMessage();
    topic.messages.push(assistantMessage);

    idle.value = false;
    const eventId: string = await invoke("send_message", { message: msg });
    userMessage.id = eventId;
    const unListen = await listen(eventId, (event) => {
      markReady();
      handleAssistantMessageChunk({
        userMessage,
        assistantMessage,
        chunk: event.payload as MessageChunk,
        doneCallback: unListen,
      });
    });
  }

  async function resendMessage(id: string) {
    const index = topic.messages.findIndex(
      (msg) => (msg as UserMessage).id === id
    );
    topic.messages.length = index + 1;

    const userMessage = topic.messages[index] as UserMessage
    userMessage.delivered = null

    const { message: assistantMessage, markReady } = useAssistantMessage();
    topic.messages.push(assistantMessage);

    const eventId: string = await invoke("resend_message", { id });
    const unListen = await listen(eventId, (event) => {
      markReady();
      handleAssistantMessageChunk({
        userMessage,
        assistantMessage,
        chunk: event.payload as MessageChunk,
        doneCallback: unListen,
      });
    });
  }

  async function reset() {
    await invoke("reset_topic");
    topic.messages = [];
  }

  return {
    prompt,
    sendMessage,
    resendMessage,
    reset,
    topic,
    idle,
  };
}
