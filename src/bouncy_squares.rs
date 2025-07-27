use leptos::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use std::rc::Rc;
use std::cell::RefCell;
use crate::portal_provider::PortalContext;
use crate::portal_provider::CanvasContainer;

#[component]
pub fn BouncingSquaresCanvas() -> impl IntoView {
    let portal_context = expect_context::<PortalContext>();
    let canvas_ref = portal_context.canvas_ref.take();
    let squares = Rc::new(RefCell::new(Vec::new()));

    Effect::new(move |_| {
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

            initialize_squares(&mut *squares.borrow_mut(), 100, canvas.width() as f64, canvas.height() as f64);

            // Animation closure using request_animation_frame
            let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
            let g = f.clone();
            let value = squares.clone();
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                update_and_draw_squares(&context, &mut *value.borrow_mut());
                
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

    view! { 
        // The canvas_id must be unique to this component otherwise the canvas won't update
        <CanvasContainer canvas_id="ui-canvas-squares"/>
     }
}

// Square structure to store properties of each square
#[derive(Clone, Copy)]
struct Square {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    side_length: f64,
    color: &'static str,
}

fn initialize_squares(squares: &mut Vec<Square>, count: usize, width: f64, height: f64) {
    let colors = vec!["#FF5733", "#33FF57", "#3357FF", "#FF33A1", "#A133FF"];
    for _ in 0..count {
        squares.push(Square {
            x: rand::random::<f64>() * width,
            y: rand::random::<f64>() * height,
            vx: (rand::random::<f64>() - 0.5) * 10.0,
            vy: (rand::random::<f64>() - 0.5) * 10.0,
            side_length: 10.0,
            color: colors[rand::random::<u32>() as usize % colors.len()],
        });
    }
}

fn update_and_draw_squares(context: &CanvasRenderingContext2d, squares: &mut Vec<Square>) {
    let width = context.canvas().unwrap().width() as f64;
    let height = context.canvas().unwrap().height() as f64;

    for square in squares.iter_mut() {
        // Update position based on velocity
        square.x += square.vx;
        square.y += square.vy;

        // Bounce off walls by reversing direction upon collision
        if square.x < 0.0 || square.x + square.side_length > width {
            square.vx *= -1.0;
        }
        if square.y < 0.0 || square.y + square.side_length > height {
            square.vy *= -1.0;
        }

        // Draw the square on the canvas
        context.set_fill_style_str(&square.color);
        context.fill_rect(
            square.x,
            square.y,
            square.side_length,
            square.side_length,
        );
    }
}
