use erupt::vk;

pub struct VoxelRenderer {
    instance: std::rc::Rc<super::Instance>,
    layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    mesh: VoxelMesh,
}

impl VoxelRenderer {
    const VOXEL_FRAG: &'static [u32] = vk_shader_macros::include_glsl!("src/shaders/voxel.frag");
    const VOXEL_VERT: &'static [u32] = vk_shader_macros::include_glsl!("src/shaders/voxel.vert");

    pub fn new(surface: &super::Surface, faces: &[u32]) -> Self {
        let instance = surface.instance();
        let device = instance.device();
        let render_info = surface.render_info();

        let vert_shader_module_create_info =
            vk::ShaderModuleCreateInfoBuilder::new().code(Self::VOXEL_VERT);
        let vert_shader_module =
            unsafe { device.create_shader_module(&vert_shader_module_create_info, None, None) }
                .unwrap();

        let frag_shader_module_create_info =
            vk::ShaderModuleCreateInfoBuilder::new().code(Self::VOXEL_FRAG);
        let frag_shader_module =
            unsafe { device.create_shader_module(&frag_shader_module_create_info, None, None) }
                .unwrap();

        let entry_point = std::ffi::CString::new("main").unwrap();

        let stages = [
            vk::PipelineShaderStageCreateInfoBuilder::new()
                .stage(vk::ShaderStageFlagBits::VERTEX)
                .module(vert_shader_module)
                .name(&entry_point),
            vk::PipelineShaderStageCreateInfoBuilder::new()
                .stage(vk::ShaderStageFlagBits::FRAGMENT)
                .module(frag_shader_module)
                .name(&entry_point),
        ];

        let input_binding_descriptions = [vk::VertexInputBindingDescriptionBuilder::new()
            .binding(0)
            .stride(std::mem::size_of::<u32>() as u32)
            .input_rate(vk::VertexInputRate::INSTANCE)];

        let input_attribute_descriptions = [vk::VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .location(0)
            .format(vk::Format::R32_UINT)
            .offset(0)];

        let vertex_input = vk::PipelineVertexInputStateCreateInfoBuilder::new()
            .vertex_binding_descriptions(&input_binding_descriptions)
            .vertex_attribute_descriptions(&input_attribute_descriptions);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewports = [vk::ViewportBuilder::new()
            .x(0.0)
            .y(0.0)
            .width(render_info.extent.width as f32)
            .height(render_info.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)];
        let scissors = [vk::Rect2DBuilder::new()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(render_info.extent)];
        let viewport = vk::PipelineViewportStateCreateInfoBuilder::new()
            .viewports(&viewports)
            .scissors(&scissors);

        let depth_stencil = vk::PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS);

        let rasterization = vk::PipelineRasterizationStateCreateInfoBuilder::new()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);
        let multisample = vk::PipelineMultisampleStateCreateInfoBuilder::new()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlagBits::_1);

        let attachments = [vk::PipelineColorBlendAttachmentStateBuilder::new()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false)];

        let color_blend = vk::PipelineColorBlendStateCreateInfoBuilder::new()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let push_constant_ranges = [vk::PushConstantRangeBuilder::new()
            .offset(0)
            .size(64)
            .stage_flags(vk::ShaderStageFlags::VERTEX)];

        let layout_info =
            vk::PipelineLayoutCreateInfoBuilder::new().push_constant_ranges(&push_constant_ranges);

        let layout = unsafe { device.create_pipeline_layout(&layout_info, None, None) }.unwrap();

        let pipeline_create_info = vk::GraphicsPipelineCreateInfoBuilder::new()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport)
            .depth_stencil_state(&depth_stencil)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .layout(layout)
            .render_pass(render_info.render_pass)
            .subpass(0);

        let pipeline =
            unsafe { device.create_graphics_pipelines(None, &[pipeline_create_info], None) }
                .unwrap()[0];

        unsafe {
            device.destroy_shader_module(Some(vert_shader_module), None);
            device.destroy_shader_module(Some(frag_shader_module), None);
        }

        let mesh = VoxelMesh::from_faces(instance.clone(), &faces);

        Self {
            instance,
            layout,
            pipeline,
            mesh,
        }
    }

    pub fn draw(&mut self, command_buffer: vk::CommandBuffer, matrix: &ultraviolet::Mat4) {
        let device = self.instance.device();
        unsafe {
            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            );
            device.cmd_push_constants(
                command_buffer,
                self.layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                64,
                matrix.as_ptr() as *const std::ffi::c_void,
            );

            device.cmd_bind_vertex_buffers(command_buffer, 0, &[self.mesh.vertex_buffer], &[0]);
            device.cmd_draw(command_buffer, 6, self.mesh.length, 0, 0)
        }
    }

    pub fn rebuild(&mut self, surface: &super::Surface, faces: &[u32]) {
        unsafe {
            std::mem::drop(std::ptr::read(self));
            std::ptr::write(self, Self::new(surface, faces))
        }
    }
}

impl Drop for VoxelRenderer {
    fn drop(&mut self) {
        let device = self.instance.device();
        unsafe {
            device.free_memory(Some(self.mesh.vertex_buffer_memory), None);
            device.destroy_buffer(Some(self.mesh.vertex_buffer), None);
            device.destroy_pipeline_layout(Some(self.layout), None);
            device.destroy_pipeline(Some(self.pipeline), None)
        }
    }
}

pub struct VoxelMesh {
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    length: u32,
}

impl VoxelMesh {
    pub fn from_faces(instance: std::rc::Rc<super::Instance>, faces: &[u32]) -> Self {
        let device = instance.device();

        let buffer_info = vk::BufferCreateInfoBuilder::new()
            .size((std::mem::size_of::<u32>() * faces.len()) as u64)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let vertex_buffer = unsafe { device.create_buffer(&buffer_info, None, None) }.unwrap();

        let requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer, None) };

        let memory_type_index = instance.get_memory_type_index(
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            requirements,
        );

        let memory_info = vk::MemoryAllocateInfoBuilder::new()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);

        let vertex_buffer_memory =
            unsafe { device.allocate_memory(&memory_info, None, None) }.unwrap();

        unsafe { device.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0) }.unwrap();

        let mut memory: *mut std::ffi::c_void = std::ptr::null_mut::<std::ffi::c_void>();

        unsafe { device.map_memory(vertex_buffer_memory, 0, buffer_info.size, None, &mut memory) }
            .unwrap();

        unsafe { std::ptr::copy_nonoverlapping(faces.as_ptr(), memory.cast(), faces.len()) };

        unsafe { device.unmap_memory(vertex_buffer_memory) };

        Self {
            vertex_buffer,
            vertex_buffer_memory,
            length: faces.len() as u32,
        }
    }
}
