// mod app;
//
// use app::*;
use leptos::*;
use leptos_router::*;

type ContactSummary = (); // TODO!
type Contact = (); // TODO!()

// contact_data reruns whenever the :id param changes
async fn contact_data(id: String) -> Contact {
    todo!()
}

// contact_list_data *doesn't* rerun when the :id changes,
// because that param is nested lower than the <ContactList/> route
async fn contact_list_data() -> Vec<ContactSummary> {
    todo!()
}

#[component]
fn Contact(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let data = create_resource(
        cx,
        move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
        move |id| contact_data(id),
    );
    todo!()
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    todo!()
}

#[component]
fn ContactList(cx: Scope) -> impl IntoView {
    // loads the contact list data once; doesn't reload when nested routes change
    let contacts = create_resource(cx, || (), |_| contact_list_data());
    view! {
      cx,
      <div>
        // show the contacts
        <ul>
          {move || contacts.read(cx).map(|contacts| view! { cx, <li>"todo contact info"</li> } )}
        </ul>

        // insert the nested child route here
        <Outlet/>
      </div>
    }
}

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <div>
        // we wrap the whole app in a <Router/> to allow client-side navigation
        // from our nav links below
          // <nav> and <main> will show on every route
          // <div>
          //   // LR will enhance the active <a> link with the [aria-current] attribute
          //   // we can use this for styling them with CSS like `[aria-current] { font-weight: bold; }`
          //   <A href="contacts">"Contacts"</A>
          //   // But we can salso use a normal class attribute like it is a normal component
          // </div>
          <h2>ads</h2>
          <DarkModeToggle is_dark=true name="aris is god!".to_string() />
      </div>
    }
}

#[component]
pub fn DarkModeToggle(
    cx: Scope,
    /// Is bool prop
    #[prop(optional)]
    is_dark: bool,
    // #[prop(optional)]
    name: String,
) -> impl IntoView {
    println!("is_dark: {is_dark}");
    // info!("is_dark here {is_dark}");
    view! {
        cx,
        <div>
            "hello"
            {is_dark}
            "Text"
            {name}
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div>
                <h3>"My App here"</h3>
                // <AppRouter />
                <DarkModeToggle name={"ads".to_string()} is_dark={true} />
                // <App/>
            </div>
        }
    })
}
