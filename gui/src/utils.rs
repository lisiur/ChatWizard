use askai_api::{OpenAIApi, Topic};

use crate::setting::Setting;

pub async fn create_topic(setting: &Setting) -> Topic {
    let api_key = setting.opts.api_key.as_deref().unwrap_or_default();
    let proxy = &setting.opts.proxy;

    let api = OpenAIApi::new(api_key);
    if let Some(proxy) = proxy {
        api.set_proxy(proxy).await;
    }
    Topic::new(api, None)
}
