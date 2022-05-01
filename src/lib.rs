#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]

use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
    num::NonZeroU32,
    ops::Deref,
};

mod enums;
mod gl;

use gl::GlFns as RawContext;

pub use enums::*;
#[doc(hidden)]
pub use gl::types::*;

macro_rules! gl_structs {
    ($( $ident:ident ) +) => {
        $(
            #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $ident(NonZeroU32);

            impl $ident {
                /// Returns [Self] if inner isn't zero otherwise return None
                pub const fn new(inner: u32) -> Option<Self> {
                    if inner != 0 {
                       unsafe { Some(Self::new_unchecked(inner)) }
                    } else {
                        None
                    }
                }

                /// # Safety
                ///
                /// This struct is just wrapper around a NonZeroU32 so the same safety requirements as [NonZeroU32::new_unchecked] apply.
                pub const unsafe fn new_unchecked(inner: u32) -> Self {
                    Self(NonZeroU32::new_unchecked(inner))
                }

                /// Returns the underlying raw value. Usefull for interacting with other libraries, extensions, or yet to be exported features.
                pub const fn inner(&self) -> u32 {
                    self.0.get()
                }
            }
        )*
    };
}

gl_structs!(Buffer Shader Program VertexArray);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UniformLocation(u32);

impl UniformLocation {
    /// Returns the underlying raw value. Usefull for interacting with other libraries or yet to be exported features.
    pub const fn inner(&self) -> u32 {
        self.0
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
    pub unsafe fn load<F>(mut loader_function: F) -> Self
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        Self {
            raw: RawContext::load_with(|addr| loader_function(CStr::from_ptr(addr)) as *mut c_void),
        }
    }

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

    // TODO: Should this panic? Maybe returing a Result is better?.
    // Other possible approaches are making two different version but honestly this warrants an issue if and when the crates becames widely used.
    // Same goes to other functions that may panic
    pub unsafe fn generate_buffer(&self) -> Buffer {
        let mut id = 0;
        self.GenBuffers(1, &mut id);
        Buffer(NonZeroU32::new(id).expect("Buffer object id cannot be zero"))
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
        VertexArray(NonZeroU32::new(id).expect("Vertex array object id cannot be zero"))
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
        Shader(
            NonZeroU32::new(self.CreateShader(shader_type.into()))
                .expect("Shader object id cannot be zero"),
        )
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
        Program(NonZeroU32::new(self.CreateProgram()).expect("Buffer object id cannot be zero"))
    }

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
        let mut length = self.get_shader_parameter_i32(shader, ShaderParameter::InfoLogLength);

        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            self.GetShaderInfoLog(
                shader.inner(),
                length,
                &mut length,
                (&mut log[..]).as_mut_ptr().cast(),
            );
            log.truncate(length as usize);
            log
        } else {
            String::new()
        }
    }

    pub unsafe fn get_program_info_log(&self, program: Program) -> String {
        let mut length = self.get_program_parameter_i32(program, ProgramParameter::InfoLogLength);

        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            self.GetProgramInfoLog(
                program.inner(),
                length,
                &mut length,
                (&mut log[..]).as_mut_ptr().cast(),
            );
            log.truncate(length as usize);
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

    // TODO: Find a way to avoid the cost of allocating a CString and unwrapping
    pub unsafe fn get_uniform_location(
        &self,
        program: Program,
        name: &str,
    ) -> Option<UniformLocation> {
        let name = CString::new(name).unwrap();
        let uniform_location = self.GetUniformLocation(program.inner(), name.as_ptr());
        if uniform_location > -1 {
            Some(UniformLocation(uniform_location as u32))
        } else {
            None
        }
    }

    pub unsafe fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        // debug_assert!()
        self.Viewport(x, y, width as i32, height as i32)
    }

    pub unsafe fn get_error(&self) -> Error {
        self.GetError().into()
    }
}
