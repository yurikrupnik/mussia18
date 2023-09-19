use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ClusterArgs<'a> {
    name: &'a str,
    cpu: i16,
}

// #[component]
// pub fn List(cx: Scope) -> impl IntoView {
//     let (name, set_name) = create_signal(cx, String::new());
//     let (greet_msg, set_greet_msg) = create_signal(cx, String::new());
//     let update_name = move |ev| {
//         let v = event_target_value(&ev);
//         set_name.set(v);
//     };
//     view! { cx,
//         <main class="container">
//             <p>"Click on the Tauri and Leptos logos to learn more."</p>
//                 <form class="row" on:submit=greet>
//                     <input
//                         id="greet-input"
//                         placeholder="Enter cluster name..."
//                         on:input=update_name
//                     />
//                     <button type="submit">"Greet"</button>
//                 </form>
//                 <p><b>{ move || greet_msg.get() }</b></p>
//         </main>
//     }
// }

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let create_local_workspace = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("create_local_workspace", args)
                .await
                .as_string()
                .unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let delete_local_workspace = move |ev: SubmitEvent| {
        ev.prevent_default();
        println!("env {ev:?}");
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&ClusterArgs {
                name: &name.get(),
                cpu: 1,
            })
            .unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("delete_local_workspace", args)
                .await
                .as_string()
                .unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! { cx,
        <main class="container">
            <div>
                <p>"Click on the Tauri and Leptos logos to learn more."</p>

                <form class="row" on:submit=greet>
                    <input
                        id="greet-input"
                        placeholder="Enter a name..."
                        on:input=update_name
                    />
                    <button type="submit">"Greet"</button>
                </form>

                <p><b>{ move || greet_msg.get() }</b></p>
            </div>
             <div>
                <p>"Click on the Tauri and Leptos logos to learn more."</p>

                <form class="row" on:submit=create_local_workspace>
                    <input
                        id="greet-input"
                        placeholder="Enter cluster name..."
                        on:input=update_name
                    />
                    <button type="submit">"Create Cluster"</button>
                </form>

                <p><b>{ move || greet_msg.get() }</b></p>
            </div>
            <div>
                <p>"Click on the Tauri and Leptos logos to learn more."</p>

                <form class="row" on:submit=delete_local_workspace>
                    <input
                        id="greet-input"
                        placeholder="Enter cluster name..."
                        on:input=update_name
                    />
                    <button type="submit">"Delete Cluster"</button>
                </form>

                <p><b>{ move || greet_msg.get() }</b></p>
            </div>
        </main>
    }
}
