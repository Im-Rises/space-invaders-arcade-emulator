use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture, WebGlVertexArrayObject};

pub struct MyWebGl2 {
    gl: WebGl2RenderingContext,
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    program: WebGlProgram,
    vertex_count: i32,
    game_texture: WebGlTexture,
    overlay_texture: WebGlTexture,
}

impl MyWebGl2 {
    pub fn new(canvas_id: String, canvas_width: u32, canvas_height: u32) -> Result<MyWebGl2, JsValue> {
        // Get the canvas element
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        // // Set the canvas width and height
        // canvas.set_width(canvas_height);
        // canvas.set_height(canvas_width);

        // Get the WebGL2 context
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        // Set the viewport
        context.viewport(0, 0, canvas_height as i32, canvas_width as i32);

        // Create the vertex shader
        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
            
                        precision highp float;

                        in vec4 a_texcoord;
                        
                        out vec2 v_texcoord;
                
                        void main() {
                            // v_texcoord = a_texcoord.xy * 0.5 + 0.5; // Normal display
                            // v_texcoord = vec2(a_texcoord.x * 0.5 + 0.5, 1.0 - (a_texcoord.y * 0.5 + 0.5)); // flip y
                            // v_texcoord = vec2(a_texcoord.y * 0.5 + 0.5, 1.0 - (a_texcoord.x * 0.5 + 0.5)); // Rotate by 90 degrees
                            v_texcoord = vec2(a_texcoord.y * 0.5 + 0.5, a_texcoord.x * 0.5 + 0.5); // Rotate by 90 degrees and flip y
                            gl_Position = a_texcoord;
                        }
                        "##,
        )?;

        // Create the fragment shader
        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es

                        precision highp float;
                        
                        in vec2 v_texcoord;
                        
                        uniform sampler2D u_texture;
                        uniform sampler2D u_overlay_texture;
                        
                        out vec4 o_outColor;
                
                        void main() {
                            // // No transparency
                            // o_outColor = texture(u_texture, v_texcoord);
                            
                            // // // Debug show
                            // o_outColor = texture(u_overlay_texture, v_texcoord);
                            // o_outColor = texture(u_texture, v_texcoord);
                            
                            vec4 color = texture(u_texture, v_texcoord);
                            if (color.rgb == vec3(0.0)) {
                                color.a = 0.0; // Set alpha to 0 for black color
                            } else {
                                // color.a = 1.0; // Set alpha to 1 for non-black colors
                                color = texture(u_overlay_texture, v_texcoord).rgba; // Set color to overlay texture color
                            }
                            o_outColor = color;
                        }
                        "##,
        )?;

        // Create the program
        let program = link_program(&context, &vert_shader, &frag_shader)?;

        // Vertices
        let vertices: [f32; 18] = [
            // first triangle
            1.0, 1.0, 0.0, // top right
            1.0, -1.0, 0.0, // bottom right
            -1.0, 1.0, 0.0, // top left
            // second triangle
            1.0, -1.0, 0.0, // bottom right
            -1.0, -1.0, 0.0, // bottom left
            -1.0, 1.0, 0.0, // top left
        ];

        // Create the VBO
        let position_attribute_location = context.get_attrib_location(&program, "a_texcoord");
        let vbo = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        // Fill the buffer with the vertices
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Create the VAO
        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        // Associate the VBO with the vertex attribute pointer for the position attribute of the vertex data
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        // Create the game texture
        let game_texture = context.create_texture().ok_or("Failed to create texture")?;
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&game_texture));

        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        // Create the overlay texture
        let overlay_texture = context.create_texture().ok_or("Failed to create texture")?;
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&overlay_texture));

        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        context.use_program(Some(&program));
        context.uniform1i(Some(&context.get_uniform_location(&program, "u_texture").unwrap()), 0);
        context.uniform1i(
            Some(&context.get_uniform_location(&program, "u_overlay_texture").unwrap()),
            1,
        );

        // Unbind the texture
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        // Unbind the VBO
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        // Unbind the VAO
        context.bind_vertex_array(None);

        // // Allow transparency
        // context.enable(WebGl2RenderingContext::BLEND);
        // context.enable(WebGl2RenderingContext::DEPTH_TEST);
        // context.blend_func(
        //     WebGl2RenderingContext::SRC_ALPHA,
        //     WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        // );
        // context.blend_equation(WebGl2RenderingContext::FUNC_ADD);

        // Create the struct
        Ok(MyWebGl2 {
            gl: context,
            vao,
            vbo,
            program,
            vertex_count: (vertices.len() / 3) as i32,
            game_texture,
            overlay_texture,
        })
    }

    pub fn u8array_to_game_texture(&self, data: &[u8], width: i32, height: i32) -> Result<WebGlTexture, JsValue> {
        let gl = &self.gl;
        let texture = &self.game_texture;

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

        // Unbind the texture
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(texture.clone())
    }

    pub fn u8array_to_overlay_texture(&self, data: &[u8], width: i32, height: i32) -> Result<WebGlTexture, JsValue> {
        let gl = &self.gl;
        let texture = &self.overlay_texture;

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(data),
        )?;

        // Unbind the texture
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Ok(texture.clone())
    }

    pub fn draw(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.gl.bind_vertex_array(Some(&self.vao));
        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vbo));

        self.gl.use_program(Some(&self.program));

        self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.game_texture));

        self.gl.active_texture(WebGl2RenderingContext::TEXTURE1);
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.overlay_texture));

        self.gl
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vertex_count);

        self.gl.bind_vertex_array(None);
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
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
