import { listen } from "@tauri-apps/api/event";
import { reactive } from "vue";
import {
  resendMessage,
  sendMessage,
  saveAsMarkdown,
  ChatData,
  ChatConfig,
} from "../api";
import {
  AssistantMessage,
  ErrorMessage,
  Message,
  MessageChunk,
  UserMessage,
} from "./message";

export class Chat {
  busy: boolean;
  messages: Message[];
  config: ChatConfig;
  constructor(public id: string, messages: Message[] = [], config: ChatConfig = {}) {
    this.busy = false;
    this.messages = reactive(messages);
    this.config = reactive(config);
  }

  static init(id: string, data: ChatData) {
    const messages: Message[] = [];
    for (let i = 0; i < data.logs.length; i++) {
      const log = data.logs[i];
      switch (log.message.role) {
        case "user": {
          const msg = new UserMessage(log.message.content);
          msg.setId(log.id);
          msg.finished = data.logs[i + 1]?.message.role === "assistant";
          msg.markHistory();
          messages.push(msg);
          break;
        }
        case "assistant": {
          const msg = new AssistantMessage(log.message.content);
          msg.markHistory();
          messages.push(msg);
          break;
        }
      }
    }

    return new Chat(id, messages, data.config);
  }

  async sendMessage(message: string, params?: { onFinish?: () => void }) {
    const userMessage = reactive(new UserMessage(message));
    this.messages.push(userMessage);

    const messageId = await sendMessage(this.id, message);
    userMessage.setId(messageId);

    this.__receiveAssistantMessage(userMessage, params);

    return messageId;
  }

  async resendMessage(messageId: string, params?: { onFinish?: () => void }) {
    const index = this.messages.findIndex((item) => {
      return item instanceof UserMessage && item.id === messageId;
    });

    this.messages.length = index + 1;

    const userMessage = this.messages[index] as UserMessage;
    userMessage.delivered = null;
    userMessage.finished = false;

    await resendMessage(this.id, userMessage.id);

    this.__receiveAssistantMessage(userMessage, params);
  }

  async exportMarkdown(path: string) {
    saveAsMarkdown(this.id, path);
  }

  async __receiveAssistantMessage(
    userMessage: UserMessage,
    params?: {
      onFinish?: () => void;
    }
  ) {
    const userMessageId = userMessage.id;
    const {
      message: assistantMessage,
      startLoading,
      endLoading,
    } = useAssistantMessage();

    startLoading();
    this.messages.push(assistantMessage);

    this.busy = true;
    const unListen = await listen(userMessageId, (event) => {
      endLoading();
      const chunk = event.payload as MessageChunk;

      switch (chunk.type) {
        case "error": {
          this.messages.pop();
          this.messages.push(new ErrorMessage(chunk.data));
          userMessage.delivered = false;
          this.busy = false;
          params?.onFinish?.();
          break;
        }
        case "data": {
          assistantMessage.appendContent(chunk.data);
          userMessage.delivered = true;
          break;
        }
        case "done": {
          assistantMessage.markHistory();
          this.busy = false;
          userMessage.finished = true;
          params?.onFinish?.();
          unListen();
          break;
        }
      }
    });
  }
}

function useAssistantMessage() {
  let index = 0;
  const values = [".&nbsp;&nbsp;", "..&nbsp;", "..."];
  const message = reactive(new AssistantMessage());

  let interval: NodeJS.Timeout;

  function startLoading() {
    message.content = values[index];
    interval = setInterval(() => {
      index += 1;
      index = index % values.length;
      message.content = values[index];
    }, 500);
  }

  function endLoading() {
    if (!message.pending) {
      return;
    }
    message.pending = false;
    clearInterval(interval);
    message.content = "";
  }

  return {
    message,
    startLoading,
    endLoading,
  };
}
