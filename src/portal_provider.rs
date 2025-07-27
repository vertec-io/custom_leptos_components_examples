use custom_leptos_components::dynamic_portal::PersistentPortal;
use custom_leptos_components::dynamic_portal::SendWrapper;
use leptos::html::Div;
use leptos::prelude::*;
use leptos::tachys::html::element::Canvas;
use leptos_router::hooks::use_location;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlDivElement;
use web_sys::Node;

#[component]
pub fn PortalProvider(children:Children) -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();
    let (mount_id, set_mount_id) = signal("dynamic_portal_container"); // Static string
    let context = PortalContext {
        mount_id,
        set_mount_id,
        canvas_ref: SendWrapper::new(canvas_ref),
    };
    provide_context(context);

    let (mount_target, set_mount_target) = arc_signal(None::<Element>);
    let location = use_location();

    // Watch for changes to the active mount_id and update the portal target
    Effect::new(move || {
        let location_path = location.pathname.get(); // Trigger effect on route change
        let context = expect_context::<PortalContext>();
        let mut current_canvas_id = context.mount_id.get();
        if current_canvas_id.is_empty() {
            current_canvas_id = &location_path;
        }
        leptos::logging::log!("UPDATING CANVAS PORTAL to: {}", &current_canvas_id);

        if let Some(target) = document().get_element_by_id(&current_canvas_id) {
            set_mount_target.set(Some(target.unchecked_into::<Element>()));
            leptos::logging::log!("Found ui_canvas_root with id: {:?}", &current_canvas_id);
        } else {
            leptos::logging::log!("NO CANVAS ROOT FOUND");
            set_mount_target.set(None);
        }
    });

    view! {
        {children()}
        <PersistentPortal mount=mount_target >
            <canvas id="ui-canvas" class=" h-full w-full" node_ref=canvas_ref></canvas>
        </PersistentPortal >
    }
}

#[derive(Clone)]
pub struct PortalContext {
    pub mount_id: ReadSignal<&'static str>,
    pub set_mount_id: WriteSignal<&'static str>,
    pub canvas_ref: SendWrapper<NodeRef<Canvas>>,
}


#[component]
pub fn CanvasContainer(
    /// This must always be unique for each component nesting this Canvas
    canvas_id: &'static str,
    /// This will override the internal "h-full w-full"
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    let portal_context = expect_context::<PortalContext>();
    let class = match class {
        "" => "flex h-full w-full",
        _ => class,
    };
    let node_ref: NodeRef<Div> = NodeRef::new();

    // let canvas_ref = portal_context.canvas_ref.take();

    // Update the mount_id with the static string
    Effect::new(move |_| {
        let canvas_node: Option<HtmlDivElement> = node_ref.get();
        if let Some(canvas_node) = canvas_node {
            let canvas_node = canvas_node.dyn_into::<Node>().unwrap();
            portal_context.set_mount_id.set(canvas_id);
            leptos::logging::log!("CANVAS NODE: {:?}", canvas_node);
            let handle = SendWrapper::new(canvas_node);

            Owner::on_cleanup(move || {
                // Before we cleanup this owner, we need to move the canvas back to the hidden container
                // This ensures that we never lose the canvas node and it's reactive system
                let canvas_node = handle.take();
                let portal_context = expect_context::<PortalContext>();
                let canvas_ref = portal_context.canvas_ref.take();

                if let Some(canvas) = canvas_ref.get() {
                    leptos::logging::log!("CANVAS NODE CHILD: {:?}", canvas_node);
                    let hidden_container = document()
                        .get_element_by_id("persistent_portal_container")
                        .unwrap_or_else(|| {
                            // Create the hidden container during cleanup
                            let container = document()
                                .create_element("div")
                                .expect("Failed to create hidden container");
                            container.set_id("persistent_portal_container");
                            container
                                .set_attribute("style", "visibility: hidden; height: 0; width: 0;")
                                .expect("Failed to set container style");
                            document()
                                .body()
                                .expect("Document body not found")
                                .append_child(&container)
                                .expect("Failed to append container to body");
                            container
                        });

                    hidden_container
                        .append_child(&canvas)
                        .expect("Failed to move canvas back to the hidden container.");
                    leptos::logging::log!("Cleaning up Canvas. Moving it back to the hidden container.");
                }

                portal_context.set_mount_id.set("");
            });
        }
    });

    view! {
        <div id=canvas_id class=class node_ref=node_ref></div>
    }
}
