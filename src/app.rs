use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes, Outlet, ParentRoute},
    StaticSegment,
};

use crate::{navigation::Navigation, portal_provider::PortalProvider, bouncy_balls::BouncingBallsCanvas, bouncy_squares::BouncingSquaresCanvas};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        // <PortalProvider>
        <Router>
            <Routes fallback=|| "Page not found.">
                <ParentRoute path=StaticSegment("") view=Home>
                    <Route path=StaticSegment("") view=move || view! { <div>"Welcome! Navigate to /home, /settings, or /blog"</div> }/>
                    <Route path=StaticSegment("home") view=move || view! { <PageWithCanvas title="Home"/> }/>
                    <Route path=StaticSegment("settings") view=move || view! { <PageWithCanvas2 title="Settings"/> }/>
                    <Route path=StaticSegment("blog") view=move || view! { <h2>"Nothing Here"</h2> }/>
                </ParentRoute>
            </Routes>
        </Router>
        // </PortalProvider>
    }
}

// #[component]
// fn Home() -> impl IntoView {
//     let (value, set_value) = signal(0);

//     // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
//     view! {
//         <Title text="Leptos + Tailwindcss"/>
//         <main>
//             <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
//                 <div class="flex flex-row-reverse flex-wrap m-auto">
//                     <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
//                         "+"
//                     </button>
//                     <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
//                         {value}
//                     </button>
//                     <button
//                         on:click=move |_| set_value.update(|value| *value -= 1)
//                         class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
//                         class:invisible=move || {value.get() < 1}
//                     >
//                         "-"
//                     </button>
//                 </div>
//             </div>
//         </main>
//     }
// }

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <PortalProvider>
            <div class="my-0 mx-auto max-w-3xl text-center">        
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

                <button class="bg-red-500 hover:bg-slate-700 px-5 py-3 text-white rounded-lg">"Primary"</button>
                <Outlet/>
            </div>
        </PortalProvider>
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
