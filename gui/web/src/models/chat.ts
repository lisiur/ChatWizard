import { listen } from "@tauri-apps/api/event";
import { reactive } from "vue";
import { resendMessage, sendMessage } from "../api";
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
  constructor(public id: string, messages: Message[] = []) {
    this.busy = false;
    this.messages = reactive(messages);
  }

  async sendMessage(message: string) {
    const userMessage = reactive(new UserMessage(message));
    this.messages.push(userMessage);

    const messageId = await sendMessage(this.id, message);
    userMessage.setId(messageId);

    this.__receiveAssistantMessage(userMessage);

    return messageId;
  }

  async resendMessage(messageId: string) {
    const index = this.messages.findIndex((item) => {
      return item instanceof UserMessage && item.id === messageId;
    });

    this.messages.length = index + 1;

    const userMessage = this.messages[index] as UserMessage;
    userMessage.delivered = null;

    await resendMessage(this.id, userMessage.id);

    this.__receiveAssistantMessage(userMessage);
  }

  async __receiveAssistantMessage(userMessage: UserMessage) {
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
