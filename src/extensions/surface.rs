use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;

pub struct Surface {
    pub handle: vk::Instance,
    pub surface_fn: vk::SurfaceFn,
}

impl Surface {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<Surface, String> {
        let surface_fn = vk::SurfaceFn::load(|name| {
            unsafe { mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr())) }
        })?;
        Ok(Surface {
            handle: instance.handle(),
            surface_fn: surface_fn,
        })
    }

    pub fn get_physical_device_surface_support_khr(&self,
                                                   physical_device: vk::PhysicalDevice,
                                                   queue_index: vk::uint32_t,
                                                   surface: vk::SurfaceKHR)
                                                   -> bool {
        unsafe {
            let mut b = mem::uninitialized();
            self.surface_fn
                .get_physical_device_surface_support_khr(physical_device,
                                                         queue_index,
                                                         surface,
                                                         &mut b);
            b > 0
        }
    }
    pub fn get_physical_device_surface_present_modes_khr(&self,
                                                         physical_device: vk::PhysicalDevice,
                                                         surface: vk::SurfaceKHR)
                                                         -> VkResult<Vec<vk::PresentModeKHR>> {
        unsafe {
            let mut count = 0;
            self.surface_fn.get_physical_device_surface_present_modes_khr(physical_device,
                                                                          surface,
                                                                          &mut count,
                                                                          ptr::null_mut());
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.surface_fn
                .get_physical_device_surface_present_modes_khr(physical_device,
                                                               surface,
                                                               &mut count,
                                                               v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_physical_device_surface_capabilities_khr(&self,
                                                        physical_device: vk::PhysicalDevice,
                                                        surface: vk::SurfaceKHR)
                                                        -> VkResult<vk::SurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = mem::uninitialized();
            let err_code = self.surface_fn
                .get_physical_device_surface_capabilities_khr(physical_device,
                                                              surface,
                                                              &mut surface_capabilities);
            match err_code {
                vk::Result::Success => Ok(surface_capabilities),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_physical_device_surface_formats_khr(&self,
                                                   physical_device: vk::PhysicalDevice,
                                                   surface: vk::SurfaceKHR)
                                                   -> VkResult<Vec<vk::SurfaceFormatKHR>> {
        unsafe {
            let mut count = 0;
            self.surface_fn.get_physical_device_surface_formats_khr(physical_device,
                                                                    surface,
                                                                    &mut count,
                                                                    ptr::null_mut());
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.surface_fn
                .get_physical_device_surface_formats_khr(physical_device,
                                                         surface,
                                                         &mut count,
                                                         v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub unsafe fn destroy_surface_khr(&self, surface: vk::SurfaceKHR) {
        unsafe {
            self.surface_fn.destroy_surface_khr(self.handle, surface, ptr::null());
        }
    }
}