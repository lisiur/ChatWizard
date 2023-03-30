import {
  NForm,
  NFormItem,
  NInput,
  NRadioButton,
  NRadioGroup,
  NSelect,
  NSwitch,
} from "naive-ui";
import { defineComponent, nextTick } from "vue";
import { getSettings, updateSettings, Theme } from "../../api";
import { useAsyncData } from "../../hooks/asyncData";
import { useI18n } from "../../hooks/i18n";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const model = useAsyncData(async () => {
      return getSettings();
    }, {});

    async function changeLocaleHandler() {
      nextTick().then(async () => {
        await updateSettings({ language: model.value.language });
      });
    }

    async function changeApiKeyHandler() {
      await updateSettings({ apiKey: model.value.apiKey ?? "" });
    }

    async function changeProxyHandler() {
      await updateSettings({ proxy: model.value.proxy ?? "" });
    }

    async function changeThemeHandler() {
      nextTick().then(async () => {
        await updateSettings({ theme: model.value.theme ?? Theme.System });
      });
    }

    async function changeForwardUrlHandler() {
      await updateSettings({ forwardUrl: model.value.forwardUrl ?? "" });
    }

    async function changeForwardApiKeyHandler() {
      await updateSettings({
        forwardApiKey: model.value.forwardApiKey ?? false,
      });
    }

    return () => (
      <div
        data-tauri-drag-region
        class="h-full p-8 flex flex-col justify-center"
      >
        <div class="mt-8 pr-12">
          {model.value ? (
            <NForm model={model.value} labelPlacement="left" labelWidth="8rem">
              <NFormItem label={t("setting.locale")}>
                <NSelect
                  v-model:value={model.value.language}
                  placeholder="Locale"
                  onUpdateValue={changeLocaleHandler}
                  options={[
                    {
                      label: "English",
                      value: "enUS",
                    },
                    {
                      label: "中文",
                      value: "zhCN",
                    },
                  ]}
                ></NSelect>
              </NFormItem>
              <NFormItem label={t("setting.theme")}>
                <NRadioGroup
                  v-model:value={model.value.theme}
                  onUpdateValue={changeThemeHandler}
                >
                  <NRadioButton value="system">
                    {t("setting.theme.system")}
                  </NRadioButton>
                  <NRadioButton value="light">
                    {t("setting.theme.light")}
                  </NRadioButton>
                  <NRadioButton value="dark">
                    {t("setting.theme.dark")}
                  </NRadioButton>
                </NRadioGroup>
              </NFormItem>
              <NFormItem label={t("setting.apiKey")}>
                <NInput
                  v-model:value={model.value.apiKey}
                  type="password"
                  placeholder={`sk-${"x".repeat(48)}`}
                  onBlur={changeApiKeyHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label={t("setting.proxy")}>
                <NInput
                  v-model:value={model.value.proxy}
                  onBlur={changeProxyHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label={t("setting.forwardUrl")}>
                <NInput
                  v-model:value={model.value.forwardUrl}
                  onBlur={changeForwardUrlHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label={t("setting.forwardApiKey")}>
                <NSwitch
                  v-model:value={model.value.forwardApiKey}
                  onUpdateValue={changeForwardApiKeyHandler}
                ></NSwitch>
              </NFormItem>
            </NForm>
          ) : null}
        </div>
      </div>
    );
  },
});
