use serde::Deserialize;

use crate::result::Result;
use crate::{models::setting::Setting, repositories::setting::SettingRepo, DbConn, Id};
use crate::{HomePage, PatchSetting, Theme};

#[derive(Clone)]
pub struct SettingService {
    #[allow(unused)]
    conn: DbConn,
    setting_repo: SettingRepo,
}

impl From<DbConn> for SettingService {
    fn from(conn: DbConn) -> Self {
        Self::new(conn)
    }
}

impl SettingService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            setting_repo: SettingRepo::new(conn.clone()),
            conn,
        }
    }

    pub fn get_setting(&self, user_id: Id) -> Result<Setting> {
        let setting = self.setting_repo.select_by_user_id(user_id)?;

        Ok(setting)
    }

    pub fn update_setting(&self, payload: UpdateSettingPayload) -> Result<()> {
        self.setting_repo.update(&PatchSetting {
            user_id: payload.user_id.unwrap_or_default(),
            language: payload.language,
            theme: payload.theme.map(|t| t.into()),
            scale: payload.scale,
            api_key: payload.api_key,
            proxy: payload.proxy,
            forward_url: payload.forward_url,
            forward_api_key: payload.forward_api_key,
            hide_main_window: payload.hide_main_window,
            hide_taskbar: payload.hide_taskbar,
            enable_web_server: payload.enable_web_server,
            home_page: payload.home_page.map(|h| h.into()),
        })?;

        Ok(())
    }
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingPayload {
    pub user_id: Option<Id>,
    pub language: Option<String>,
    pub theme: Option<Theme>,
    pub scale: Option<i32>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
    pub hide_main_window: Option<bool>,
    pub hide_taskbar: Option<bool>,
    pub enable_web_server: Option<bool>,
    pub home_page: Option<HomePage>,
}
