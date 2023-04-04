import {
  UpdateManifest,
  UpdateStatusResult,
} from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { Event } from "@tauri-apps/api/event";
import { ref } from "vue";
import { checkUpdate, emit, getVersion, listen } from "../utils/api";

const version = ref("");
getVersion().then((v) => {
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
