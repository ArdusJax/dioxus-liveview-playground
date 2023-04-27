use axum::{extract::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;
use dioxus_liveview;

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html data-bs-theme="dark" lang="en">
                <head> 
                    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" />
                    <title>Dioxus LiveView with Axum</title>  
                </head>
                <body> 
                    <div class="container my-5">
                    
  <div class="p-5 text-center bg-body-tertiary rounded-3">
    <svg class="bi mt-4 mb-3" style="color: var(--bs-indigo);" width="100" height="100"><use xlink:href='#bootstrap'></use></svg>
    <h1 class="text-body-emphasis">Jumbotron with icon</h1>
    <p class="col-lg-8 mx-auto fs-5 text-muted">
      This is a custom jumbotron featuring an SVG image at the top, some longer text that wraps early thanks to a responsive <code>.col-*</code> class, and a customized call to action.
    </p>
    <div class="d-inline-flex gap-2 mb-5">
      <button class="d-inline-flex align-items-center btn btn-primary btn-lg px-4 rounded-pill" type="button">
        Call to action
        <svg class="bi ms-2" width="24" height="24"><use xlink:href='#arrow-right-short'></use></svg>
      </button>
      <button class="btn btn-outline-secondary btn-lg px-4 rounded-pill" type="button">
        Secondary link
      </button>
    </div>
  </div>

                    </div> 
                <div id="main" class="container my-5">
                {glue}
                </div>
                </body>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.min.js"></script>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.bundle.min.js"></script>
                </html>
                "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view
                        .launch(dioxus_liveview::axum_socket(socket), MemeEditor)
                        .await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        button {
            onclick: move |event| println!("Clicked! Event: {event:?}"), "click me!"
        }
    })
}

//
// Meme Editor project
//
#[inline_props]
fn CaptionEditor<'a>(
    cx: Scope<'a>,
    caption: &'a str,
    on_input: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    let input_style = r"
        border: none;
        background: cornflowerblue;
        padding: 8px 16px;
        margin: 0;
        border-radius: 4px;
        color: white;
    ";

    cx.render(rsx!(input {
        style: "{input_style}",
        value: "{caption}",
        oninput: move |event| on_input.call(event),
    }))
}

#[inline_props]
fn Meme<'a>(cx: Scope<'a>, caption: &'a str) -> Element<'a> {
    let container_style = r#"
        position: relative;
        width: fit-content;
    "#;

    let caption_container_style = r#"
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 16px 8px;
    "#;

    let caption_style = r"
        font-size: 32px;
        margin: 0;
        color: white;
        text-align: center;
    ";

    cx.render(rsx!(
        div {
            style: "{container_style}",
            img {
                src: "https://i.imgflip.com/2zh47r.jpg",
                height: "500px",
            },
            div {
                style: "{caption_container_style}",
                p {
                    style: "{caption_style}",
                    "{caption}"
                }
            }
        }
    ))
}

fn MemeEditor(cx: Scope) -> Element {
    let container_style = r"
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin: 0 auto;
        width: fit-content;
    ";

    let caption = use_state(cx, || "me waiting for my rust code to compile".to_string());

    cx.render(rsx! {
        div {
            style: "{container_style}",
            h1 { "Meme Editor" },
            Meme {
                caption: caption,
            },
            CaptionEditor {
                caption: caption,
                on_input: move |event: FormEvent| {caption.set(event.value.clone());},
            },
        }
    })
}
//
