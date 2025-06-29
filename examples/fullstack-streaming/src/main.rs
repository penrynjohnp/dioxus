use dioxus::prelude::*;
use futures::StreamExt;
use server_fn::codec::{StreamingText, TextStream};

fn app() -> Element {
    let mut response = use_signal(String::new);

    rsx! {
        button {
            onclick: move |_| async move {
                response.write().clear();
                let stream = test_stream().await?;
                response.write().push_str("Stream started\n");
                let mut stream = stream.into_inner();
                while let Some(Ok(text)) = stream.next().await {
                    response.write().push_str(&text);
                }
                Ok(())
            },
            "Start stream"
        }
        "{response}"
    }
}

#[server(output = StreamingText)]
pub async fn test_stream() -> ServerFnResult<TextStream<ServerFnError>> {
    let (tx, rx) = futures::channel::mpsc::unbounded();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            dioxus::logger::tracing::info!("Sending new chunk!");
            if tx.unbounded_send(Ok("Hello, world!".to_string())).is_err() {
                // If the channel is closed, stop sending chunks
                break;
            }
        }
    });

    Ok(TextStream::new(rx))
}

fn main() {
    dioxus::launch(app)
}
