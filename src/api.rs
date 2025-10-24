use gloo_net::http::Request;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use serde::Deserialize;
use thiserror::Error;
use web_sys::AbortController;

#[derive(Error, Clone, Debug)]
pub enum ApiError {
    #[error("Unable to retrieve content.")]
    ContentNotRetrievable,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SectionIndex {
    pub index: Vec<SectionLink>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SectionLink {
    pub title: String,
    pub href: String,
}

pub async fn fetch_index(section: &'static str) -> Result<SectionIndex, Error> {
    let path = format!("/markdown/{}.yaml", section);
    let text = fetch_text(&path).await?;
    let result = serde_yaml::from_str::<SectionIndex>(&text)?;
    Ok(result)
}

pub async fn fetch_markdown(section: &'static str, article: String) -> Result<String, Error> {
    let path = format!("/markdown/{}/{}", section, article);
    let text = fetch_text(&path).await?;
    Ok(markdown::to_html(&text))
}

async fn fetch_text(path: &str) -> Result<String, Error> {
    let abort_controller = SendWrapper::new(AbortController::new().ok());
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());
    on_cleanup(move || {
        if let Some(abort_controller) = abort_controller.take() {
            abort_controller.abort()
        }
    });
    let response = Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await?;
    if response.status() != 200 {
        log::error!("Response code: {}", response.status());
        Err(ApiError::ContentNotRetrievable)?
    } else {
        Ok(response.text().await?)
    }
}
