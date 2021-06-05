mod instance;
mod surface;
mod renderer;

pub use instance::Instance;
pub use surface::Surface;
pub use renderer::Renderer;

mod debug {
    use erupt::vk;

    pub unsafe extern "system" fn callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
        _: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _: *mut std::ffi::c_void,
    ) -> vk::Bool32 {
        let level = if message_severity >= vk::DebugUtilsMessageSeverityFlagBitsEXT::ERROR_EXT {
            log::Level::Error
        } else if message_severity >= vk::DebugUtilsMessageSeverityFlagBitsEXT::WARNING_EXT {
            log::Level::Warn
        } else if message_severity >= vk::DebugUtilsMessageSeverityFlagBitsEXT::INFO_EXT {
            log::Level::Info
        } else if message_severity >= vk::DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE_EXT {
            log::Level::Debug
        } else {
            log::Level::Trace
        };
        let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message).to_string_lossy();
        log::log!(level, "{}", message);
        vk::FALSE
    }
}

use erupt::vk;

#[derive(Copy, Clone)]
struct SurfaceInfo {
    surface: vk::SurfaceKHR,
    surface_caps: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D
}

#[derive(Copy, Clone)]
struct QueueInfo {
    family: u32,
    queue: vk::Queue
}

#[derive(Copy, Clone)]
struct SyncObject {
    in_flight: vk::Fence,
    image_available: vk::Semaphore,
    render_finished: vk::Semaphore,
    command_buffer: vk::CommandBuffer,
}
