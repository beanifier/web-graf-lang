use graf_lang::{interpreter::Runtime, parser::graf::program};
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use web_sys::{
    Document, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlTextAreaElement, window,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = document();

    let input = document
        .get_element_by_id("input")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()?;

    let console = document
        .get_element_by_id("console")
        .unwrap()
        .dyn_into::<HtmlElement>()?;

    let run_btn = document
        .get_element_by_id("run")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()?;

    let step_btn = document
        .get_element_by_id("step")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()?;

    let step_count = document
        .get_element_by_id("stepcount")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;

    let runtime = Arc::new(RwLock::new(None));

    {
        let input = input.clone();
        let console = console.clone();
        let runtime = runtime.clone();
        let button = run_btn.clone();
        let stepbuton = step_btn.clone();

        let cb = Closure::<dyn Fn()>::new(move || {
            console.set_inner_text("");

            let mut a = runtime.write().unwrap();
            if let Some(_) = a.as_ref() {
                button.set_inner_text("Run");
                stepbuton.set_attribute("disabled", "").unwrap();
                *a = None;
            } else {
                let rrrr = runtime.clone();
                let consolee = console.clone();
                let prog = program(&input.value());
                if let Err(x) = prog {
                    println!("{:?}", x);
                    console.set_inner_text(&x.to_string());
                    return;
                }
                *a = Some(Runtime::with_io_fn(
                    prog.unwrap(),
                    Vec::new(),
                    Box::new(move |x| {
                        let mut text = consolee.inner_text();
                        text.push(x as char);
                        consolee.set_inner_text(&text);
                    }),
                    Box::new(move || {
                        match window().unwrap().prompt_with_message("input char").unwrap() {
                            Some(s) => s.chars().next().unwrap() as u8,
                            None => {
                                let mut a = rrrr.write().unwrap();
                                *a = None;
                                0
                            }
                        }
                    }),
                ));
                button.set_inner_text("Stop");
                stepbuton.remove_attribute("disabled").unwrap();
            }
        });

        run_btn.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    {
        let runtime = runtime.clone();

        let button = run_btn.clone();
        let stepbuton = step_btn.clone();

        let stp = step_count.clone();

        let cb = Closure::<dyn Fn()>::new(move || {
            let b = stp.value().parse::<u32>().unwrap_or(0).max(1);
            let mut a = runtime.write().unwrap();
            for _ in 0..b {
                if let Some(rt) = a.as_mut() {
                    if rt.step() {
                        *a = None;
                        button.set_inner_text("Run");
                        stepbuton.set_attribute("disabled", "").unwrap();
                    }
                } else {
                    break;
                }
            }
        });

        step_btn.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    Ok(())
}

fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}
