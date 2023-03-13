import { defineComponent, ref, watch } from "vue";
import mdRender from "./utils/mdRender";
import assistantAvatar from "./assets/assistant_avatar.png";
import userAvatar from "./assets/user_avatar.png";
import newIcon from "./assets/new.svg";
import networkIcon from "./assets/networks.svg";
import keyIcon from "./assets/key.svg";
import {
  AssistantMessage,
  ErrorMessage,
  Message,
  UserMessage,
  useTopic,
} from "./hooks/message";
import { useConfig } from "./hooks/config";
import { NButton, NIcon, NScrollbar, NSpace, NTooltip } from "naive-ui";
import { writeToClipboard } from "./utils/clipboard";
import { useComposition } from "./hooks/composition";
import { useVersion } from "./hooks/version";
import { AngleDoubleUp } from "@vicons/fa";
import { dialog, message } from "./utils/prompt";

export default defineComponent({
  setup() {
    const inputRef = ref<HTMLTextAreaElement>();
    const { isComposition } = useComposition(inputRef);

    const { version, hasNewVersion, installNewVersion, newVersion, relaunch } =
      useVersion();
    const { checkApiKey: check_api_key, setProxy, setApiKey } = useConfig();

    check_api_key();

    const { prompt, sendMessage, resendMessage, reset, topic } = useTopic();

    function keydownHandler(e: KeyboardEvent) {
      if (e.key === "Enter" && !e.ctrlKey && !isComposition.value) {
        sendMessage();
        e.preventDefault();
      }
    }

    function newTopicHandler() {
      reset();
      inputRef.value?.focus();
    }

    function showUpdateHandler() {
      const releaseContent = (newVersion.value?.body ?? "").replaceAll(
        /%0A/g,
        "\n"
      );
      const renderContent = mdRender(releaseContent);
      const loading = ref(false);
      const dl = dialog.create({
        showIcon: false,
        title: "New version is available!",
        content: () => {
          return <div class="markdown-root" v-html={renderContent}></div>;
        },
        action: () => {
          const positiveText = ref("Upgrade");
          return (
            <NSpace>
              <NButton onClick={() => dl.destroy()}>Cancel</NButton>
              <NButton
                type="primary"
                loading={loading.value}
                onClick={() => {
                  positiveText.value = "Downloading...";
                  loading.value = true;
                  installNewVersion()
                    .then(() => {
                      dl.destroy();
                      dialog.success({
                        title: "Download Success",
                        content: "Please restart the app to apply the update.",
                        positiveText: "Restart",
                        negativeText: "Later",
                        onPositiveClick: relaunch,
                      });
                    })
                    .catch((err) => {
                      message.error(err);
                    })
                    .finally(() => {
                      positiveText.value = "Upgrade";
                      loading.value = false;
                    });
                }}
              >
                {positiveText.value}
              </NButton>
            </NSpace>
          );
        },
      });
    }

    return () => (
      <div class="h-full flex flex-col bg-[#1e1e1e]">
        <div class="flex-1 overflow-hidden py-4">
          <NScrollbar>
            <div class="grid gap-6">
              {topic.messages.map((message, index) => (
                <div key={index}>
                  {renderMessage(message, {
                    resendHandler: () =>
                      resendMessage((message as UserMessage).id),
                  })}
                </div>
              ))}
            </div>
          </NScrollbar>
        </div>
        <div class="border-t border-gray-700">
          <div class="flex items-center">
            <div class="text-gray-600 p-2">
              <span> v{version.value} </span>
              {hasNewVersion.value ? (
                <span onClick={showUpdateHandler} class="cursor-pointer">
                  <NIcon>
                    <AngleDoubleUp />
                  </NIcon>
                </span>
              ) : null}
            </div>
            <div class="flex-1 flex justify-end p-1">
              {renderButton({
                handler: setApiKey,
                icon: keyIcon,
                tooltip: "Set Api Key",
              })}
              {renderButton({
                handler: setProxy,
                icon: networkIcon,
                tooltip: "Set proxy",
              })}
              {renderButton({
                handler: newTopicHandler,
                icon: newIcon,
                tooltip: "New Topic",
              })}
            </div>
          </div>
          <textarea
            ref={inputRef}
            v-model={prompt.value}
            placeholder="Type your question here..."
            class="p-2 resize-none w-full bg-transparent outline-none text-white placeholder-slate-500"
            rows="5"
            onKeydown={keydownHandler}
          ></textarea>
        </div>
      </div>
    );
  },
});

function renderButton(props: {
  icon: string;
  tooltip: string;
  handler: () => void;
}) {
  return (
    <NTooltip trigger="hover" delay={500}>
      {{
        trigger: () => (
          <button
            class="bg-transparent rounded px-2 py-1"
            onClick={props.handler}
          >
            <img src={props.icon} class="w-6"></img>
          </button>
        ),
        default: () => props.tooltip,
      }}
    </NTooltip>
  );
}

function renderAvatar(avatar: string) {
  return <img src={avatar} class="w-8 h-8 "></img>;
}

function renderTriangle(
  direction: "left" | "right",
  style?: {
    color: string;
    size: string;
  }
) {
  if (direction === "left") {
    return (
      <div
        class={"border-solid border-y-transparent border-l-0"}
        style={{
          borderRightColor: style?.color,
          borderRightWidth: style?.size,
          borderTopWidth: style?.size,
          borderBottomWidth: style?.size,
        }}
      ></div>
    );
  } else if (direction === "right") {
    return (
      <div
        class={"border-solid border-y-transparent border-r-0"}
        style={{
          borderLeftColor: style?.color,
          borderLeftWidth: style?.size,
          borderTopWidth: style?.size,
          borderBottomWidth: style?.size,
        }}
      ></div>
    );
  }
}

function renderMessage(
  message: Message,
  handlers: {
    resendHandler: () => void;
  }
) {
  if (message.role === "assistant") {
    return renderAssistantMessage(message as AssistantMessage);
  } else if (message.role === "user") {
    return renderUserMessage(message as UserMessage, {
      resendHandler: handlers.resendHandler,
    });
  } else if (message.role === "error") {
    return renderErrorMessage(message as ErrorMessage);
  }
}

function renderAssistantMessage(message: AssistantMessage) {
  const html = (() => {
    if (message.pending) {
      return message.content;
    } else {
      return mdRender(message.content);
    }
  })();
  return (
    <div class="flex justify-start items-start pl-4 pr-16">
      {renderAvatar(assistantAvatar)}
      <div class="relative ml-2">
        <div class="absolute left-[-.2rem] top-1">
          {renderTriangle("left", {
            color: "#323232",
            size: ".5rem",
          })}
        </div>
        <div
          class="markdown-root bg-[#323232] inline-block text-white py-1 px-2 ml-1 rounded-md z-1"
          v-html={html}
        ></div>
        {message.done ? (
          <div class="absolute bottom-0 right-[-2.2rem] text-xs">
            <NButton
              type="default"
              text
              size="tiny"
              class="ml-2 text-gray-500"
              onClick={() => writeToClipboard(message.content)}
            >
              Copy
            </NButton>
          </div>
        ) : null}
      </div>
    </div>
  );
}

function renderUserMessage(
  message: UserMessage,
  handlers: {
    resendHandler: () => void;
  }
) {
  return (
    <div class="flex justify-end items-start pr-4 pl-16">
      <div class="relative mr-2">
        <div class="bg-[#3375ee] inline-block text-white py-1 px-2 mr-1 rounded-md">
          {message.content}
        </div>
        <div class="absolute right-[-.2rem] top-1">
          {renderTriangle("right", {
            color: "#3375ee",
            size: ".5rem",
          })}
        </div>
        <div class="absolute bottom-[-1.1rem] right-0 text-xs">
          {(() => {
            switch (message.delivered) {
              case null: {
                break;
              }
              case true: {
                return <span class="text-gray-600">delivered</span>;
              }
              case false: {
                return (
                  <NButton
                    type="error"
                    text
                    size="tiny"
                    class="mr-2"
                    onClick={handlers.resendHandler}
                  >
                    resend
                  </NButton>
                );
              }
            }
          })()}
        </div>
      </div>
      {renderAvatar(userAvatar)}
    </div>
  );
}

function renderErrorMessage(message: ErrorMessage) {
  return (
    <div class="flex justify-center px-16">
      <div class="text-xs text-white bg-red-400 px-4 py-1 rounded">
        {(() => {
          switch (message.error.type) {
            case "network": {
              const error = message.error.data;
              switch (error.type) {
                case "timeout": {
                  return "Timeout";
                }
                case "unknown": {
                  return error.message;
                }
              }
            }
            case "api": {
              const error = message.error.data;
              return error.message;
            }
          }
        })()}
      </div>
    </div>
  );
}
