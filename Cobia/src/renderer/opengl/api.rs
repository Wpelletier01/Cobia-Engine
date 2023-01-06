#![allow(dead_code)]

use std::ffi::{CString,c_void};
use std::ptr;

use super::EOpenGL;
use crate::core::logs::{CERRORS};
use std::os::raw::{c_uint,c_int,c_char};
//
//
type GLenum =       u32;
type GLuint =       c_uint;
type GLint  =       c_int;
type GLchar =       c_char;
type GLbitfield =   c_uint;
//
//
// ------------------------------------------------------------------------------------------------
// Constant 
//
const BUFFER_SIZE: i32 = 16;
//
// ------------------------------------------------------------------------------------------------
// Error Management
//
/// Store possible causes of error
struct MsgMashingType{

    msg_:   String,
    type_:  GlErrTypes,

}
//
impl MsgMashingType {
    //
    // 
    fn new(msg:&str,_type:GlErrTypes) -> Self{

        MsgMashingType{
            msg_:msg.to_string(),
            type_:_type,
        }

    }
    //
}
//
//
/// Different type of possible cause of error that opengl can generate 
#[derive(PartialEq)]
enum GlErrTypes{
    
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidFramebufferOperation,
    Unknown

}
//
impl GlErrTypes{
    //
    // return opengl error type to string
    fn to_string(&self) -> String{

        return match *self {

            GlErrTypes::InvalidEnum =>                  "INVALID_ENUM".to_string(),
            GlErrTypes::InvalidValue =>                 "INVALID_VALUE".to_string(),
            GlErrTypes::InvalidOperation =>             "INVALID_OPERATION".to_string(),
            GlErrTypes::StackOverflow =>                "STACK_OVERFLOW".to_string(),
            GlErrTypes::StackUnderflow =>               "STACK_UNDERFLOW".to_string(),
            GlErrTypes::OutOfMemory =>                  "OUT_OF_MEMORY".to_string(),
            GlErrTypes::InvalidFramebufferOperation =>  "INVALID_FRAMEBUFFER_OPERATION".to_string(),
            GlErrTypes::Unknown =>                      "UNKNOWN".to_string(),

        }
    }
}
//
//
/// Check if opengl raised a flag to signal the type of error that occurred
fn check_error_status() -> Option<GlErrTypes>{


    let err_value:GLenum;

    unsafe {

        err_value = gl::GetError();

    }
    
    // check if a flag have been raised
    if err_value != gl::NO_ERROR{
        
        return match err_value {
            gl::INVALID_ENUM => Some(GlErrTypes::InvalidEnum),
            gl::INVALID_VALUE => Some(GlErrTypes::InvalidValue),
            gl::INVALID_OPERATION => Some(GlErrTypes::InvalidOperation),
            gl::STACK_OVERFLOW => Some(GlErrTypes::StackOverflow),
            gl::STACK_UNDERFLOW => Some(GlErrTypes::StackUnderflow),
            gl::OUT_OF_MEMORY => Some(GlErrTypes::OutOfMemory),
            gl::INVALID_FRAMEBUFFER_OPERATION =>
                Some(GlErrTypes::InvalidFramebufferOperation),
            _ => Some(GlErrTypes::Unknown)
        }
    }
    None
}
//
//
/// Check if opengl raised an error and if so return message that the contained the possible cause 
/// of error that the Opengl documentation says
fn check_for_error(func_name: &str, possible_msg: Vec<MsgMashingType>) -> Result<(),EOpenGL> {
        
    let err_type = match check_error_status() {

        Some(e) => e,
        None => return Ok(())

    };

    let mut index:usize = 0;

    if possible_msg.len() > 1{

        index = match possible_msg.iter().position(|e| e.type_ == err_type) {

            Some(i) => i,
            None => 
                return Err(EOpenGL::API_CALL(func_name.to_string(), "unknown reason".to_string()))

        };

    }
    
    Err(EOpenGL::API_CALL(func_name.to_string(), possible_msg[index].msg_.to_string()))

}
//
//
/// Just help remove boilerplate code from same possible error causes message
pub(crate) fn check_for_uniform_specifier(func_name: &str) -> Result<(),EOpenGL> {

    check_for_error(
        func_name,
        vec![
            MsgMashingType::new(
                "Possible error caused:\n
                    -   The program passed does not refer to a program object owned by the GL\n
                    -   The size of the uniform variable declared in the shader does not match
                        the size indicated by the glProgramUniform command\n
                    -   One of the signed or unsigned integer variants of this function is used to
                        load a uniform variable or if one of the floating-point variants of this
                        function is used to load a uniform variable\n
                    -   The parameter 'location' is an invalid uniform location for program and
                        location is not equal to -1\n
                    -   The parameter 'count' is greater than 1 and the indicated uniform variable
                        is not an array variable \n
                    -   A sampler is loaded using a command other than glProgramUniform1i and
                        glProgramUniform1iv\n",
                GlErrTypes::InvalidOperation,
            ),
            MsgMashingType::new(
                "the parameter 'count' is less than 0",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())
}
//
//
// ------------------------------------------------------------------------------------------------
// Function 
//
// ------------------------------------------------------------------------------------------------
// Shader 
// 
/// Creates a shader object
pub(crate) fn create_shader(shader_type:GLenum) -> Result<u32,EOpenGL>  {

    let _id:u32;
    
    unsafe{
        _id = gl::CreateShader(shader_type);
    }


    check_for_error(
        "glCreateShader",
        vec![
        MsgMashingType::new(
            "The parameter `shader_type` is not an accepted value",
            GlErrTypes::InvalidEnum,

        )
        ]
    )?;

    Ok(_id)

}
//
//
/// Replaces the source code in a shader object
pub(crate) fn shader_source(shader:&u32,content:&CString) -> Result<(),EOpenGL>{
    //
    unsafe{
        gl::ShaderSource(*shader,1,&content.as_ptr(),ptr::null_mut());

    }

    check_for_error(
        "glShaderSource",
        vec![
            MsgMashingType::new(
                "possible cause of error:\n
                     - The parameter 'shader' is not a value generated by Opengl\n
                     - The parameter 'count' is less than 0\n",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "the parameter 'shader' is not a shader object",
                GlErrTypes::InvalidOperation,

            )
        ],
    )?;

    Ok(())
}
//
//
/// Compiles a shader object
pub(crate) fn compile_shader(shader_id:&u32) -> Result<(),EOpenGL>{
    //
    unsafe{
        gl::CompileShader(*shader_id);
    }

    check_for_error(
        "glCompileShader",
        vec![
            MsgMashingType::new(
                "The parameter 'shader' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "the parameter 'shader' is not a shader object",
                GlErrTypes::InvalidOperation,

            )
        ],
    )?;

    Ok(())

}
//
//
/// Returns a parameter from a shader object
pub(crate) fn get_shader_iv(shader_id:&u32,pname:GLenum) -> Result<i32,EOpenGL>{


    let mut status:i32 = 0;

    unsafe{

        gl::GetShaderiv(*shader_id,pname,&mut status);
    }


    check_for_error(
        "glGetShaderiv",
        vec![

            MsgMashingType::new(
                "The parameter 'shader' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,


            ),

            MsgMashingType::new(
                "The parameter shader does not refer to a shader object",
                GlErrTypes::InvalidOperation,

            ),

            MsgMashingType::new(
                "the parameters pname is not an accepted value",
                GlErrTypes::InvalidEnum,

            )
        ],

    )?;

    Ok(status)

}
//
//
/// Returns the information log for a shader object
pub(crate) fn get_shader_info_log(shader_id:&u32,mut info_log:Vec<u8>,mut size:i32) -> Result<String,EOpenGL>{

    unsafe{

        gl::GetShaderInfoLog(
            *shader_id,
            size,
            &mut size,
            info_log.as_mut_ptr() as *mut _,

        );
    }

    check_for_error(
        "glGetShaderInfoLog",
        vec![

            MsgMashingType::new(
                "possible cause of error:\n
                     - The parameter 'shader' is not a value generated by Opengl\n
                     - The parameter 'maxlength' is less than 0\n",
                GlErrTypes::InvalidValue,

            ),

            MsgMashingType::new(
                "The parameter 'shader' does not refer to a shader object",
                GlErrTypes::InvalidOperation,

            ),

        ],

    )?;

    unsafe {

        info_log.set_len(size as usize);

    }

    let msg = String::from_utf8(info_log).map_err(|e|
        {
            CERRORS("Unable to convert vec to string that describe error for shader compilation Reason: {}",
            &[&e.to_string()]);
        }
    ).unwrap();

    println!("the reason {}",msg);
    
    Ok(msg)

}
//
//
/// Deletes a shader object
pub(crate) fn delete_shader(shader_id:&u32) -> Result<(),EOpenGL>{

    unsafe{

        gl::DeleteShader(*shader_id);

    }

    check_for_error(
        "glDeleteShader",
        vec![
            MsgMashingType::new(
                "The parameter 'shader' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())

}
// 
//
// ------------------------------------------------------------------------------------------------
// Program
//
/// Creates a program object
pub(crate) fn create_program() -> Result<u32,EOpenGL>{

    let id:u32;
    unsafe{
        id = gl::CreateProgram();
    }

    if id == 0{

        return Err(EOpenGL::API_CALL(
            "glCreateProgram".to_string(),
            "The program creation returned an value of 0".to_string()
            )
        );

    }

    Ok(id)
}
//
//
/// Attaches a shader object to a program object
pub(crate) fn attach_shader(program: &u32,shader: &u32) -> Result<(),EOpenGL>{
    //
    unsafe{
        gl::AttachShader(*program, *shader);
    }

    check_for_error(
        "glAttachShader",
        vec![
            MsgMashingType::new(
                "the parameter 'program' or 'shader' is not a value generated by \
                     Opengl",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                     -  The parameter 'program' is not a program  object\n
                     -  The shader passed is already attached to the program\n
                     -  The parameter shader does not refer to a shader object\n",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(())
}
//
//
/// Links a program object
pub(crate) fn link_program(program:&u32) -> Result<(),EOpenGL>{
    //
    unsafe{
        gl::LinkProgram(*program);
    }

    check_for_error(
        "glLinkProgram",
        vec![
            MsgMashingType::new(
                "the parameter 'program' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                     -  The parameter 'program' does not refer to a program object\n
                     -  The program object is he currently active and transform feedback mode is \
                        active\n",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(())
}
//
//
//
/// Returns value for a parameter from a program object for boolean parameter kind
pub(crate) fn get_program_iv(program: &u32,pname:GLenum) -> Result<i32,EOpenGL>{
    //
    let mut result = gl::FALSE as i32;

    unsafe{

        gl::GetProgramiv(*program, pname,&mut result);
    }

    check_for_error(
        "glGetProgramiv",
        vec![
            MsgMashingType::new(
                "the parameter 'program' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                     -  The parameter 'program' does not refer to a program object\n
                     -  The parameter 'pname'  is GL_GEOMETRY_VERTICES_OUT, \
                        GL_GEOMETRY_INPUT_TYPE, or GL_GEOMETRY_OUTPUT_TYPE and program does not \
                        contain a geometry shader \
                        or pname is GL_COMPUTE_WORK_GROUP_SIZE and program does not contain a \
                        binary for the compute shader stage\n",
                GlErrTypes::InvalidOperation,

            ),
            MsgMashingType::new(
                "the parameters program is not an accepted value",
                GlErrTypes::InvalidEnum,

            )
        ],

    )?;

    Ok(result)
}
//
//
//
/// Returns value for a parameter from a program object for boolean parameter kind
pub(crate) fn get_program_iv_bool(program: &GLuint, pname:GLenum) -> Result<GLint,EOpenGL>{
    //
    let mut result = gl::FALSE as GLint;

    unsafe{

        gl::GetProgramiv(*program, pname,&mut result);
    }

    check_for_error(
        "glGetProgramiv",
        vec![
            MsgMashingType::new(
                "the parameter 'program' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                     -  The parameter 'program' does not refer to a program object\n
                     -  The parameter 'pname'  is GL_GEOMETRY_VERTICES_OUT, \
                        GL_GEOMETRY_INPUT_TYPE, or GL_GEOMETRY_OUTPUT_TYPE and program does not \
                        contain a geometry shader \
                        or pname is GL_COMPUTE_WORK_GROUP_SIZE and program does not contain a \
                        binary for the compute shader stage\n",
                GlErrTypes::InvalidOperation,

            ),
            MsgMashingType::new(
                "the parameters program is not an accepted value",
                GlErrTypes::InvalidEnum,

            )
        ],

    )?;

    Ok(result)

}
//
//
//
/// Returns information about an active uniform variable for the specified program object
pub(crate) fn get_active_uniform(program: &u32,index:u32,) -> Result<(String,GLenum),EOpenGL>{


    let size:i32 = 0;
    let _type:GLenum = 0;

    let _name:Vec<u8> = Vec::with_capacity(BUFFER_SIZE as usize);
    let length:i32 = 0;

    unsafe{

        gl::GetActiveUniform(
            *program,
            index,
            BUFFER_SIZE,
            length as *mut i32,
            size as *mut i32,
            _type as *mut u32,
            _name.as_ptr() as *mut i8);

    }

    check_for_error(
        "glGetActiveUniform",
        vec![
            MsgMashingType::new(
                "possible cause of error:\n\
                     -  the parameter 'program' is not a value generated by Opengl\n
                     -  The parameter 'index' is greater than or equal to the number of active \
                        uniform variables in the parameter program or the parameter 'bufsize' \
                        is less than 0\n",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "The parameter 'program' does not refer to a program object",
                GlErrTypes::InvalidOperation,

            )
        ],
    )?;

    Ok((String::from_utf8(_name).unwrap(),_type))
}
//
//
//
///  Returns the information log for a program object
pub(crate) fn get_program_info_log(program:&u32,info_log:Vec<u8>) -> Result<String,EOpenGL> {

    unsafe{

        gl::GetProgramInfoLog(
            *program,
            info_log.len() as i32,
            ptr::null_mut(),
            info_log.as_ptr() as *mut GLchar);

    }

    check_for_error(
        "glGetProgramInfoLog",
        vec![
            MsgMashingType::new(
                "possible cause of error:\n\
                     -  The parameter 'program' is not a value generated by Opengl\n
                     -  The parameter 'maxLength' is less than 0\n",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "The parameter 'program' does not refer to a program object",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(String::from_utf8(info_log).unwrap())
}
//
//
/// Installs a program object as part of current rendering state
pub(crate) fn use_program(program_id:&u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::UseProgram(*program_id);
    }

    check_for_error(
        "glUseProgram",
        vec![
            MsgMashingType::new(
                "the parameter program is neither 0 nor a value generated by OpenGL",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "The parameter 'program' does not refer to a program object",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(())
}
//
//
//
/// Deletes a program object
pub(crate) fn delete_program(program_id:&u32) -> Result<(),EOpenGL>{


    unsafe{
        gl::DeleteProgram(*program_id);
    }

    check_for_error(
        "glDeleteProgram",
        vec![
            MsgMashingType::new(
                "The parameter 'program' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())

}
//
//
/// Returns the location of a uniform variable
pub(crate) fn get_uniform_location(prog_id:&u32, name:CString) -> Result<u32,EOpenGL>{

    let uni_id:i32;


    unsafe{
        uni_id =  gl::GetUniformLocation(*prog_id, name.as_ptr());
    }

    check_for_error(
        "glGetUniformLocation",
        vec![
            MsgMashingType::new(
                "The parameter 'program' is not a value generated by Opengl",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                    - The parameter 'program' does not refer to a program object\n
                    - The parameter 'program' has not been successfully linked\n",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(uni_id as u32)
}
//
//
// ------------------------------------------------------------------------------------------------
// UNIFORM
//
//
///  Specify the value of a uniform variable for a specified program object
pub(crate) fn uniform_matrix_4fv(
    uniform_id:u32,
    count:i32,
    transpose:bool,
    data: nalgebra_glm::Mat4x4 ) -> Result<(),EOpenGL>{

    unsafe{

        gl::UniformMatrix4fv(
            uniform_id as i32,
            count,
            transpose as u8,
            data.as_ptr());
    }

    check_for_uniform_specifier("glUniformMatrix4fv")?;

    Ok(())
}
//
//
///  Specify the value of a uniform variable for a specified program object
pub(crate) fn uniform_int_1(uniform_id:u32,value:i32) -> Result<(),EOpenGL>{

    unsafe{

        gl::Uniform1i(uniform_id as i32,value);
    }

    check_for_uniform_specifier("glUniform1i")?;

    Ok(())
}
//
//
///  Specify the value of a uniform variable for a specified program object
pub(crate) fn uniform_int_2(uniform_id:u32,value:i32,value_2: i32) -> Result<(),EOpenGL>{

    unsafe{

        gl::Uniform2i(uniform_id as i32,value,value_2);
    }

    check_for_uniform_specifier("glUniform2i")?;

    Ok(())
}
//
//
///  Specify the value of a uniform variable for a specified program object
pub(crate) fn uniform_int_3(uniform_id:u32,value:i32,value_2:i32,value_3:i32) -> Result<(),EOpenGL>{

    unsafe{

        gl::Uniform3i(uniform_id as i32,value,value_2,value_3);
    }

    check_for_uniform_specifier("glUniform3i")?;

    Ok(())
}
//
//
///  Specify the value of a uniform variable for a specified program object
pub(crate) fn uniform_int_4(
    uniform_id:i32,
    value:i32,
    value_2:i32,
    value_3:i32,
    value_4:i32) -> Result<(),EOpenGL>{

    unsafe{

        gl::Uniform4i(uniform_id as i32,value,value_2,value_3,value_4);
    }

    check_for_uniform_specifier("glUniform3i")?;

    Ok(())
}
//
//
// ------------------------------------------------------------------------------------------------
// BUFFER
//
//
/// generate vertex.rs array object names
pub(crate) fn gen_vertex_array(nb:i32,array_id:*mut u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::GenVertexArrays(nb, array_id);
    }

    check_for_error(
        "glGenVertexArrays",
        vec![
            MsgMashingType::new(
                "The parameter 'n' is negative",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())
}
//
//
/// bind a vertex.rs array object
pub(crate) fn bind_vertex_array(array:u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::BindVertexArray(array);
    }

    check_for_error(
        "glBindVertexArray",
        vec![
            MsgMashingType::new(
                "The parameter 'array' is not zero or the name of a vertex array object \
                    previously returned from a call to glGenVertexArrays",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(())
}
//
//
/// generate buffer object names
pub(crate) fn gen_buffer(nb:i32,buffer_id:*mut u32) -> Result<(),EOpenGL>{

    unsafe{

        gl::GenBuffers(nb, buffer_id );

    }

    check_for_error(
        "glGenBuffers",
        vec![
            MsgMashingType::new(
                "The parameter 'n' is negative",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())
}
//
//
/// bind a named buffer object
pub(crate) fn bind_buffer(type_:GLenum,buffer_id:u32) -> Result<(),EOpenGL>{

    unsafe{

        gl::BindBuffer(type_, buffer_id);

    }

    check_for_error(
        "glBindBuffer",
        vec![
            MsgMashingType::new(
                "The parameter 'target' is not one of the allowable values",
                GlErrTypes::InvalidEnum,

            ),
            MsgMashingType::new(
                "The parameter 'buffer' is not a name previously returned from a call to \
                    glGenBuffers",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())
}
//
//
/// creates and initializes a buffer object's data store
pub(crate) fn buffer_data(
    target: GLenum,
    size: isize,
    data: *const c_void,
    usage: GLenum) -> Result<(),EOpenGL>{

    unsafe{

        gl::BufferData(target, size, data, usage);

    }

    check_for_error(
        "glBufferData",
        vec![
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                    - The parameter 'target' is not one of the accepted buffer targets\n
                    - The parameter 'usage' is not GL_STREAM_DRAW\n
                    ",
                GlErrTypes::InvalidEnum,

            ),

            MsgMashingType::new(
                "possible cause of error:\n
                     - The reserved buffer object name 0 is bound to targets\n
                     - The GL_BUFFER_IMMUTABLE_STORAGE flag of the buffer object is GL_TRUE\n
                     ",
                GlErrTypes::InvalidOperation,

            ),

            MsgMashingType::new(
                "The parameter 'size' is negative",
                GlErrTypes::InvalidValue,

            ),

            MsgMashingType::new(
                "Unable to create a data store with the specified by the 'size' parameter",
                GlErrTypes::OutOfMemory,

            )

        ],

    )?;

    Ok(())

}
//
//
/// define an array of generic vertex.rs attribute data
pub(crate) fn vertex_attrib_pointer(
    index:      u32,
    size:       i32,
    type_:      GLenum,
    normalized: bool,
    stride:     i32,
    pointer:    *const c_void) -> Result<(),EOpenGL>{

    unsafe{
        gl::VertexAttribPointer(index, size, type_, normalized as u8, stride, pointer);
    }

    check_for_error(

        "glVertexAttribPointer",
        vec![
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                    - The parameter 'index' is greater than or equal to GL_MAX_VERTEX_ATTRIBS\n
                    - The parameter 'size' is not 1, 2, 3, 4 or (for glVertexAttribPointer), \
                      GL_BGRA\n
                    - the parameter 'stride' is negative\n",
                GlErrTypes::InvalidValue,


            ),

            MsgMashingType::new(
                "The parameter 'type' is not an accepted value",
                GlErrTypes::InvalidEnum

            ),

            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                     -  The parameter 'size' is GL_BGRA and type is not GL_UNSIGNED_BYTE, \
                        GL_INT_2_10_10_10_REV or GL_UNSIGNED_INT_2_10_10_10_REV\n
                     -  The parameter 'type' is GL_INT_2_10_10_10_REV or \
                        GL_UNSIGNED_INT_2_10_10_10_REV and size is not 4 or GL_BGRA\n
                     -  The parameter 'type'  is GL_UNSIGNED_INT_10F_11F_11F_REV and size is not 3\n
                     -  The parameter 'size' is GL_BGRA and the parameter 'normalized' is GL_FALSE\n
                     -  zero is bound to the GL_ARRAY_BUFFER buffer object binding point and the \
                        pointer argument is not NULL\n",

                GlErrTypes::InvalidOperation
            )
        ],

    )?;

    Ok(())
}
//
//
///  Enable a generic vertex.rs attribute array
pub(crate) fn enable_vertex_attrib_array(index:u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::EnableVertexAttribArray(index);
    }

    check_for_error(

        "glEnableVertexAttribArray",
        vec![
            MsgMashingType::new(
                "no vertex.rs array object is bound",
                GlErrTypes::InvalidOperation,

            ),
            MsgMashingType::new(
                "The parameter 'index' is greater than or equal to GL_MAX_VERTEX_ATTRIBS",
                GlErrTypes::InvalidValue,

            )
        ]
    )?;

    Ok(())
}
//
//
///  Disable a generic vertex.rs attribute array
pub(crate) fn disable_vertex_attrib_array(index:u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::DisableVertexAttribArray(index);
    }

    check_for_error(

        "glDisableVertexAttribArray",
        vec![
            MsgMashingType::new(
                "no vertex.rs array object is bound",
                GlErrTypes::InvalidOperation,

            ),
            MsgMashingType::new(
                "The parameter 'index' is greater than or equal to GL_MAX_VERTEX_ATTRIBS",
                GlErrTypes::InvalidValue
            )

        ],

    )?;

    Ok(())
}
//
//
// ------------------------------------------------------------------------------------------------
// TEXTURE
//
//
/// generate texture names
pub(crate) fn gen_textures(nb:i32,array:&mut u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::GenTextures(nb,array);
    }

    check_for_error(
        "glGenTextures",
        vec![
            MsgMashingType::new(
                "The parameter 'n' is negative",
                GlErrTypes::InvalidValue,
            )
        ],
    )?;

    Ok(())
}
//
//
/// bind a named texture to a texturing target
pub(crate) fn bind_textures(target: GLenum,texture:u32) -> Result<(),EOpenGL>{

    unsafe{

        gl::BindTexture(target, texture);

    }

    check_for_error(
        "glBindTexture",
        vec![
            MsgMashingType::new(
                "the parameter 'target' is not one of the allowable values",
                GlErrTypes::InvalidEnum
            ),

            MsgMashingType::new(
                "The parameter 'texture' is not a name returned from a previous call \
                    to glGenTextures",
                GlErrTypes::InvalidValue
            ),

            MsgMashingType::new(
                "The parameter 'texture'  was previously created with a target that doesn't \
                 match that of the parameter 'target'",
                GlErrTypes::InvalidOperation
            )

        ],

    )?;

    Ok(())
}
//
//
/// set texture parameters
pub(crate) fn tex_parameter_i(target: GLenum,pname:GLenum,parameter:GLenum) -> Result<(),EOpenGL> {

    unsafe{

        gl::TexParameteri(target, pname, parameter as i32);

    }

    check_for_error(
        "glTexParameteri",

        vec![
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                     -  The parameter 'pname' is not one of the accepted defined values\n
                     -  called for a non-scalar parameter (pname GL_TEXTURE_BORDER_COLOR or \
                        GL_TEXTURE_SWIZZLE_RGBA\n
                     -  The parameter 'target' is not one of the accepted defined values\n
                     -  The effective target is either GL_TEXTURE_2D_MULTISAMPLE or \
                        GL_TEXTURE_2D_MULTISAMPLE_ARRAY, and the parameter 'pname' \
                        is any of the sampler states\n
                     -  the effective target is GL_TEXTURE_RECTANGLE and either of the parameter \
                        'pnames' GL_TEXTURE_WRAP_S or GL_TEXTURE_WRAP_T is set to either \
                        GL_MIRROR_CLAMP_TO_EDGE, GL_MIRRORED_REPEAT or GL_REPEAT\n
                     -  he effective target is GL_TEXTURE_RECTANGLE and the parameter 'pname' \
                        GL_TEXTURE_MIN_FILTER is set to a value other than GL_NEAREST or GL_LINEAR \
                        (no mipmap filtering is permitted)",
                GlErrTypes::InvalidEnum,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                    -   The effective target is either GL_TEXTURE_2D_MULTISAMPLE or \
                        GL_TEXTURE_2D_MULTISAMPLE_ARRAY, and the parameter 'pname' \
                        GL_TEXTURE_BASE_LEVEL is set to a value other than zero\n
                    -   The texture is not the name of an existing texture object.\n
                    -   The effective target is GL_TEXTURE_RECTANGLE and the parameter 'pname' \
                        GL_TEXTURE_BASE_LEVEL is set to any value other than zero\n
                    ",
                GlErrTypes::InvalidOperation,

            ),
            MsgMashingType::new(
                "The parameter 'pname' is GL_TEXTURE_BASE_LEVEL or GL_TEXTURE_MAX_LEVEL, \
                      and param or params is negative",
                GlErrTypes::InvalidValue,

            )
        ],

    )?;

    Ok(())
}
//
//
/// specify a two-dimensional texture image
pub(crate) fn tex_image_2d(
    target: GLenum,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: GLenum,
    type_: GLenum,
    data: *const c_void) -> Result<(),EOpenGL> {


    unsafe{

        gl::TexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            data as *const _);
    }

    check_for_error(
        "glTexImage2D",
        vec![
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                     -  The parameter 'target' is not GL_TEXTURE_2D, GL_TEXTURE_1D_ARRAY, \
                        GL_TEXTURE_RECTANGLE, GL_PROXY_TEXTURE_2D, GL_PROXY_TEXTURE_1D_ARRAY, \
                        GL_PROXY_TEXTURE_RECTANGLE, GL_PROXY_TEXTURE_CUBE_MAP, \
                        GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_NEGATIVE_X, \
                        GL_TEXTURE_CUBE_MAP_POSITIVE_Y, GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, \
                        GL_TEXTURE_CUBE_MAP_POSITIVE_Z, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z\n
                     -  The parameter 'target' is one of the six capsule map 2D image targets and \
                        the width and height parameters are not equal\n
                     -  the parameter 'type' is not a type constant\n
                     -  the parameter 'width' is less than 0 or greater than GL_MAX_TEXTURE_SIZE\n",
                GlErrTypes::InvalidEnum,
            ),
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                     -  The parameter 'target' is not GL_TEXTURE_1D_ARRAY or \
                        GL_PROXY_TEXTURE_1D_ARRAY and the parameter 'height' is less than 0 \
                        or greater than GL_MAX_TEXTURE_SIZE\n
                     -  The parameter 'target' is GL_TEXTURE_1D_ARRAY or GL_PROXY_TEXTURE_1D_ARRAY \
                        and height is less than 0 or greater than GL_MAX_ARRAY_TEXTURE_LAYERS\n
                     -  The parameter 'level' is less than 0\n
                     -  The parameter 'level' is greater than log2(max), where max is the \
                        returned value of GL_MAX_TEXTURE_SIZE\n
                     -  The parameter 'internalformat'  is not one of the accepted resolution \
                        and format symbolic constants\n
                     -  The parameter 'width' or 'height' s less than 0 or greater than \
                        GL_MAX_TEXTURE_SIZE.\n
                     -  The parameter 'border' is not 0\n
                     -  The parameter 'target'  is GL_TEXTURE_RECTANGLE or \
                        GL_PROXY_TEXTURE_RECTANGLE and the parameter 'level' is not 0\n",
                GlErrTypes::InvalidValue,

            ),
            MsgMashingType::new(
                "possible cause of error:\n
                    \n
                     -  The parameter 'type' is one of GL_UNSIGNED_BYTE_3_3_2, \
                        GL_UNSIGNED_BYTE_2_3_3_REV, GL_UNSIGNED_SHORT_5_6_5, \
                        GL_UNSIGNED_SHORT_5_6_5_REV, or GL_UNSIGNED_INT_10F_11F_11F_REV, \
                        and format is not GL_RGB\n
                     -  The parameter 'type'  is one of GL_UNSIGNED_SHORT_4_4_4_4, \
                        GL_UNSIGNED_SHORT_4_4_4_4_REV, GL_UNSIGNED_SHORT_5_5_5_1, \
                        GL_UNSIGNED_SHORT_1_5_5_5_REV, GL_UNSIGNED_INT_8_8_8_8, \
                        GL_UNSIGNED_INT_8_8_8_8_REV, GL_UNSIGNED_INT_10_10_10_2, \
                        GL_UNSIGNED_INT_2_10_10_10_REV, or GL_UNSIGNED_INT_5_9_9_9_REV, \
                        and format is neither GL_RGBA nor GL_BGRA.\n
                     -  The parameter 'target' s not GL_TEXTURE_2D, GL_PROXY_TEXTURE_2D, \
                        GL_TEXTURE_RECTANGLE, or GL_PROXY_TEXTURE_RECTANGLE, \
                        and the parameter 'internalformat' is GL_DEPTH_COMPONENT, \
                        GL_DEPTH_COMPONENT16, GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32F\n
                     -  The parameter 'format' is GL_DEPTH_COMPONENT and the parameter \
                        'internalformat' is not GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT16,\
                         GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32F\n
                     -  The parameter 'internalformat' is GL_DEPTH_COMPONENT, \
                        GL_DEPTH_COMPONENT16,GL_DEPTH_COMPONENT24, or GL_DEPTH_COMPONENT32F, \
                         and format is not GL_DEPTH_COMPONENT\n
                     -  non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target \
                        and the buffer object's data store is currently mapped\n
                     -  non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target \
                        and the data would be unpacked from the buffer object such that the memory \
                        reads required would exceed the data store size\n
                     -  non-zero buffer object name is bound to the GL_PIXEL_UNPACK_BUFFER target \
                        and data is not evenly divisible into the number of bytes needed to store \
                        in memory a datum indicated by type\n",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(())
}
//
//
/// generate mipmaps for a specified texture object
pub(crate) fn gen_mipmap(target: GLenum) -> Result<(),EOpenGL>{

    unsafe{
        gl::GenerateMipmap(target);
    }

    check_for_error(
        "glGenerateMipmap",
        vec![
            MsgMashingType::new(
                "The parameter `target` is not one of the accepted texture targets",
                GlErrTypes::InvalidEnum,

            ),
            MsgMashingType::new(
                "The parameter is GL_TEXTURE_CUBE_MAP or GL_TEXTURE_CUBE_MAP_ARRAY, and the \
                     specified texture object is not capsule complete or capsule array complete,
                     respectively",
                GlErrTypes::InvalidOperation,

            )
        ],

    )?;

    Ok(())
}
//
//
/// select active texture unit
pub(crate) fn active_texture(texture: u32) -> Result<(),EOpenGL>{

    unsafe{
        gl::ActiveTexture(texture);
    }

    check_for_error(
        "glActiveTexture",
        vec![
            MsgMashingType::new(
                "the parameter 'texture'  is not one of GL_TEXTUREi, where i ranges from zero \
                    to the value of GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS minus one",
                GlErrTypes::InvalidEnum
            )
        ],

    )?;
    Ok(())
}
//
//
// ------------------------------------------------------------------------------------------------
// OTHER
//
//
/// specify clear values for the color buffers
pub(crate) fn clear_color(red:f32, green:f32, blue:f32, alpha:f32) -> Result<(),EOpenGL>{

    unsafe{
        gl::ClearColor(red, green, blue, alpha);
    }

    Ok(())
}
//
//
/// clear buffers to preset values
pub(crate) fn clear(mask:GLbitfield) -> Result<(),EOpenGL>{

    unsafe{
        gl::Clear(mask);
    }

    check_for_error(
        "glClear",
        vec![
            MsgMashingType::new(
                "any bit other than the three defined bits is set in the parameter 'mask'",
                GlErrTypes::InvalidValue,
            )
        ],

    )?;

    Ok(())
}
//
//
/// render primitives from array data
pub(crate) fn draw_elements(
    target: GLenum,
    count:i32,
    _type:GLenum,
    indices:*const c_void) -> Result<(),EOpenGL>{

    unsafe{
        gl::DrawElements(target,count,_type ,indices);
    }

    check_for_error(
        "glDrawElements",
        vec![
            MsgMashingType::new(
                "The parameter 'mode' is not an accepted value",
                GlErrTypes::InvalidEnum,
            ),
            MsgMashingType::new(
                "The parameter 'count' is negative",
                GlErrTypes::InvalidValue,
            ),
            MsgMashingType::new(
                "possible cause of error:\n
                     -  Geometry shader is active and the parameter 'mode' is incompatible with \
                        the input primitive type of the geometry shader in the currently installed \
                        program object\n
                    -   non-zero buffer object name is bound to an enabled array or the element \
                        array and the buffer object's data store is currently mapped\n",
                GlErrTypes::InvalidOperation,
            )
        ],

    )?;

    Ok(())
}
//
//
/// render primitives from array data
pub(crate) fn draw_arrays(mode:GLenum,first:i32,count:i32) -> Result<(),EOpenGL>{

    unsafe {
        gl::DrawArrays(mode,first,count);
    }

    check_for_error(
        "glDrawArrays",
        vec![
            MsgMashingType::new(
                "The parameter 'mode' is not an accepted value",
                GlErrTypes::InvalidEnum,
            ),
            MsgMashingType::new(
                "the parameter 'count' is negative",
                GlErrTypes::InvalidValue
            ),
            MsgMashingType::new(

                "possible cause of error:\n\
                -   a non-zero buffer object name is bound to an enabled array and the buffer \
                    object's data store is currently mapped\n
                -   a geometry shader is active and mode is incompatible with the input primitive \
                    type of the geometry shader in the currently installed program object\n
                ",
                GlErrTypes::InvalidOperation
            )
        ]
    )?;

    Ok(())


}
//
//
/// set the viewport
pub(crate) fn view_port(x:i32, y:i32, width:i32, height:i32) -> Result<(),EOpenGL>{

    unsafe{
        gl::Viewport(x, y, width, height);
    }

    check_for_error(
        "glViewport",
        vec![
            MsgMashingType::new(
                "Either the parameter 'width' or 'height' is negative",
                GlErrTypes::InvalidValue,

            )
        ],
    )?;

    Ok(())
}
//
//
/// enable server-side GL capabilities
pub(crate) fn enable( capability:GLenum) -> Result<(),EOpenGL>{

    unsafe{
        gl::Enable(capability);
    }

    check_for_error(
        "glEnable",
        vec![
            MsgMashingType::new(
                "The parameter 'cap' is not value supported\n see list of supported values at:\
                     https://registry.khronos.org/OpenGL-Refpages/gl4/",
                GlErrTypes::InvalidEnum,

            ),
        ],

    )?;

    Ok(())
}
//
//
