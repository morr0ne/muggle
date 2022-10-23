#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]

use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
};

#[doc(hidden)]
pub use angel::types::*;
use angel::Gl as RawContext;
pub use angel::{LoadError, Result};

mod enums;
mod objects;

pub use enums::*;
pub use objects::*;

pub struct Context {
    raw: RawContext,
}

impl Context {
    pub const fn raw(&self) -> &RawContext {
        &self.raw
    }

    pub unsafe fn load<F>(loader_function: F) -> angel::Result<Self>
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        RawContext::load(loader_function).map(|raw| Self { raw })
    }

    /// # Usage
    /// TODO
    /// # Errors
    /// TODO: GL_INVALID_OPERATION when the program has not been linked successefully, somehow prevent that from happening.
    pub unsafe fn set_active_shader_program(&self, pipeline: PipeLine, program: Program) {
        self.raw
            .ActiveShaderProgram(pipeline.inner(), program.inner())
    }

    /// [Texture](Texture) implements Add/Sub for u32, so (assuming you are in a valid gl context) you could for example write the following code
    /// ```no_run
    /// gl.set_active_texture(Texture::TEXTURE12 + 7)
    /// ```
    /// to get the texture unit at index 19
    /// # Errors
    /// GL_INVALID_ENUM is generated if the texture index is more than GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS - 1
    pub unsafe fn set_active_texture(&self, texture: Texture) {
        self.raw.ActiveTexture(texture.inner())
    }

    /// # Usage
    /// TODO
    /// OpenGL is mainly a C99 api so it returns a [CStr], this is converted using [to_string_lossy](CStr::to_string_lossy) hence why it returns a [Cow<str>]
    pub unsafe fn get_string(&self, string_parameter: StringParameter) -> Cow<str> {
        CStr::from_ptr(self.raw.GetString(string_parameter.into()).cast()).to_string_lossy()
    }

    pub unsafe fn get_parameter_bool(&self, parameter: Parameter) -> bool {
        let mut value = 0;
        self.raw.GetBooleanv(parameter.into(), &mut value);
        value != 0
    }

    pub unsafe fn get_parameter_f64(&self, parameter: Parameter) -> f64 {
        let mut value = 0.0;
        self.raw.GetDoublev(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_parameter_f32(&self, parameter: Parameter) -> f32 {
        let mut value = 0.0;
        self.raw.GetFloatv(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_parameter_i32(&self, parameter: Parameter) -> i32 {
        let mut value = 0;
        self.raw.GetIntegerv(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_parameter_i64(&self, parameter: Parameter) -> i64 {
        let mut value = 0;
        self.raw.GetInteger64v(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_shader_parameter_i32(
        &self,
        shader: Shader,
        shader_parameter: ShaderParameter,
    ) -> i32 {
        let mut parameter = 0;
        self.raw
            .GetShaderiv(shader.inner(), shader_parameter.into(), &mut parameter);
        parameter
    }

    pub unsafe fn get_program_parameter_i32(
        &self,
        program: Program,
        program_parameter: ProgramParameter,
    ) -> i32 {
        let mut parameter = 0;
        self.raw
            .GetProgramiv(program.inner(), program_parameter.into(), &mut parameter);
        parameter
    }

    pub unsafe fn generate_buffer(&self) -> Buffer {
        let mut id = 0;
        self.raw.GenBuffers(1, &mut id);
        Buffer::new(id)
    }

    // TODO: Some idiomatic way to generate multiple buffers at once
    // pub unsafe fn generate_buffers<const N: usize>(&self, count: i32) -> [Buffer; N] {
    //     let mut buffers = [0; N];

    //     self.raw.GenBuffers(count, buffers.as_mut_ptr());
    //     // (
    //     //     Buffer(NonZeroU32::new(ids[0]).expect("Buffer object id cannot be zero")),
    //     //     Buffer(NonZeroU32::new(ids[1]).expect("Buffer object id cannot be zero")),
    //     // )

    //     todo!()
    // }

    pub unsafe fn generate_vertex_array(&self) -> VertexArray {
        let mut id = 0;
        self.raw.GenVertexArrays(1, &mut id);
        VertexArray::new(id)
    }

    pub unsafe fn bind_buffer(&self, target: BufferTarget, buffer: Buffer) {
        self.raw.BindBuffer(target.into(), buffer.inner())
    }

    pub unsafe fn bind_vertex_array(&self, vertex_array: VertexArray) {
        self.raw.BindVertexArray(vertex_array.inner())
    }

    pub unsafe fn buffer_data_bytes(&self, target: BufferTarget, data: &[u8], usage: BufferUsage) {
        self.raw.BufferData(
            target.into(),
            data.len() as isize,
            data.as_ptr().cast(),
            usage.into(),
        );
    }

    pub unsafe fn create_shader(&self, shader_type: ShaderType) -> Shader {
        Shader::new(self.raw.CreateShader(shader_type.into()))
    }

    pub unsafe fn shader_source(&self, shader: Shader, source: &str) {
        self.raw.ShaderSource(
            shader.inner(),
            1,
            &(source.as_ptr().cast()),
            &(source.len() as GLint),
        )
    }

    pub unsafe fn compile_shader(&self, shader: Shader) {
        self.raw.CompileShader(shader.inner())
    }

    pub unsafe fn delete_shader(&self, shader: Shader) {
        self.raw.DeleteShader(shader.inner())
    }

    pub unsafe fn create_program(&self) -> Program {
        Program::new(self.raw.CreateProgram())
    }

    /// # Errors
    /// GL_INVALID_OPERATION is generated if the shader has already been attached to the program
    pub unsafe fn attach_shader(&self, program: Program, shader: Shader) {
        self.raw.AttachShader(program.inner(), shader.inner())
    }

    pub unsafe fn link_program(&self, program: Program) {
        self.raw.LinkProgram(program.inner())
    }

    pub unsafe fn use_program(&self, program: Program) {
        self.raw.UseProgram(program.inner())
    }

    pub unsafe fn get_shader_info_log(&self, shader: Shader) -> String {
        let mut len = self.get_shader_parameter_i32(shader, ShaderParameter::InfoLogLength);

        if len > 0 {
            let mut log = String::with_capacity(len as usize);
            log.extend(std::iter::repeat('\0').take(len as usize));
            self.raw.GetShaderInfoLog(
                shader.inner(),
                len,
                &mut len,
                (&mut log[..]).as_mut_ptr().cast(),
            );
            log.truncate(len as usize);
            log
        } else {
            String::new()
        }
    }

    pub unsafe fn get_program_info_log(&self, program: Program) -> String {
        let mut len = self.get_program_parameter_i32(program, ProgramParameter::InfoLogLength);

        if len > 0 {
            let mut log = String::with_capacity(len as usize);
            log.extend(std::iter::repeat('\0').take(len as usize));
            self.raw.GetProgramInfoLog(
                program.inner(),
                len,
                &mut len,
                (&mut log[..]).as_mut_ptr().cast(),
            );
            log.truncate(len as usize);
            log
        } else {
            String::new()
        }
    }

    pub unsafe fn vertex_attribute_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.raw.VertexAttribPointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const c_void,
        );
    }

    pub unsafe fn enable_vertex_attribute_array(&self, index: u32) {
        self.raw.EnableVertexAttribArray(index);
    }

    pub unsafe fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        self.raw.DrawArrays(mode.into(), first, count)
    }

    pub unsafe fn draw_elements(&self, mode: DrawMode, count: i32, type_: GLenum, indices: i32) {
        self.raw
            .DrawElements(mode.into(), count, type_, indices as *const c_void)
    }

    // TODO: Find a way to avoid the cost of allocating a CString and the potential panic of unwrapping
    pub unsafe fn get_uniform_location(
        &self,
        program: Program,
        name: &str,
    ) -> Option<UniformLocation> {
        let name = CString::new(name).unwrap();
        let uniform_location = self.raw.GetUniformLocation(program.inner(), name.as_ptr());
        if uniform_location > -1 {
            Some(UniformLocation::new_unchecked(uniform_location as u32))
        } else {
            None
        }
    }

    pub unsafe fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        // debug_assert!()
        self.raw.Viewport(x, y, width as i32, height as i32)
    }

    pub unsafe fn get_error(&self) -> Error {
        self.raw.GetError().into()
    }

    pub unsafe fn uniform_4_f32(&self, location: UniformLocation, x: f32, y: f32, z: f32, w: f32) {
        self.raw.Uniform4f(location.inner() as i32, x, y, z, w);
    }

    // TODO: strong typing
    pub unsafe fn begin_conditional_render(&self, id: GLuint, mode: GLenum) {
        self.raw.BeginConditionalRender(id, mode)
    }

    pub unsafe fn end_conditional_render(&self) {
        self.raw.EndConditionalRender()
    }

    pub unsafe fn clear(&self, mask: Mask) {
        self.raw.Clear(mask.inner())
    }

    pub unsafe fn set_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.raw.ClearColor(red, green, blue, alpha)
    }
}
