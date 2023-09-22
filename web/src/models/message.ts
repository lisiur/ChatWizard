import { v4 as uuid } from "uuid";

export class Message {
  tmpId = uuid(); // for transition group
  id = "";
  content = "";
}

export class UserMessage extends Message {
  delivered = false;
  finished = null as boolean | null;

  constructor(content: string) {
    super();
    this.content = content;
  }

  setId(id: string) {
    this.id = id;
    return this;
  }

  markHistory() {
    this.delivered = true;
    return this;
  }
}

export class AssistantMessage extends Message {
  static throttleTime = 200; // 200ms

  // waiting for response
  pending = true;
  // response is completed
  done = false;

  timer: NodeJS.Timeout | null = null;

  cachedContent = "";
  leading = true;

  constructor(id: string, content: string) {
    super();
    this.id = id;
    this.content = content;
  }

  appendContent(content: string) {
    if (this.leading) {
      this.leading = false;
      this.content = content;
    } else {
      this.cacheContent(this, content);
    }
    return this;
  }

  // use self for vue reactivity
  cacheContent(self: AssistantMessage, content: string) {
    self.cachedContent += content;

    if (!this.timer) {
      this.timer = setInterval(() => {
        self.content += self.cachedContent
        self.cachedContent = "";
      }, AssistantMessage.throttleTime)
    }
  }

  markHistory() {
    clearTimeout(this.timer!);
    this.content += this.cachedContent;
    this.cachedContent = "";
    this.pending = false;
    this.done = true;
    return this;
  }
}

export class ErrorMessage extends Message {
  error: ErrorData;
  constructor(error: ErrorData) {
    super();
    this.error = error;
  }
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

export type ErrorData =
  | {
      type: "network";
      error: {
        type: "timeout" | "unknown";
        message: string;
      };
    }
  | {
      type: "api";
      error: {
        type: string;
        message?: string;
      };
    };
