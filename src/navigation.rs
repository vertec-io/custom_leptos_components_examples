use leptos::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let (current_route, set_current_route) = create_signal(String::from("/"));

    let base_style = "flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:bg-secondary/90 hover:cursor-pointer";

    let routes = vec![
        ("/home", "Home"),
        ("/settings", "Settings"),
        ("/blog", "Blog"),
    ];

    let class_for_route = move |route: &str| {
        if current_route.get() == route {
            format!("{} font-bold text-blue-500", base_style)
        } else {
            base_style.to_string()
        }
    };

    let handle_click = move |route: &str| {
        set_current_route.set(route.to_string());
    };

    view! {
        <nav class="grid gap-2 text-sm font-medium">
            {routes.into_iter().map(|(route, label)| {
                view! {
                    <a
                        href=route
                        class=move || class_for_route(route)
                        on:click=move |_| handle_click(route)
                    >
                        {label}
                    </a>
                }
            }).collect::<Vec<_>>()}
        </nav>
    }
}