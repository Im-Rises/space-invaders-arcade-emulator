use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture, WebGlVertexArrayObject};

//AudioBuffer, AudioBufferSourceNode, AudioContext,

pub struct MyWebGl2 {
    gl: WebGl2RenderingContext,
    vao: WebGlVertexArrayObject,
    vertex_buffer: WebGlBuffer,
    program: WebGlProgram,
    texture: WebGlTexture,
    vertex_count: i32,
    // audio_context: AudioContext,
}

impl MyWebGl2 {
    pub fn new(canvas_width: u32, canvas_height: u32) -> Result<MyWebGl2, (JsValue)> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .get_element_by_id("canvas")
            .expect("should have a canvas element with id `canvas`")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .expect("canvas element should be a `HtmlCanvasElement`");

        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);

        let gl = canvas
            .get_context("webgl2")
            .expect("should have a webgl2 context")
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        // Create shader
        let vert_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
            in vec2 a_texcoord;
            void main() {
                gl_Position = vec4(a_texcoord, 0, 1);
            }
            "##,
        )?;

        let frag_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
            precision highp float;
            uniform sampler2D u_image;
            out vec4 outColor;
            void main() {
                outColor = texture(u_image, gl_FragCoord.xy);
            }
            "##,
        )?;

        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        // Create vertex buffer
        let vertices: [f32; 18] = [
            // first triangle
            0.5, 0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
            -0.5, 0.5, 0.0, // top left
            // second triangle
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom left
            -0.5, 0.5, 0.0, // top left
        ];
        let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Create VAO
        let vao = gl.create_vertex_array().ok_or("failed to create VAO").unwrap();
        gl.bind_vertex_array(Some(&vao));

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        // create texture
        let texture = gl.create_texture().ok_or("failed to create texture")?;
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(MyWebGl2 {
            gl,
            vao,
            vertex_buffer,
            program,
            texture,
            vertex_count: (vertices.len() / 3) as i32,
        })
    }

    pub fn u8array_to_texture(&self, data: &[u8], width: i32, height: i32) -> Result<WebGlTexture, JsValue> {
        let gl = &self.gl;
        let texture = &self.texture;

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(data),
        )?;

        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(texture.clone())
    }

    pub fn draw(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.gl.bind_vertex_array(Some(&self.vao));
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));

        self.gl.use_program(Some(&self.program));
        self.gl
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vertex_count);

        self.gl.bind_vertex_array(None);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }
}

fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

// pub fn play_mp3(mp3_data: &[u8]) {
//     // create a new AudioContext
//     let context = AudioContext::new().unwrap();
//
//     // create an AudioBuffer from the MP3 data
//     context
//         .decode_audio_data(mp3_data)
//         .unwrap()
//         .then(|result| {
//             let buffer = result.unwrap();
//             let source = context.create_buffer_source().unwrap();
//             source.set_buffer(Some(&buffer));
//             source.connect_with_audio_node(&context.destination()).unwrap();
//             source.start().unwrap();
//             Ok(())
//         })
//         .unwrap();
// }
