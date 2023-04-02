import { listen } from "@tauri-apps/api/event";
import { reactive, ref, Ref } from "vue";
import {
  resendMessage,
  sendMessage,
  exportMarkdown,
  ChatIndex,
  ChatLog,
  getChat,
  updateChat,
  deleteChatLog,
} from "../api";
import {
  AssistantMessage,
  ErrorMessage,
  Message,
  MessageChunk,
  UserMessage,
} from "./message";

export class Chat {
  busy: Ref<boolean>;
  index: ChatIndex;
  messages: Array<Message>;
  constructor(index: ChatIndex, messages: Message[] = []) {
    this.busy = ref(false);
    this.index = reactive(index);
    this.messages = reactive(messages);
  }

  static init(chat: ChatIndex, logs: Array<ChatLog>) {
    const messages: Message[] = [];
    for (let i = 0; i < logs.length; i++) {
      const log = logs[i];
      switch (log.role) {
        case "user": {
          const msg = new UserMessage(log.message);
          msg.setId(log.id);
          msg.finished = logs[i + 1]?.role === "assistant";
          msg.markHistory();
          messages.push(msg);
          break;
        }
        case "assistant": {
          const msg = new AssistantMessage(log.id, log.message);
          msg.markHistory();
          messages.push(msg);
          break;
        }
      }
    }

    return new Chat(chat, messages);
  }

  async deleteLog(logId: string) {
    await deleteChatLog(logId);
    const index = this.messages.findIndex((item) => item.id === logId);
    this.messages.splice(index, 1)
  }

  async sendMessage(message: string, params?: { onFinish?: () => void }) {
    const userMessage = reactive(new UserMessage(message));
    this.messages.push(userMessage);

    const [messageId, replyId] = await sendMessage(this.index.id, message);
    userMessage.setId(messageId);

    this.__receiveAssistantMessage(this, userMessage, replyId, params);

    return messageId;
  }

  async resendMessage(messageId: string, params?: { onFinish?: () => void }) {
    const index = this.messages.findIndex((item) => {
      return item instanceof UserMessage && item.id === messageId;
    });

    const userMessage = this.messages[index] as UserMessage;

    this.messages.length = index + 1;

    userMessage.delivered = false;
    userMessage.finished = null;

    const [newMessageId, replyId] = await resendMessage(userMessage.id);
    userMessage.id = newMessageId;

    this.__receiveAssistantMessage(this, userMessage, replyId, params);
  }

  async updateBacktrack(backtrack: number) {
    this.index.config.backtrack = backtrack;
    await updateChat({
      id: this.index.id,
      config: this.index.config,
    });
  }

  async exportMarkdown(path: string) {
    exportMarkdown(this.index.id, path);
  }

  async __receiveAssistantMessage(
    chat: Chat,
    userMessage: UserMessage,
    id: string,
    params?: {
      onFinish?: () => void;
    }
  ) {
    const userMessageId = userMessage.id;
    const {
      message: assistantMessage,
      startLoading,
      endLoading,
    } = useAssistantMessage(id);

    startLoading();
    this.messages.push(assistantMessage);

    this.busy.value = true;
    const unListen = await listen(userMessageId, (event) => {
      endLoading();
      const chunk = event.payload as MessageChunk;

      switch (chunk.type) {
        case "error": {
          console.log(chunk);
          this.messages.pop();
          this.messages.push(new ErrorMessage(chunk.data));
          userMessage.finished = false;
          this.busy.value = false;
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
          this.busy.value = false;
          userMessage.finished = true;
          params?.onFinish?.();
          unListen();

          getChat(chat.index.id).then((newChat) => {
            console.log(newChat);
            Object.assign(chat.index, newChat);
          });
          break;
        }
      }
    });
  }
}

function useAssistantMessage(id: string) {
  let index = 0;
  const values = [".&nbsp;&nbsp;", "..&nbsp;", "..."];
  const message = reactive(new AssistantMessage(id, ""));

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
