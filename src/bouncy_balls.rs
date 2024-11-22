use leptos::html::Canvas;
use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use std::rc::Rc;
use std::cell::RefCell;

#[component]
pub fn BouncingBallsCanvas() -> impl IntoView {
    // Retrieve the canvas NodeRef from context
    let canvas_ref = use_context::<NodeRef<Canvas>>()
        .expect("Canvas NodeRef should be set in context");

    let balls = Rc::new(RefCell::new(Vec::new()));

    create_effect(move |_| {
        if let Some(canvas_element) = canvas_ref.get() {
            let canvas: HtmlCanvasElement = <HtmlCanvasElement as Clone>::clone(&canvas_element).dyn_into().unwrap();
            canvas.set_width(800);
            canvas.set_height(600);

            // Get the 2d rendering context from the canvas
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            initialize_balls(&mut *balls.borrow_mut(), 100, canvas.width() as f64, canvas.height() as f64);

            // Animation closure using request_animation_frame
            let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
            let g = f.clone();

            let value = balls.clone();
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                update_and_draw_balls(&context, &mut *value.borrow_mut());
                
                window()
                    .expect("should have a window")
                    .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                    .expect("should register animation frame");
            }) as Box<dyn FnMut()>));

            // Start the animation loop
            window()
                .expect("should have a window")
                .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("should register animation frame");
        }
    });

    view! { }
}

// Ball structure to store properties of each ball
#[derive(Clone, Copy)]
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    color: &'static str,
}

fn initialize_balls(balls: &mut Vec<Ball>, count: usize, width: f64, height: f64) {
    let colors = vec!["#FF5733", "#33FF57", "#3357FF", "#FF33A1", "#A133FF"];
    for _ in 0..count {
        balls.push(Ball {
            x: rand::random::<f64>() * width,
            y: rand::random::<f64>() * height,
            vx: (rand::random::<f64>() - 0.5) * 10.0,
            vy: (rand::random::<f64>() - 0.5) * 10.0,
            radius: 5.0,
            color: colors[rand::random::<usize>() % colors.len()],
        });
    }
}

fn update_and_draw_balls(context: &CanvasRenderingContext2d, balls: &mut Vec<Ball>) {
    let width = context.canvas().unwrap().width() as f64;
    let height = context.canvas().unwrap().height() as f64;

    for ball in balls.iter_mut() {
        // Update position based on velocity
        ball.x += ball.vx;
        ball.y += ball.vy;

        // Bounce off walls by reversing direction upon collision
        if ball.x - ball.radius < 0.0 || ball.x + ball.radius > width {
            ball.vx *= -1.0;
        }
        if ball.y - ball.radius < 0.0 || ball.y + ball.radius > height {
            ball.vy *= -1.0;
        }

        // Draw the ball on the canvas
        context.begin_path();
        context.set_fill_style_str(&ball.color);
        context
            .arc(ball.x, ball.y, ball.radius, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
    }
}
