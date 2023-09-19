mod app;

use app::*;
use leptos::*;

// #[component]
// pub fn Router() {
//
// }

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div>
                <h3>"My App here"</h3>
                <App/>
            </div>
        }
    })
}
