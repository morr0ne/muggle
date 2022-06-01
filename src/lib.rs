#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]

use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
    ops::{Add, Deref, Sub},
};

pub mod gl;
pub mod phosphorus;

#[doc(hidden)]
pub use gl::types::*;
#[doc(hidden)]
pub use gl::Gl as RawContext;
pub use gl::{LoadError, Result};

mod enums;
mod objects;

pub use enums::*;
pub use objects::*;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Texture(u32);

impl Texture {
    pub const TEXTURE0: Self = Self(GL_TEXTURE0);
    pub const TEXTURE1: Self = Self(GL_TEXTURE1);
    pub const TEXTURE2: Self = Self(GL_TEXTURE2);
    pub const TEXTURE3: Self = Self(GL_TEXTURE3);
    pub const TEXTURE4: Self = Self(GL_TEXTURE4);
    pub const TEXTURE5: Self = Self(GL_TEXTURE5);
    pub const TEXTURE6: Self = Self(GL_TEXTURE6);
    pub const TEXTURE7: Self = Self(GL_TEXTURE7);
    pub const TEXTURE8: Self = Self(GL_TEXTURE8);
    pub const TEXTURE9: Self = Self(GL_TEXTURE9);
    pub const TEXTURE10: Self = Self(GL_TEXTURE10);
    pub const TEXTURE11: Self = Self(GL_TEXTURE11);
    pub const TEXTURE12: Self = Self(GL_TEXTURE12);
    pub const TEXTURE13: Self = Self(GL_TEXTURE13);
    pub const TEXTURE14: Self = Self(GL_TEXTURE14);
    pub const TEXTURE15: Self = Self(GL_TEXTURE15);
    pub const TEXTURE16: Self = Self(GL_TEXTURE16);
    pub const TEXTURE17: Self = Self(GL_TEXTURE17);
    pub const TEXTURE18: Self = Self(GL_TEXTURE18);
    pub const TEXTURE19: Self = Self(GL_TEXTURE19);
    pub const TEXTURE20: Self = Self(GL_TEXTURE20);
    pub const TEXTURE21: Self = Self(GL_TEXTURE21);
    pub const TEXTURE22: Self = Self(GL_TEXTURE22);
    pub const TEXTURE23: Self = Self(GL_TEXTURE23);
    pub const TEXTURE24: Self = Self(GL_TEXTURE24);
    pub const TEXTURE25: Self = Self(GL_TEXTURE25);
    pub const TEXTURE26: Self = Self(GL_TEXTURE26);
    pub const TEXTURE27: Self = Self(GL_TEXTURE27);
    pub const TEXTURE28: Self = Self(GL_TEXTURE28);
    pub const TEXTURE29: Self = Self(GL_TEXTURE29);
    pub const TEXTURE30: Self = Self(GL_TEXTURE30);
    pub const TEXTURE31: Self = Self(GL_TEXTURE31);

    pub const fn inner(&self) -> u32 {
        self.0
    }
}

impl Add for Texture {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Texture(self.0 + rhs.0)
    }
}

impl Sub for Texture {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Texture(self.0 - rhs.0)
    }
}

impl Add<u32> for Texture {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Texture(self.0 + rhs)
    }
}

impl Sub<u32> for Texture {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Texture(self.0 - rhs)
    }
}

pub struct Context {
    raw: RawContext,
}

impl Deref for Context {
    type Target = RawContext;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl Context {
    pub unsafe fn load<F>(loader_function: F) -> gl::Result<Self>
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
        self.ActiveShaderProgram(pipeline.inner(), program.inner())
    }

    /// [Texture](Texture) implements Add/Sub for u32, so (assuming you are in a valid gl context) you could for example write the following code
    /// ```no_run
    /// gl.set_active_texture(Texture::TEXTURE12 + 7)
    /// ```
    /// to get the texture unit at index 19
    /// # Errors
    /// GL_INVALID_ENUM is generated if the texture index is more than  GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS - 1
    pub unsafe fn set_active_texture(&self, texture: Texture) {
        self.ActiveTexture(texture.inner())
    }

    /// # Usage
    /// TODO
    /// OpenGL is mainly a C99 api so it returns a [CStr], this is converted using [to_string_lossy](CStr::to_string_lossy) hence why it returns a [Cow<str>]
    pub unsafe fn get_string(&self, string_parameter: StringParameter) -> Cow<str> {
        CStr::from_ptr(self.GetString(string_parameter.into()).cast()).to_string_lossy()
    }

    pub unsafe fn get_parameter_bool(&self, parameter: Parameter) -> bool {
        let mut value = 0;
        self.GetBooleanv(parameter.into(), &mut value);
        value != 0
    }

    pub unsafe fn get_parameter_f64(&self, parameter: Parameter) -> f64 {
        let mut value = 0.0;
        self.GetDoublev(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_parameter_f32(&self, parameter: Parameter) -> f32 {
        let mut value = 0.0;
        self.GetFloatv(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_parameter_i32(&self, parameter: Parameter) -> i32 {
        let mut value = 0;
        self.GetIntegerv(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_parameter_i64(&self, parameter: Parameter) -> i64 {
        let mut value = 0;
        self.GetInteger64v(parameter.into(), &mut value);
        value
    }

    pub unsafe fn get_shader_parameter_i32(
        &self,
        shader: Shader,
        shader_parameter: ShaderParameter,
    ) -> i32 {
        let mut parameter = 0;
        self.GetShaderiv(shader.inner(), shader_parameter.into(), &mut parameter);
        parameter
    }

    pub unsafe fn get_program_parameter_i32(
        &self,
        program: Program,
        program_parameter: ProgramParameter,
    ) -> i32 {
        let mut parameter = 0;
        self.GetProgramiv(program.inner(), program_parameter.into(), &mut parameter);
        parameter
    }

    pub unsafe fn generate_buffer(&self) -> Buffer {
        let mut id = 0;
        self.GenBuffers(1, &mut id);
        Buffer::new(id)
    }

    // TODO: Some idiomatic way to generate multiple buffers at once
    // pub unsafe fn generate_buffers<const N: usize>(&self, count: i32) -> [Buffer; N] {
    //     let mut buffers = [0; N];

    //     self.GenBuffers(count, buffers.as_mut_ptr());
    //     // (
    //     //     Buffer(NonZeroU32::new(ids[0]).expect("Buffer object id cannot be zero")),
    //     //     Buffer(NonZeroU32::new(ids[1]).expect("Buffer object id cannot be zero")),
    //     // )

    //     todo!()
    // }

    pub unsafe fn generate_vertex_array(&self) -> VertexArray {
        let mut id = 0;
        self.GenVertexArrays(1, &mut id);
        VertexArray::new(id)
    }

    pub unsafe fn bind_buffer(&self, target: BufferTarget, buffer: Buffer) {
        self.BindBuffer(target.into(), buffer.inner())
    }

    pub unsafe fn bind_vertex_array(&self, vertex_array: VertexArray) {
        self.BindVertexArray(vertex_array.inner())
    }

    pub unsafe fn buffer_data_bytes(&self, target: BufferTarget, data: &[u8], usage: BufferUsage) {
        self.BufferData(
            target.into(),
            data.len() as isize,
            data.as_ptr().cast(),
            usage.into(),
        );
    }

    pub unsafe fn create_shader(&self, shader_type: ShaderType) -> Shader {
        Shader::new(self.CreateShader(shader_type.into()))
    }

    pub unsafe fn shader_source(&self, shader: Shader, source: &str) {
        self.ShaderSource(
            shader.inner(),
            1,
            &(source.as_ptr().cast()),
            &(source.len() as GLint),
        )
    }

    pub unsafe fn compile_shader(&self, shader: Shader) {
        self.CompileShader(shader.inner())
    }

    pub unsafe fn delete_shader(&self, shader: Shader) {
        self.DeleteShader(shader.inner())
    }

    pub unsafe fn create_program(&self) -> Program {
        Program::new(self.CreateProgram())
    }

    /// # Errors
    /// GL_INVALID_OPERATION is generated if the shader has already been attached to the program
    pub unsafe fn attach_shader(&self, program: Program, shader: Shader) {
        self.AttachShader(program.inner(), shader.inner())
    }

    pub unsafe fn link_program(&self, program: Program) {
        self.LinkProgram(program.inner())
    }

    pub unsafe fn use_program(&self, program: Program) {
        self.UseProgram(program.inner())
    }

    pub unsafe fn get_shader_info_log(&self, shader: Shader) -> String {
        let mut len = self.get_shader_parameter_i32(shader, ShaderParameter::InfoLogLength);

        if len > 0 {
            let mut log = String::with_capacity(len as usize);
            log.extend(std::iter::repeat('\0').take(len as usize));
            self.GetShaderInfoLog(
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
            self.GetProgramInfoLog(
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
        self.VertexAttribPointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const c_void,
        );
    }

    pub unsafe fn enable_vertex_attribute_array(&self, index: u32) {
        self.EnableVertexAttribArray(index);
    }

    pub unsafe fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        self.DrawArrays(mode.into(), first, count)
    }

    pub unsafe fn draw_elements(&self, mode: DrawMode, count: i32, type_: GLenum, indices: i32) {
        self.DrawElements(mode.into(), count, type_, indices as *const c_void)
    }

    // TODO: Find a way to avoid the cost of allocating a CString and the potential panic of unwrapping
    pub unsafe fn get_uniform_location(
        &self,
        program: Program,
        name: &str,
    ) -> Option<UniformLocation> {
        let name = CString::new(name).unwrap();
        let uniform_location = self.GetUniformLocation(program.inner(), name.as_ptr());
        if uniform_location > -1 {
            Some(UniformLocation::new_unchecked(uniform_location as u32))
        } else {
            None
        }
    }

    pub unsafe fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        // debug_assert!()
        self.Viewport(x, y, width as i32, height as i32)
    }

    pub unsafe fn get_error(&self) -> Error {
        self.GetError().into()
    }

    pub unsafe fn uniform_4_f32(&self, location: UniformLocation, x: f32, y: f32, z: f32, w: f32) {
        self.Uniform4f(location.inner() as i32, x, y, z, w);
    }

    // TODO: strong typing
    pub unsafe fn begin_conditional_render(&self, id: GLuint, mode: GLenum) {
        self.BeginConditionalRender(id, mode)
    }

    pub unsafe fn end_conditional_render(&self) {
        self.EndConditionalRender()
    }
}
