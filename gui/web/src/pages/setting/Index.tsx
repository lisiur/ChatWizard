import { NForm, NFormItem, NInput, NSelect } from "naive-ui";
import { defineComponent, nextTick } from "vue";
import { getSettings, setApiKey, setProxy, setTheme, Theme } from "../../api";
import { useAsyncData } from "../../hooks/asyncData";

export default defineComponent({
  setup() {
    const model = useAsyncData(async () => {
      return getSettings();
    }, {});

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
      <div data-tauri-drag-region class="h-full p-8 flex flex-col">
        <div class="flex-1 overflow-auto">
          {model.value ? (
            <NForm model={model.value} labelPlacement="left" labelWidth="5rem">
              <NFormItem label="Api Key">
                <NInput
                  v-model:value={model.value.apiKey}
                  type="password"
                  placeholder={`sk-${"*".repeat(48)}`}
                  onBlur={changeApiKeyHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label="Proxy">
                <NInput
                  v-model:value={model.value.proxy}
                  placeholder="Proxy"
                  onBlur={changeProxyHandler}
                ></NInput>
              </NFormItem>
              <NFormItem label="Theme">
                <NSelect
                  v-model:value={model.value.theme}
                  placeholder="Theme"
                  onUpdateValue={changeThemeHandler}
                  options={[
                    {
                      label: "System",
                      value: "system",
                    },
                    {
                      label: "Light",
                      value: "light",
                    },
                    {
                      label: "Dark",
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
