use gloo_net::http::Request;
use leptos::prelude::on_cleanup;
use send_wrapper::SendWrapper;
use std::future::Future;
use web_sys::AbortController;

pub fn fetch_markdown(path: &str) -> impl Future<Output = Option<String>> + Send + '_ {
    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        // abort in-flight requests if, e.g., we've navigated away from this page
        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });

        Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| log::error!("{e}"))
            .ok()?
            .text()
            .await
            .ok()
    })
}
