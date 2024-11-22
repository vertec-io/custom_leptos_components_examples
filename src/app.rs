use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thaw::*;

use crate::{bouncy_balls::BouncingBallsCanvas, bouncy_squares::BouncingSquaresCanvas, navigation::Navigation, portal_provider::PortalProvider};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }>
                    <Route path="" view=Home/>
                    <Route path="/home" view= move || view! { <PageWithCanvas title="Home"/> }/>
                    <Route path="/settings" view= move || view! { <PageWithCanvas2 title="Settings"/> }/>
                    <Route path="/blog" view= move || view! { <h2>Nothing Here</h2> }/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl teAxt-center">        
            <PortalProvider>
            <Navigation/>
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count.get() == 0 {
                    "Click me!".to_string()
                } else {
                    count.get().to_string()
                }}
                " | Some more text"
            </button>

            <Button class="bg-red-500 hover:bg-slate-700" variant=ButtonVariant::Primary>"Primary"</Button>
            <Outlet/>
            </PortalProvider>
        </div>
    }
}

#[component]
fn PageWithCanvas(title: &'static str) -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">{title}</h2>
            <div id="ui_canvas" class="border border-gray-300 p-4">
                "This is the ui_canvas for " {title}
                <BouncingSquaresCanvas/>
            </div>
        </div>
    }
}

#[component]
fn PageWithCanvas2(title: &'static str) -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">{title}</h2>
            <div id="ui_canvas" class="border border-gray-300 p-4">
                "This is the ui_canvas for " {title}
                <BouncingBallsCanvas/>
            </div>
        </div>
    }
}