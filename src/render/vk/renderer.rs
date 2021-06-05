use erupt::vk;

pub struct Renderer {
    instance: std::rc::Rc<super::Instance>,
    command_pool: vk::CommandPool,
    syncs: Vec<super::SyncObject>,
    current_frame: usize,
}

impl Renderer {
    const MAX_FRAMES_IN_FLIGHT: usize = 2;

    pub fn new(surface: &super::Surface) -> Self {
        let instance = surface.instance();
        let device = instance.device();
        let queue = instance.graphics();

        // Create Command Pool

        let command_pool_create_info = vk::CommandPoolCreateInfoBuilder::new()
            .queue_family_index(queue.family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let command_pool =
            unsafe { device.create_command_pool(&command_pool_create_info, None, None) }.unwrap();

        // Create Sync Objects

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .command_buffer_count(Self::MAX_FRAMES_IN_FLIGHT as u32);
        let command_buffers =
            unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }.unwrap();
        let semaphore_create_info = vk::SemaphoreCreateInfoBuilder::new();
        let fence_create_info =
            vk::FenceCreateInfoBuilder::new().flags(vk::FenceCreateFlags::SIGNALED);
        let syncs = command_buffers
            .into_iter()
            .map(|command_buffer| {
                let in_flight =
                    unsafe { device.create_fence(&fence_create_info, None, None) }.unwrap();
                let image_available =
                    unsafe { device.create_semaphore(&semaphore_create_info, None, None) }.unwrap();
                let render_finished =
                    unsafe { device.create_semaphore(&semaphore_create_info, None, None) }.unwrap();
                super::SyncObject {
                    in_flight,
                    image_available,
                    render_finished,
                    command_buffer,
                }
            })
            .collect();

        Self {
            instance,
            command_pool,
            syncs,
            current_frame: 0,
        }
    }
}


impl Drop for Renderer {
    fn drop(&mut self) {
        let device = self.instance.device();
        self.syncs.iter().for_each(|sync_object| unsafe {
            device.destroy_fence(Some(sync_object.in_flight), None);
            device.destroy_semaphore(Some(sync_object.image_available), None);
            device.destroy_semaphore(Some(sync_object.render_finished), None)
        });
        unsafe { device.destroy_command_pool(Some(self.command_pool), None) }
    }
}
