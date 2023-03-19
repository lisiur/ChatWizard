import { NForm, NFormItem, NInput, NSelect } from "naive-ui";
import { defineComponent, nextTick } from "vue";
import {
  getSettings,
  setApiKey,
  setLocale,
  setProxy,
  setTheme,
  Theme,
} from "../../api";
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
        await setLocale(model.value.locale as string);
      });
    }

    async function changeApiKeyHandler() {
      await setApiKey(model.value.apiKey ?? "");
    }

    async function changeProxyHandler() {
      await setProxy(model.value.proxy ?? "");
    }

    async function changeThemeHandler() {
      nextTick().then(async () => {
        await setTheme(model.value.theme ?? Theme.System);
      });
    }

    return () => (
      <div
        data-tauri-drag-region
        class="h-full p-8 flex flex-col justify-center"
      >
        <div class="mt-8">
          {model.value ? (
            <NForm model={model.value} labelPlacement="left" labelWidth="5rem">
              <NFormItem label={t("setting.locale")}>
                <NSelect
                  v-model:value={model.value.locale}
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
              <NFormItem label={t("setting.apiKey")}>
                <NInput
                  v-model:value={model.value.apiKey}
                  type="password"
                  placeholder={`sk-${"*".repeat(48)}`}
                  onBlur={changeApiKeyHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label={t("setting.proxy")}>
                <NInput
                  v-model:value={model.value.proxy}
                  onBlur={changeProxyHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label={t("setting.theme")}>
                <NSelect
                  v-model:value={model.value.theme}
                  onUpdateValue={changeThemeHandler}
                  options={[
                    {
                      label: t("setting.theme.system"),
                      value: "system",
                    },
                    {
                      label: t("setting.theme.light"),
                      value: "light",
                    },
                    {
                      label: t("setting.theme.dark"),
                      value: "dark",
                    },
                  ]}
                ></NSelect>
              </NFormItem>
            </NForm>
          ) : null}
        </div>
      </div>
    );
  },
});
