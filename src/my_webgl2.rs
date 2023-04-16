// use wasm_bindgen::prelude::*;
// use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlBuffer, WebGlProgram, WebGlShader};
//
// #[wasm_bindgen]
// pub struct MyWebGl2 {
//     gl: WebGl2RenderingContext,
//     vertex_buffer: WebGlBuffer,
//     color_buffer: WebGlBuffer,
//     program: WebGlProgram,
// }
//
// #[wasm_bindgen]
// impl MyWebGl2 {
//     pub fn new() -> MyWebGl2 {
//         let gl = MyWebGl2::mywebgl2_init().unwrap();
//         MyWebGl2 {
//             gl,
//             vertex_buffer: MyWebGl2::init_vertex_buffer(&gl).unwrap(),
//             color_buffer: MyWebGl2::init_color_buffer(&gl).unwrap(),
//             program: MyWebGl2::init_shader(&gl).unwrap(),
//         }
//     }
//
//     fn mywebgl2_init() -> Result<WebGl2RenderingContext, JsValue> {
//         let window = web_sys::window().expect("Error: Cannot get window");
//         let document = window.document().expect("Error: Cannot get document");
//         let canvas = document.get_element_by_id("canvas").expect("Error: Cannot get canvas");
//         let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Error: Cannot convert canvas to HtmlCanvasElement");
//         let gl = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;
//         Ok(gl)
//     }
//
//     fn init_vertex_buffer(gl: &WebGl2RenderingContext) -> Result<WebGlBuffer, JsValue> {
//         let vertices = vec![
//             0.0, 0.5, -0.5, -0.5, 0.5, -0.5,
//         ];
//         let vertices = wasm_bindgen::memory()
//             .dyn_into::<WebAssembly::Memory>()?
//             .buffer()
//             .slice(
//                 vertices.as_ptr() as u32 / 4,
//                 (vertices.as_ptr() as u32 + vertices.len() as u32 * 4) / 4,
//             );
//         let vertices = js_sys::Float32Array::new(&vertices);
//         let buffer = gl.create_buffer().ok_or("Error: Cannot create buffer")?;
//         gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
//         gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &vertices, WebGl2RenderingContext::STATIC_DRAW);
//         Ok(buffer)
//     }
//
//     fn init_color_buffer(gl: &WebGl2RenderingContext) -> Result<WebGlBuffer, JsValue> {
//         let colors = vec![
//             1.0, 0.0, 0.0, 1.0,
//             0.0, 1.0, 0.0, 1.0,
//             0.0, 0.0, 1.0, 1.0,
//         ];
//         let colors = wasm_bindgen::memory()
//             .dyn_into::<WebAssembly::Memory>()?
//             .buffer()
//             .slice(
//                 colors.as_ptr() as u32 / 4,
//                 (colors.as_ptr() as u32 + colors.len() as u32 * 4) / 4,
//             );
//         let colors = js_sys::Float32Array::new(&colors);
//         let buffer = gl.create_buffer().ok_or("Error: Cannot create buffer")?;
//         gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
//         gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &colors, WebGl2RenderingContext::STATIC_DRAW);
//         Ok(buffer)
//     }
// }
