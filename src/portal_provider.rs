use leptos::*;
use custom_leptos_components::dynamic_portal::DynamicPortal;

use leptos_router::use_location;
use leptos::html::Canvas;
use web_sys::HtmlDivElement;
use wasm_bindgen::JsCast;

#[component]
pub fn PortalProvider(children:Children) -> impl IntoView {
    // let (mount_target, set_mount_target) = create_signal::<HtmlDivElement>(None);
    let (mount_target, set_mount_target) = create_signal(None::<HtmlDivElement>);
    let location = use_location();

    // Create a NodeRef for the canvas element
    let canvas_ref = create_node_ref::<Canvas>();
    // Set the canvas NodeRef in the context
    provide_context(canvas_ref);

    // Watch for changes to the active `ui_canvas` element and update the portal target
    create_effect(move |_| {
        let _location_path = location.pathname.get();
        leptos::logging::log!("UPDATING CANVAS PORTAL");
        if let Some(target) = document().get_element_by_id("ui_canvas") {
            set_mount_target.set(Some(target.unchecked_into::<HtmlDivElement>()));
        } else {
            set_mount_target.set(None);
        }
    });

    view! {
        {children()}
        <DynamicPortal mount=mount_target use_shadow=false is_svg=false>
            { move || view! {
                <canvas class="border h-full w-full" ref=canvas_ref></canvas>
            }}
        </DynamicPortal>
    }
}

