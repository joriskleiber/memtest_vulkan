use std::fmt;

use ash::vk;

pub struct MemoryHeapFlagsDebugWrapper(pub vk::MemoryHeapFlags);

impl fmt::Debug for MemoryHeapFlagsDebugWrapper {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.intersects(vk::MemoryHeapFlags::DEVICE_LOCAL) {
            write!(f, "{}", "DEVICE_LOCAL")?;
        } else {
            write!(f, "{}", "(empty)")?;
        }

        Ok(())
    }
}

pub struct MemoryHeapDebugWrapper(pub vk::MemoryHeap);

impl fmt::Debug for MemoryHeapDebugWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryHeap")
            .field("size", &self.0.size)
            .field("flags", &MemoryHeapFlagsDebugWrapper(self.0.flags))
            .finish()
    }
}

pub struct MemoryPropertyFlagsDebugWrapper(pub vk::MemoryPropertyFlags);

impl fmt::Debug for MemoryPropertyFlagsDebugWrapper {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flags = vec![];

        if self.0.intersects(vk::MemoryPropertyFlags::DEVICE_LOCAL) {
            flags.push("DEVICE_LOCAL");
        }
        if self.0.intersects(vk::MemoryPropertyFlags::HOST_VISIBLE) {
            flags.push("HOST_VISIBLE");
        }
        if self.0.intersects(vk::MemoryPropertyFlags::HOST_COHERENT) {
            flags.push("HOST_COHERENT");
        }
        if self.0.intersects(vk::MemoryPropertyFlags::HOST_CACHED) {
            flags.push("HOST_CACHED");
        }
        if self.0.intersects(vk::MemoryPropertyFlags::LAZILY_ALLOCATED) {
            flags.push("LAZILY_ALLOCATED");
        }

        for (i, flag) in flags.iter().enumerate() {
            if i > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{}", flag)?;
        }

        Ok(())
    }
}

pub struct MemoryTypeDebugWrapper(pub vk::MemoryType);

impl fmt::Debug for MemoryTypeDebugWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryType")
            .field(
                "property_flags",
                &MemoryPropertyFlagsDebugWrapper(self.0.property_flags),
            )
            .field("heap_index", &self.0.heap_index)
            .finish()
    }
}
