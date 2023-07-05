import { computed, defineComponent, ref, watch } from "vue";
import * as api from "../../api";
import { message, prompt } from "../../utils/prompt";
import { Plus as PlusIcon } from "@vicons/fa";
import { NIcon, NInputNumber, NScrollbar, NText } from "naive-ui";
import { useI18n } from "../../hooks/i18n";
import Explorer, { ExplorerItem } from "../../components/Explorer";
import DragBar from "../../components/DragBar";
import { useModelService } from "../../services/model";

export default defineComponent({
  setup() {
    const { t } = useI18n();

    const { models, reload } = useModelService();

    const explorerList = computed(() => {
      return (models.value ?? []).map((m) => ({
        id: m.id,
        title: m.name,
        data: m,
      }));
    });

    const currentId = ref<string>();
    watch(models, () => {
      if (!currentId.value && (models.value ?? [])?.length > 0) {
        currentId.value = models.value![0].id;
      }
    });

    const currentChatModel = computed(() =>
      models.value?.find((m) => m.id === currentId.value)
    );

    async function createModel() {
      prompt(t("chatModel.inputNameHint"), {
        async okHandler(title) {
          const id = await api.createChatModel({
            name: title,
            price: 0,
          });
          await reload();
          selectHandler(id);
        },
      });
    }

    async function explorerHandler(action: string, item: ExplorerItem) {
      switch (action) {
        case "delete": {
          await deleteHandler(item.id);
          return;
        }
        case "select": {
          await selectHandler(item.id);
          return;
        }
        case "rename": {
          await renameHandler(item.id, item.title);
          return;
        }
      }
    }

    async function renameHandler(id: string, name: string) {
      prompt(t("chatModel.inputNameHint"), {
        defaultValue: name,
        async okHandler(title) {
          await api.updateChatModel({
            id: id,
            name: title,
          });
          reload();
        },
      });
    }

    async function updateHandler() {
      if (!currentId.value) {
        return;
      }

      await api.updateChatModel({
        id: currentChatModel.value!.id,
        price: currentChatModel.value?.price,
      });
    }

    async function deleteHandler(id: string) {
      if (currentChatModel.value?.id === id) {
        currentId.value = undefined;
      }
      await api.deleteChatModel(id);
      reload();
    }

    async function selectHandler(id: string) {
      currentId.value = id;
    }

    return () => (
      <div class="h-full flex">
        <div
          class="w-48 border-r h-full flex flex-col"
          style="border-color: var(--border-color); background-color: var(--explorer-bg-color); color: var(--explorer-color)"
        >
          <div
            class="h-10 border-b flex justify-center m-2 mt-3 items-center bg-primary cursor-pointer"
            style="color: var(--base-color);border-color: var(--border-color)"
            onClick={createModel}
          >
            <NIcon class="mr-1">
              <PlusIcon />
            </NIcon>
            <span> {t("chatModel.new")} </span>
          </div>
          <div class="p-2 text-gray-400">{t("chatModel.models")}</div>
          <Explorer
            class="flex-1 overflow-auto"
            active={currentChatModel.value?.id}
            menus={[
              {
                label: t("chatModel.rename"),
                key: "rename",
              },
              {
                type: "divider",
              },
              {
                label: t("common.delete"),
                key: "delete",
              },
            ]}
            unstickList={explorerList.value}
            onAction={explorerHandler}
          ></Explorer>
        </div>
        <div class="flex-1 overflow-hidden flex flex-col">
          {currentChatModel.value ? (
            <DragBar title={currentChatModel.value?.name}></DragBar>
          ) : null}
          <div
            class="flex-1 overflow-hidden p-4"
            style="background-color: var(--body-color)"
          >
            {currentChatModel.value ? (
              <NScrollbar class="h-full">
                <NText>
                  {t("chatModel.price")} (1k {t("chatModel.tokens")}):
                </NText>
                <NInputNumber
                  value={+currentChatModel.value.price.toFixed(6)}
                  onUpdateValue={(v) => {
                    if (v) {
                      currentChatModel.value!.price = v;
                    }
                  }}
                  min={0}
                  step={0.001}
                  showButton={false}
                  class="mt-4 rounded-lg outline-none placeholder-slate-500"
                  onBlur={updateHandler}
                >
                  {{
                    prefix: () => "$",
                  }}
                </NInputNumber>
              </NScrollbar>
            ) : (
              <div class="h-full" data-tauri-drag-region></div>
            )}
          </div>
        </div>
      </div>
    );
  },
});
