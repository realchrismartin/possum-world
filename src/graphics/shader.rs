use web_sys::{WebGl2RenderingContext,WebGlShader,WebGlProgram};

pub struct Shader
{
    shader : WebGlProgram
}

impl Shader
{
    pub fn new(context: &WebGl2RenderingContext, vertex_shader_content : &str, frag_shader_content: &str) -> Result<Self,String> 
    {
        let vertex_shader = match Self::compile(context,WebGl2RenderingContext::VERTEX_SHADER,vertex_shader_content)
        {
            Ok(vertex_shader) => vertex_shader,
            Err(error) => return Err(error)
        };

        let frag_shader = match Self::compile(context,WebGl2RenderingContext::FRAGMENT_SHADER,frag_shader_content)
        {
            Ok(frag_shader) => frag_shader,
            Err(error) => return Err(error)
        };

        let shader_program = match Self::link(context,&vertex_shader,&frag_shader)
        {
            Ok(shader_program) => shader_program,
            Err(error) => return Err(error)
        };

        Ok(Self
        {
            shader: shader_program
        })
    }

    fn compile(context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str) -> Result<WebGlShader, String>
    {
        let shader_object = context.create_shader(shader_type)
        .ok_or_else(|| String::from("Failed to create shader"))?;

        context.shader_source(&shader_object, source);
        context.compile_shader(&shader_object);

        if context.get_shader_parameter(&shader_object, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false)
        {
            Ok(shader_object)
        } else 
        {
            Err(context
                .get_shader_info_log(&shader_object)
                .unwrap_or_else(|| String::from("Failed to compile shader")))
        }
    }

    fn link(context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    frag_shader: &WebGlShader) -> Result<WebGlProgram,String>
    {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Failed to create shader program"))?;

        context.attach_shader(&program,vertex_shader);
        context.attach_shader(&program,frag_shader);
        context.link_program(&program);
        
        if context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false)
        {
            Ok(program)
        } else 
        {
            Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Failed to link shader program")))
        }
    }
}