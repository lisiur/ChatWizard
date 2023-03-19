import { app } from "@tauri-apps/api";
import {
  checkUpdate,
  installUpdate,
  UpdateManifest,
  UpdateStatus,
  UpdateStatusResult,
} from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { emit, Event, listen } from "@tauri-apps/api/event";
import { ref } from "vue";

const version = ref("");
app.getVersion().then((v) => {
  version.value = v;
});
const hasNewVersion = ref(false);
const newVersion = ref<UpdateManifest>();
checkUpdate().then(({ shouldUpdate, manifest }) => {
  hasNewVersion.value = shouldUpdate;
  newVersion.value = manifest;
});
export function useVersion() {
  function installNewVersion() {
    emit("tauri://update-install");
    return new Promise(async (resolve, reject) => {
      const unListen = await listen(
        "tauri://update-status",
        (res: Event<UpdateStatusResult>) => {
          switch (res.payload.status) {
            case "PENDING": {
              return;
            }
            case "DONE": {
              resolve(true);
              unListen();
              return;
            }
            case "ERROR": {
              reject(res.payload.error);
              unListen();
              return;
            }
          }
        }
      );
    });
  }

  return {
    version,
    hasNewVersion,
    newVersion,
    installNewVersion,
    relaunch,
  };
}
