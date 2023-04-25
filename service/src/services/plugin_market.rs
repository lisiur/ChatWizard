use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    models::plugin::{NewPlugin, PluginConfig},
    repositories::{plugin::PluginRepo, setting::SettingRepo},
    result::Result,
    DbConn, Id,
};

#[derive(Clone)]
pub struct PluginMarketService {
    #[allow(unused)]
    conn: DbConn,
    setting_repo: SettingRepo,
    plugin_repo: PluginRepo,
}

impl From<DbConn> for PluginMarketService {
    fn from(conn: DbConn) -> Self {
        Self::new(conn)
    }
}

impl PluginMarketService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            setting_repo: SettingRepo::new(conn.clone()),
            plugin_repo: PluginRepo::new(conn.clone()),
            conn,
        }
    }

    pub async fn get_market_plugins(&self) -> Result<Vec<MarketPlugin>> {
        let url = "https://chatwizard.github.io/plugins/index.json";

        let setting = self.setting_repo.select_by_user_id(Id::local())?;
        let client = setting.create_client(Some(Duration::from_secs(10)));

        println!("{:?}", url);

        let res = client.get(url).await?;
        let plugins: Vec<MarketPlugin> = res.json().await?;

        println!("{:?}", plugins);

        Ok(plugins)
    }

    pub async fn install_market_plugin(&self, payload: InstallMarketPluginPayload) -> Result<Id> {
        let id = Id::random();

        let setting = self.setting_repo.select_by_user_id(Id::local())?;
        let client = setting.create_client(Some(Duration::from_secs(10)));

        let res = client.get(&payload.plugin.url).await?;

        let code = res.bytes().await?.to_vec();

        let plugin = NewPlugin {
            id,
            name: payload.plugin.name,
            description: payload.plugin.description,
            version: payload.plugin.version,
            author: payload.plugin.author,
            code,
            config: PluginConfig::default().into(),
        };
        self.plugin_repo.insert(plugin)?;

        Ok(id)
    }

    pub async fn get_market_plugin_readme(&self, url: String) -> Result<String> {
        let setting = self.setting_repo.select_by_user_id(Id::local())?;
        let client = setting.create_client(Some(Duration::from_secs(10)));

        let res = client.get(&url).await?;

        let readme = res.text().await?;

        Ok(readme)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPlugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub url: String,
    pub readme: String,
    pub homepage: String,
}

pub struct InstallMarketPluginPayload {
    pub plugin: MarketPlugin,
    pub user_id: Id,
}
