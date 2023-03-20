import { computed, defineComponent, PropType, Ref, ref, watch } from "vue";
import {
  Settings as SettingIcon,
  InformationCircleOutline as InfoIcon,
} from "@vicons/ionicons5";
import {
  NDrawer,
  NDynamicTags,
  NForm,
  NFormItem,
  NIcon,
  NInput,
  NInputNumber,
  NScrollbar,
  NSelect,
  NTooltip,
} from "naive-ui";
import { Chat } from "../models/chat";
import { useI18n } from "../hooks/i18n";
import { debugLog, updateChat } from "../api";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
  },
  setup(props) {
    const { t } = useI18n();

    watch(
      () => props.chat.config,
      () => {
        debugLog(
          `chat config changed: ${props.chat.id} ${JSON.stringify(props.chat.config, null, 2)}`
        );
        updateChat({
          id: props.chat.id,
          config: props.chat.config,
        });
      },
      { deep: true }
    );

    const drawerShown = ref(false);

    const configs = computed<
      Array<{
        type: "number" | "select" | "input" | "dynamicTags";
        label: string;
        path: keyof typeof props.chat.config;
        tooltip: string;
        options?: Ref<{ label?: string; value: string }[]>;
        precision?: number;
        max?: number;
        min?: number;
        step?: number;
      }>
    >(() => [
      {
        type: "select",
        label: t("chat.config.model"),
        path: "model",
        tooltip: t("chat.config.model.hint"),
        options: computed(() => [
          { value: "gpt-4" },
          { value: "gpt-4-0314" },
          { value: "gpt-4-32k" },
          { value: "gpt-4-32k-0314" },
          { value: "gpt-3.5-turbo" },
          { value: "gpt-3.5-turbo-0301" },
        ]),
      },
      {
        type: "number",
        label: t("chat.config.maxBacktrack"),
        path: "maxBacktrack",
        tooltip: t("chat.config.maxBacktrack.hint"),
        min: 0,
        precision: 0,
        step: 1,
      },
      {
        type: "number",
        label: t("chat.config.temperature"),
        path: "temperature",
        tooltip: t("chat.config.temperature.hint"),
        min: 0,
        max: 2,
        precision: 1,
        step: 0.1,
      },
      // {
      //   type: "number",
      //   label: t("chat.config.topP"),
      //   path: "topP",
      //   tooltip: t("chat.config.topP.hint"),
      //   min: 0,
      //   max: 1,
      //   precision: 1,
      //   step: 0.1,
      // },
      // {
      //   type: "number",
      //   label: t("chat.config.n"),
      //   path: "n",
      //   tooltip: t("chat.config.n.hint"),
      //   min: 0,
      //   max: 1,
      //   precision: 1,
      //   step: 0.1,
      // },
      // {
      //   type: "dynamicTags",
      //   label: t("chat.config.stop"),
      //   path: "stop",
      //   tooltip: t("chat.config.stop.hint"),
      //   max: 4,
      // },
      {
        type: "number",
        label: t("chat.config.maxTokens"),
        path: "maxTokens",
        tooltip: t("chat.config.maxTokens.hint"),
        min: 0,
        precision: 0,
        step: 100,
      },
      {
        type: "number",
        label: t("chat.config.presencePenalty"),
        path: "presencePenalty",
        tooltip: t("chat.config.presencePenalty.hint"),
        min: -2.0,
        max: 2.0,
        precision: 1,
        step: 0.1,
      },
      {
        type: "number",
        label: t("chat.config.frequencyPenalty"),
        path: "frequencyPenalty",
        tooltip: t("chat.config.frequencyPenalty.hint"),
        min: -2.0,
        max: 2.0,
        precision: 1,
        step: 0.1,
      },
    ]);

    function showDrawer() {
      drawerShown.value = true;
    }

    return () => (
      <div>
        <span onClick={showDrawer} class="relative" style="top: .1rem">
          <NIcon size={20}>
            <SettingIcon />
          </NIcon>
        </span>
        <NDrawer v-model:show={drawerShown.value}>
          <NScrollbar>
            <NForm class="p-4" model={props.chat.config}>
              {configs.value.map((config) => (
                <NFormItem>
                  {{
                    label: () => (
                      <div class="flex items-center">
                        {config.label}
                        <NTooltip>
                          {{
                            trigger: () => (
                              <NIcon class="ml-1" size={18}>
                                <InfoIcon />
                              </NIcon>
                            ),
                            default: () => config.tooltip,
                          }}
                        </NTooltip>
                      </div>
                    ),
                    default: () => {
                      switch (config.type) {
                        case "number": {
                          return (
                            <NInputNumber
                              v-model:value={props.chat.config[config.path]}
                              min={config.min}
                              max={config.max}
                              step={config.step}
                              precision={config.precision}
                            ></NInputNumber>
                          );
                        }
                        case "input": {
                          return (
                            <NInput
                              v-model:value={props.chat.config[config.path]}
                            ></NInput>
                          );
                        }
                        case "select": {
                          return (
                            <NSelect
                              v-model:value={props.chat.config[config.path]}
                              options={config.options!.value.map((item) => {
                                return {
                                  key: item.value,
                                  label: item.label || item.value,
                                  value: item.value,
                                };
                              })}
                            ></NSelect>
                          );
                        }
                        case "dynamicTags": {
                          return (
                            <NDynamicTags
                              v-model:value={props.chat.config[config.path]}
                            ></NDynamicTags>
                          );
                        }
                      }
                    },
                  }}
                </NFormItem>
              ))}
            </NForm>
          </NScrollbar>
        </NDrawer>
      </div>
    );
  },
});
