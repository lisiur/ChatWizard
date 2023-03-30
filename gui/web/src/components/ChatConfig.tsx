import {
  computed,
  defineComponent,
  PropType,
  Ref,
  ref,
  toRefs,
  watch,
} from "vue";
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
import { updateChat } from "../api";

export default defineComponent({
  props: {
    chat: {
      type: Object as PropType<Chat>,
      required: true,
    },
  },
  setup(props) {
    const { t } = useI18n();

    let unwatch: () => void;
    watch(
      () => props.chat,
      () => {
        if (unwatch) {
          unwatch();
        }
        unwatch = watch(
          Object.values(toRefs(props.chat.index.config)),
          () => {
            updateChat({
              id: props.chat.index.id,
              config: props.chat.index.config,
            });
          },
          { deep: true }
        );
      },
      {
        immediate: true,
      }
    );

    const drawerShown = ref(false);

    const configs = computed<
      Array<{
        type: "number" | "select" | "input" | "dynamicTags";
        label: string;
        path: keyof typeof props.chat.index.config.params;
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
        label: t("chat.config.temperature"),
        path: "temperature",
        tooltip: t("chat.config.temperature.hint"),
        min: 0,
        max: 2,
        precision: 1,
        step: 0.1,
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
            <NForm class="p-4" model={props.chat.index.config}>
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
                              v-model:value={props.chat.index.config.params[config.path]}
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
                              v-model:value={props.chat.index.config.params[config.path]}
                            ></NInput>
                          );
                        }
                        case "select": {
                          return (
                            <NSelect
                              v-model:value={props.chat.index.config.params[config.path]}
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
                              v-model:value={props.chat.index.config.params[config.path]}
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
