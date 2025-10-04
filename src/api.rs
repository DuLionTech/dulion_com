use gloo_net::http::Request;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::future::Future;
use web_sys::AbortController;

pub fn fetch_markdown(path: &str) -> impl Future<Output = Option<String>> + Send + '_ {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());
        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });

        Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| log::error!("Failed to retrieve article: {e}"))
            .ok()
            .filter(|r| {
                if r.status() != 200 {
                    log::error!("Response code: {}", r.status());
                }
                r.ok()
            })?
            .text()
            .await
            .map_err(|e| log::error!("Failed to deserialize article: {e}"))
            .ok()
            .map(|r| markdown::to_html(&r))
    })
}
