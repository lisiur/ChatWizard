use serde::Deserialize;

use crate::result::Result;
use crate::{models::setting::Setting, repositories::setting::SettingRepo, DbConn, Id};
use crate::{PatchSetting, Theme};

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
            api_key: payload.api_key,
            proxy: payload.proxy,
            forward_url: payload.forward_url,
            forward_api_key: payload.forward_api_key,
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
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}
