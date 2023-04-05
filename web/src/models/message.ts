export class Message {
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
  // waiting for response
  pending = true;
  // response is completed
  done = false;

  constructor(id: string, content: string) {
    super();
    this.id = id;
    this.content = content;
  }

  appendContent(content: string) {
    this.content += content;
    return this;
  }

  markHistory() {
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
