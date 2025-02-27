//! Object representation for Context3D objects

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2_stub_method;
use crate::context::RenderContext;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_render::backend::{
    BufferUsage, Context3D, Context3DBlendFactor, Context3DCommand, Context3DCompareMode,
    Context3DTextureFormat, Context3DTriangleFace, Context3DVertexBufferFormat, ProgramType,
    Texture,
};
use ruffle_render::bitmap::BitmapHandle;
use ruffle_render::commands::CommandHandler;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

use super::program_3d_object::Program3DObject;
use super::texture_object::TextureObject;
use super::{ClassObject, IndexBuffer3DObject, VertexBuffer3DObject};

#[derive(Clone, Collect, Copy)]
#[collect(no_drop)]
pub struct Context3DObject<'gc>(GcCell<'gc, Context3DData<'gc>>);

impl<'gc> Context3DObject<'gc> {
    pub fn from_context(
        activation: &mut Activation<'_, 'gc>,
        context: Box<dyn Context3D>,
    ) -> Result<Object<'gc>, Error<'gc>> {
        let class = activation.avm2().classes().context3d;
        let base = ScriptObjectData::new(class);

        let mut this: Object<'gc> = Context3DObject(GcCell::allocate(
            activation.context.gc_context,
            Context3DData {
                base,
                render_context: Some(context),
                commands: vec![],
            },
        ))
        .into();
        this.install_instance_slots(activation);

        class.call_native_init(Some(this), &[], activation)?;

        Ok(this)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn configure_back_buffer(
        &mut self,
        activation: &mut Activation<'_, 'gc>,
        width: u32,
        height: u32,
        anti_alias: u32,
        depth_and_stencil: bool,
        wants_best_resolution: bool,
        wants_best_resolution_on_browser_zoom: bool,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::ConfigureBackBuffer {
                width,
                height,
                anti_alias,
                depth_and_stencil,
                wants_best_resolution,
                wants_best_resolution_on_browser_zoom,
            },
        );
    }

    pub fn create_index_buffer(
        &self,
        num_indices: u32,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        let index_buffer = self
            .0
            .write(activation.context.gc_context)
            .render_context
            .as_mut()
            .unwrap()
            .create_index_buffer(BufferUsage::StaticDraw, num_indices);

        Ok(Value::Object(IndexBuffer3DObject::from_handle(
            activation,
            *self,
            index_buffer,
        )?))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_texture(
        &self,
        width: u32,
        height: u32,
        format: Context3DTextureFormat,
        optimize_for_render_to_texture: bool,
        streaming_levels: u32,
        class: ClassObject<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        check_texture_stub(activation, format);
        let texture = self
            .0
            .write(activation.context.gc_context)
            .render_context
            .as_mut()
            .unwrap()
            .create_texture(
                width,
                height,
                format,
                optimize_for_render_to_texture,
                streaming_levels,
            )?;

        Ok(Value::Object(TextureObject::from_handle(
            activation, *self, texture, class,
        )?))
    }

    pub fn create_vertex_buffer(
        &self,
        num_vertices: u32,
        data_32_per_vertex: u8,
        usage: BufferUsage,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        let handle = self
            .0
            .write(activation.context.gc_context)
            .render_context
            .as_mut()
            .unwrap()
            .create_vertex_buffer(usage, num_vertices, data_32_per_vertex);
        Ok(Value::Object(VertexBuffer3DObject::from_handle(
            activation,
            *self,
            handle,
            data_32_per_vertex,
        )?))
    }

    pub fn upload_vertex_buffer_data(
        &self,
        buffer: VertexBuffer3DObject<'gc>,
        data: Vec<u8>,
        start_vertex: usize,
        data32_per_vertex: u8,
        activation: &mut Activation<'_, 'gc>,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::UploadToVertexBuffer {
                buffer: buffer.handle(),
                data,
                start_vertex,
                data32_per_vertex,
            },
        );
    }

    pub fn upload_index_buffer_data(
        &self,
        buffer: IndexBuffer3DObject<'gc>,
        data: Vec<u8>,
        start_offset: usize,
        activation: &mut Activation<'_, 'gc>,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::UploadToIndexBuffer {
                buffer: buffer.handle(),
                data,
                start_offset,
            },
        );
    }

    pub fn set_vertex_buffer_at(
        &self,
        index: u32,
        buffer: Option<VertexBuffer3DObject<'gc>>,
        buffer_offset: u32,
        buffer_format: Context3DVertexBufferFormat,
        activation: &mut Activation<'_, 'gc>,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::SetVertexBufferAt {
                index,
                buffer: buffer.map(|b| b.handle()),
                buffer_offset,
                format: buffer_format,
            },
        );
    }

    pub fn create_program(
        &self,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Program3DObject::from_context(
            activation, *self,
        )?))
    }

    pub fn upload_shaders(
        &self,
        activation: &mut Activation<'_, 'gc>,
        program: Program3DObject<'gc>,
        vertex_shader_agal: Vec<u8>,
        fragment_shader_agal: Vec<u8>,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::UploadShaders {
                vertex_shader: program.vertex_shader_handle(),
                vertex_shader_agal,
                fragment_shader: program.fragment_shader_handle(),
                fragment_shader_agal,
            },
        );
    }

    pub fn set_program(&self, activation: &mut Activation<'_, 'gc>, program: Program3DObject<'gc>) {
        self.0
            .write(activation.context.gc_context)
            .commands
            .push(Context3DCommand::SetShaders {
                vertex_shader: program.vertex_shader_handle(),
                fragment_shader: program.fragment_shader_handle(),
            });
    }

    pub fn draw_triangles(
        &self,
        activation: &mut Activation<'_, 'gc>,
        index_buffer: IndexBuffer3DObject<'gc>,
        first_index: u32,
        mut num_triangles: i32,
    ) {
        if num_triangles == -1 {
            // FIXME - should we error if the number of indices isn't a multiple of 3?
            num_triangles = (index_buffer.count() / 3) as i32;
        }

        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::DrawTriangles {
                index_buffer: index_buffer.handle(),
                first_index: first_index as usize,
                num_triangles: num_triangles as isize,
            },
        );
    }

    pub fn set_program_constants_from_matrix(
        &self,
        activation: &mut Activation<'_, 'gc>,
        program_type: ProgramType,
        first_register: u32,
        matrix_raw_data_column_major: Vec<f32>,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::SetProgramConstantsFromVector {
                program_type,
                first_register,
                matrix_raw_data_column_major,
            },
        );
    }

    pub fn set_culling(&self, activation: &mut Activation<'_, 'gc>, face: Context3DTriangleFace) {
        self.0
            .write(activation.context.gc_context)
            .commands
            .push(Context3DCommand::SetCulling { face });
    }

    pub fn set_blend_factors(
        &self,
        activation: &mut Activation<'_, 'gc>,
        source_factor: Context3DBlendFactor,
        destination_factor: Context3DBlendFactor,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::SetBlendFactors {
                source_factor,
                destination_factor,
            },
        );
    }

    pub fn set_render_to_texture(
        &self,
        activation: &mut Activation<'_, 'gc>,
        texture: Rc<dyn Texture>,
        enable_depth_and_stencil: bool,
        anti_alias: u32,
        surface_selector: u32,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::SetRenderToTexture {
                texture,
                enable_depth_and_stencil,
                anti_alias,
                surface_selector,
            },
        );
    }

    pub fn set_render_to_back_buffer(&self, activation: &mut Activation<'_, 'gc>) {
        self.0
            .write(activation.context.gc_context)
            .commands
            .push(Context3DCommand::SetRenderToBackBuffer);
    }

    pub fn present(&self, activation: &mut Activation<'_, 'gc>) -> Result<(), Error<'gc>> {
        let mut write = self.0.write(activation.context.gc_context);
        let commands = std::mem::take(&mut write.commands);

        let context: &mut dyn Context3D = write.render_context.as_deref_mut().unwrap();

        activation.context.renderer.context3d_present(
            context,
            commands,
            activation.context.gc_context,
        )?;
        Ok(())
    }

    // Renders our finalized frame to the screen, as part of the Ruffle rendering process.
    pub fn render(&self, context: &mut RenderContext<'_, 'gc>) {
        let context3d = self.0.read();
        let context3d = context3d.render_context.as_ref().unwrap();

        if context3d.should_render() {
            let handle = context3d.bitmap_handle();

            context.commands.render_stage3d(
                handle,
                // FIXME - apply x and y translation from Stage3D
                context.transform_stack.transform(),
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_clear(
        &self,
        activation: &mut Activation<'_, 'gc>,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
        depth: f64,
        stencil: u32,
        mask: u32,
    ) {
        self.0
            .write(activation.context.gc_context)
            .commands
            .push(Context3DCommand::Clear {
                red,
                green,
                blue,
                alpha,
                depth,
                stencil,
                mask,
            });
    }

    pub(crate) fn copy_bitmap_to_texture(
        &self,
        source: BitmapHandle,
        dest: Rc<dyn Texture>,
        layer: u32,
        activation: &mut Activation<'_, 'gc>,
    ) {
        self.0.write(activation.context.gc_context).commands.push(
            Context3DCommand::CopyBitmapToTexture {
                source,
                dest,
                layer,
            },
        )
    }

    pub(crate) fn set_texture_at(
        &self,
        activation: &mut Activation<'_, 'gc>,
        sampler: u32,
        texture: Option<Rc<dyn Texture>>,
        cube: bool,
    ) {
        self.0
            .write(activation.context.gc_context)
            .commands
            .push(Context3DCommand::SetTextureAt {
                sampler,
                texture,
                cube,
            })
    }

    pub(crate) fn set_depth_test(
        &self,
        activation: &mut Activation<'_, 'gc>,
        depth_mask: bool,
        pass_compare_mode: Context3DCompareMode,
    ) {
        self.0
            .write(activation.context.gc_context)
            .commands
            .push(Context3DCommand::SetDepthTest {
                depth_mask,
                pass_compare_mode,
            })
    }

    pub(crate) fn create_cube_texture(
        &self,
        size: u32,
        format: Context3DTextureFormat,
        optimize_for_render_to_texture: bool,
        streaming_levels: u32,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        check_texture_stub(activation, format);
        let texture = self
            .0
            .write(activation.context.gc_context)
            .render_context
            .as_mut()
            .unwrap()
            .create_cube_texture(
                size,
                format,
                optimize_for_render_to_texture,
                streaming_levels,
            )?;

        let class = activation.avm2().classes().cubetexture;

        Ok(Value::Object(TextureObject::from_handle(
            activation, *self, texture, class,
        )?))
    }
}

#[derive(Collect)]
#[collect(no_drop)]
pub struct Context3DData<'gc> {
    /// Base script object
    base: ScriptObjectData<'gc>,

    #[collect(require_static)]
    render_context: Option<Box<dyn Context3D>>,

    commands: Vec<Context3DCommand<'gc>>,
}

impl<'gc> TObject<'gc> for Context3DObject<'gc> {
    fn base(&self) -> Ref<ScriptObjectData<'gc>> {
        Ref::map(self.0.read(), |read| &read.base)
    }

    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc>> {
        RefMut::map(self.0.write(mc), |write| &mut write.base)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_context_3d(&self) -> Option<Context3DObject<'gc>> {
        Some(*self)
    }
}

impl std::fmt::Debug for Context3DObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context3D")
    }
}

// This would ideally be placed closer to the actual usage, but
// we don't have stub support in 'render' crates
fn check_texture_stub(activation: &mut Activation<'_, '_>, format: Context3DTextureFormat) {
    match format {
        Context3DTextureFormat::BgrPacked => {
            avm2_stub_method!(
                activation,
                "flash.display3D.Context3D",
                "createTexture",
                "with BgrPacked"
            );
        }
        Context3DTextureFormat::Compressed => {
            avm2_stub_method!(
                activation,
                "flash.display3D.Context3D",
                "createTexture",
                "with Compressed"
            );
        }
        _ => {}
    }
}
