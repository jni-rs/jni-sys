struct Index {
    byte: usize,
    mask: u8,
}

impl Index {
    #[inline]
    const fn position(index: usize) -> Self {
        #[cfg(target_endian = "big")]
        let bit = 7 - (index % 8);
        #[cfg(not(target_endian = "big"))]
        let bit = index % 8;

        Self {
            byte: index / 8,
            mask: 1 << bit,
        }
    }
}

impl crate::jvmtiCapabilities {
    pub const EMPTY: Self = Self { inner: [0; 16] };

    #[inline]
    pub const fn get(&self, bit: usize) -> bool {
        let Index { byte, mask } = Index::position(bit);
        self.inner[byte] & mask == mask
    }

    #[inline]
    pub const fn with(mut self, bit: usize, value: bool) -> Self {
        // mutable reference was added in 1.83, but MSRV is 1.77, so pass this by value
        let Index { byte, mask } = Index::position(bit);
        if value {
            self.inner[byte] |= mask;
        } else {
            self.inner[byte] &= !mask;
        }
        self
    }
}

macro_rules! capabilities {
    ($($bit:ident,)+) => {
        #[repr(usize)]
        enum Position {
            $($bit,)+
        }

        impl crate::jvmtiCapabilities {
            $(
            pub const $bit: usize = Position::$bit as usize;
            )+
        }

        impl ::core::fmt::Debug for crate::jvmtiCapabilities {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct("jvmtiCapabilities")
                    $(
                    .field(stringify!($bit), &self.get(Self::$bit))
                    )+
                    .finish_non_exhaustive()
            }
        }
    }
}

capabilities! {
    CAN_TAG_OBJECTS_BIT,
    CAN_GENERATE_FIELD_MODIFICATION_EVENTS_BIT,
    CAN_GENERATE_FIELD_ACCESS_EVENTS_BIT,
    CAN_GET_BYTECODES_BIT,
    CAN_GET_SYNTHETIC_ATTRIBUTE_BIT,
    CAN_GET_OWNED_MONITOR_INFO_BIT,
    CAN_GET_CURRENT_CONTENDED_MONITOR_BIT,
    CAN_GET_MONITOR_INFO_BIT,
    CAN_POP_FRAME_BIT,
    CAN_REDEFINE_CLASSES_BIT,
    CAN_SIGNAL_THREAD_BIT,
    CAN_GET_SOURCE_FILE_NAME_BIT,
    CAN_GET_LINE_NUMBERS_BIT,
    CAN_GET_SOURCE_DEBUG_EXTENSION_BIT,
    CAN_ACCESS_LOCAL_VARIABLES_BIT,
    CAN_MAINTAIN_ORIGINAL_METHOD_ORDER_BIT,
    CAN_GENERATE_SINGLE_STEP_EVENTS_BIT,
    CAN_GENERATE_EXCEPTION_EVENTS_BIT,
    CAN_GENERATE_FRAME_POP_EVENTS_BIT,
    CAN_GENERATE_BREAKPOINT_EVENTS_BIT,
    CAN_SUSPEND_BIT,
    CAN_REDEFINE_ANY_CLASS_BIT,
    CAN_GET_CURRENT_THREAD_CPU_TIME_BIT,
    CAN_GET_THREAD_CPU_TIME_BIT,
    CAN_GENERATE_METHOD_ENTRY_EVENTS_BIT,
    CAN_GENERATE_METHOD_EXIT_EVENTS_BIT,
    CAN_GENERATE_ALL_CLASS_HOOK_EVENTS_BIT,
    CAN_GENERATE_COMPILED_METHOD_LOAD_EVENTS_BIT,
    CAN_GENERATE_MONITOR_EVENTS_BIT,
    CAN_GENERATE_VM_OBJECT_ALLOC_EVENTS_BIT,
    CAN_GENERATE_NATIVE_METHOD_BIND_EVENTS_BIT,
    CAN_GENERATE_GARBAGE_COLLECTION_EVENTS_BIT,
    CAN_GENERATE_OBJECT_FREE_EVENTS_BIT,
    CAN_FORCE_EARLY_RETURN_BIT,
    CAN_GET_OWNED_MONITOR_STACK_DEPTH_INFO_BIT,
    CAN_GET_CONSTANT_POOL_BIT,
    CAN_SET_NATIVE_METHOD_PREFIX_BIT,
    CAN_RETRANSFORM_CLASSES_BIT,
    CAN_RETRANSFORM_ANY_CLASS_BIT,
    CAN_GENERATE_RESOURCE_EXHAUSTION_HEAP_EVENTS_BIT,
    CAN_GENERATE_RESOURCE_EXHAUSTION_THREADS_EVENTS_BIT,
    CAN_GENERATE_EARLY_VMSTART_BIT,
    CAN_GENERATE_EARLY_CLASS_HOOK_EVENTS_BIT,
    CAN_GENERATE_SAMPLED_OBJECT_ALLOC_EVENTS_BIT,
    CAN_SUPPORT_VIRTUAL_THREADS_BIT,
}
