use erupt::{vk, ExtendableFromConst};

pub struct Instance {
    messenger: vk::DebugUtilsMessengerEXT,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    graphics: super::QueueInfo,
    present: super::QueueInfo,

    // Ordered based on Rust's Struct Drop Order (Device, then Instance, then Entry)
    device: erupt::DeviceLoader,
    instance: erupt::InstanceLoader,
    #[allow(dead_code)]
    entry: erupt::DefaultEntryLoader,
}

impl Instance {
    pub fn new(window: &crate::window::Window) -> std::rc::Rc<Self> {
        let entry = erupt::EntryLoader::new().unwrap();
        let mut instance_extensions =
            erupt::utils::surface::enumerate_required_extensions(window.window()).unwrap();
        let mut instance_layers = Vec::new();
        let device_extensions = vec![vk::KHR_SWAPCHAIN_EXTENSION_NAME];
        let mut device_layers = Vec::new();

        // Create Instance & Debug Messenger

        let (instance, messenger) = if crate::DEBUG_MODE {
            let val = erupt::cstr!("VK_LAYER_KHRONOS_validation");
            instance_extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION_NAME);
            instance_layers.push(val);
            device_layers.push(val);
            let mut messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXTBuilder::new()
                .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
                .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                .pfn_user_callback(Some(super::debug::callback));
            let instance_create_info = vk::InstanceCreateInfoBuilder::new()
                .enabled_extension_names(&instance_extensions)
                .enabled_layer_names(&instance_layers)
                .extend_from(&mut messenger_create_info);
            let instance = unsafe {
                erupt::InstanceLoader::new(&entry, &instance_create_info, None)
            }.unwrap();
            let messenger = unsafe {
                instance.create_debug_utils_messenger_ext(&messenger_create_info, None)
            }
            .unwrap();
            (instance, messenger)
        } else {
            let instance_create_info = vk::InstanceCreateInfoBuilder::new()
                .enabled_extension_names(&instance_extensions)
                .enabled_layer_names(&instance_layers);
            let instance = unsafe {
                erupt::InstanceLoader::new(&entry, &instance_create_info, None)
            }.unwrap();
            (instance, vk::DebugUtilsMessengerEXT::null())
        };

        // Create Surface

        let surface =
            unsafe { erupt::utils::surface::create_surface(&instance, window.window(), None) }.unwrap();

        // Find Physical Device & Queue Families

        let physical_devices = unsafe { instance.enumerate_physical_devices(None) }.unwrap();
        let (physical_device, _, graphics_family, present_family) = physical_devices
            .into_iter()
            .filter_map(|physical_device| {
                let properties =
                    unsafe { instance.get_physical_device_properties(physical_device) };
                let queue_families = unsafe {
                    instance.get_physical_device_queue_family_properties(physical_device, None)
                };

                let present_family = match (0..queue_families.len()).find(|&index| {
                    unsafe {
                        instance.get_physical_device_surface_support_khr(
                            physical_device,
                            index as u32,
                            surface,
                        )
                    }
                    .unwrap()
                }) {
                    Some(index) => index as u32,
                    None => return None,
                };
                let graphics_family = match queue_families
                    .into_iter()
                    .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                {
                    Some(index) => index as u32,
                    None => return None,
                };
                Some((physical_device, properties, graphics_family, present_family))
            })
            .min_by_key(|(_, properties, ..)| match properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => 0,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                _ => 2,
            })
            .unwrap();

        // Get Physical Device Memory Properties

        let memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };

        // Create Logical Device & Queues

        let mut unique_queues = std::collections::HashSet::new();
        unique_queues.insert(graphics_family);
        unique_queues.insert(present_family);
        let queue_create_infos = unique_queues
            .into_iter()
            .map(|family| {
                vk::DeviceQueueCreateInfoBuilder::new()
                    .queue_family_index(family)
                    .queue_priorities(&[1.0])
            })
            .collect::<Vec<_>>();
        let device_create_info = vk::DeviceCreateInfoBuilder::new()
            .enabled_extension_names(&device_extensions)
            .enabled_layer_names(&device_layers)
            .queue_create_infos(&queue_create_infos);
        let device = unsafe {
            erupt::DeviceLoader::new(&instance, physical_device, &device_create_info, None)
        }.unwrap();
        let graphics_queue = unsafe { device.get_device_queue(graphics_family, 0) };
        let present_queue = unsafe { device.get_device_queue(present_family, 0) };

        let ret = Self {
            entry,
            instance,
            device,

            messenger,
            surface,
            physical_device,
            memory_properties,
            graphics: super::QueueInfo {
                family: graphics_family,
                queue: graphics_queue,
            },
            present: super::QueueInfo {
                family: present_family,
                queue: present_queue,
            },
        };
        std::rc::Rc::new(ret)
    }

    pub fn device(&self) -> &erupt::DeviceLoader {
        &self.device
    }
    pub(super) fn graphics(&self) -> super::QueueInfo {
        self.graphics
    }
    pub(super) fn present(&self) -> super::QueueInfo {
        self.present
    }

    pub(super) fn surface_info(&self, (width, height): (u32, u32)) -> super::SurfaceInfo {
        let surface_caps = unsafe {
            self.instance.get_physical_device_surface_capabilities_khr(
                self.physical_device,
                self.surface,
            )
        }
        .unwrap();

        let surface_formats = unsafe {
            self.instance.get_physical_device_surface_formats_khr(
                self.physical_device,
                self.surface,
                None,
            )
        }
        .unwrap();
        let surface_format = surface_formats
            .iter()
            .cloned()
            .find(|surface_format| {
                surface_format.format == vk::Format::B8G8R8A8_SRGB
                    && surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR_KHR
            })
            .unwrap_or(surface_formats[0]);

        let present_modes = unsafe {
            self.instance.get_physical_device_surface_present_modes_khr(
                self.physical_device,
                self.surface,
                None,
            )
        }
        .unwrap();
        let present_mode = present_modes
            .into_iter()
            .find(|&present_mode| present_mode == vk::PresentModeKHR::MAILBOX_KHR)
            .unwrap_or(vk::PresentModeKHR::FIFO_KHR);

        let extent = vk::Extent2D {
            width: width.clamp(
                surface_caps.min_image_extent.width,
                surface_caps.max_image_extent.width,
            ),
            height: height.clamp(
                surface_caps.min_image_extent.height,
                surface_caps.max_image_extent.height,
            ),
        };

        super::SurfaceInfo {
            surface: self.surface,
            surface_caps,
            surface_format,
            present_mode,
            extent,
        }
    }

    pub fn get_memory_type_index(
        &self,
        properties: vk::MemoryPropertyFlags,
        requirements: vk::MemoryRequirements,
    ) -> u32 {
        (0..self.memory_properties.memory_type_count)
            .find(|i| {
                (((requirements.memory_type_bits >> i) & 1) != 0)
                    && self.memory_properties.memory_types[*i as usize]
                        .property_flags
                        .contains(properties)
            })
            .unwrap()
    }

    pub fn wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }.unwrap();
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_surface_khr(Some(self.surface), None);
            if !self.messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(Some(self.messenger), None)
            }
            self.instance.destroy_instance(None)
        }
    }
}
