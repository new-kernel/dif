use core::fmt::{Debug, Formatter};

/// This function turns an ``&str`` into a ``DifFieldNames`` variant.
#[allow(non_snake_case)]
pub fn to_DifFieldNames(str: &'static str) -> DifFieldNames {
    for i in 0..DIF_FIELD_NAMES.len() {
        if str == DIF_FIELD_NAMES[i].to_str() {
            return DIF_FIELD_NAMES[i];
        }
    }

    return DifFieldNames::None;
}

pub const DIF_FIELD_NAMES: &'static [DifFieldNames; 21] = &[
    DifFieldNames::None,
    DifFieldNames::DifName,
    DifFieldNames::DeviceName,
    DifFieldNames::BootMethod,
    DifFieldNames::DisableWDT,
    DifFieldNames::PeripheralAddress,
    DifFieldNames::SocName,
    DifFieldNames::AllocMemory,
    DifFieldNames::EnableSerial,
    DifFieldNames::EnableFrameBuffer,
    DifFieldNames::PrintingMethod,
    DifFieldNames::IrqChip,
    DifFieldNames::EnableDeviceIrqs,
    DifFieldNames::DeviceSpecificKernel,
    DifFieldNames::StartInit,
    DifFieldNames::InitInput,
    DifFieldNames::InitFs,
    DifFieldNames::InitNet,
    DifFieldNames::ShutdownOnPanic,
    DifFieldNames::RestartOnPanic,
    DifFieldNames::DisableIrqsOnPanic,
];


#[derive(Copy, Clone, PartialEq)]
pub enum DifFieldNames {
    /// Empty line. This line's value doesn't matter.
    None,

    /// The name of the file, (``"device.dif"``).
    /// This line's value is a string.
    DifName,

    /// Name of the device, (``"Device123x"``).
    /// This line's value is a string.
    DeviceName,

    /// Name of the boot method, (``"UEFI"``) or (``GPIO Boot``).
    /// This line's value is a string.
    BootMethod,

    /// Disable a WatchDog Timer. (``"true"``) or (``"false``).
    /// This line's value is a boolean.
    DisableWDT,

    /// The peripheral base address, (``"12340"``).
    /// This can be converted into a pointer by:
    /// ```rust
    /// let peripherals = DIF.get(DifFieldNames::PeripheralAddress).parse::<u32>().unwrap();
    /// let peripheral_address = peripherals as *mut u32;
    /// ```
    /// Rust's ``parse()`` doesn't convert "x" to a decimal so the address can't be the actual
    /// address.
    ///
    /// This line's value should be a decimal/integer of the address.
    PeripheralAddress,

    /// The SOC's name, (``"SOC123"``).
    /// This line's value is a string.
    SocName,

    /// Allocate memory with the ``alloc`` library.
    /// This line's value is a boolean.
    AllocMemory,

    /// Enable serial I/O.
    /// This line's value is a boolean.
    EnableSerial,

    /// Enable a framebuffer.
    /// This line's value is a boolean.
    EnableFrameBuffer,

    /// The main printing method.
    /// This line's value is a boolean.
    PrintingMethod,

    /// The IRQ chip name.
    /// This line's value is a string.
    IrqChip,

    /// Enable device specific IRQs.
    /// This line's value is a boolean.
    EnableDeviceIrqs,

    /// Start a device specific kernel (used by Novusk).
    /// This line's value is a boolean.
    DeviceSpecificKernel,

    /// Start main kernel initialization (used by Novusk).
    /// This line's value is a boolean.
    StartInit,

    /// Start a user interface.
    /// This line's value is a boolean.
    StartUser,

    /// Initialize input.
    /// This line's value is a boolean.
    InitInput,

    /// Initialize filesystem.
    /// This line's value is a boolean.
    InitFs,

    /// Initialize networking.
    /// This line's value is a boolean.
    InitNet,

    /// Shutdown on panic.
    /// This line's value is a boolean.
    ShutdownOnPanic,

    /// Restart on panic.
    /// This line's value is a boolean.
    RestartOnPanic,

    /// Disable IRQs on panic.
    /// This line's value is a boolean.
    DisableIrqsOnPanic,
}

impl Debug for DifFieldNames {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl DifFieldNames {
    /// This converts a ``DifFieldNames`` into an ``&str`` value.
    pub fn to_str(&self) -> &str {
        return match self {
            DifFieldNames::None => "None",
            DifFieldNames::DifName => "DifName",
            DifFieldNames::DeviceName => "DeviceName",
            DifFieldNames::BootMethod => "BootMethod",
            DifFieldNames::DisableWDT => "DisableWDT",
            DifFieldNames::PeripheralAddress => "PeripheralAddress",
            DifFieldNames::SocName => "SocName",
            DifFieldNames::AllocMemory => "AllocMemory",
            DifFieldNames::EnableSerial => "EnableSerial",
            DifFieldNames::EnableFrameBuffer => "EnableFrameBuffer",
            DifFieldNames::PrintingMethod => "PrintingMethod",
            DifFieldNames::IrqChip => "IrqChip",
            DifFieldNames::EnableDeviceIrqs => "EnableDeviceIrqs",
            DifFieldNames::DeviceSpecificKernel => "DeviceSpecificKernel",
            DifFieldNames::StartInit => "StartInit",
            DifFieldNames::StartUser => "StartUser",
            DifFieldNames::InitInput => "InitInput",
            DifFieldNames::InitFs => "InitFs",
            DifFieldNames::InitNet => "InitNet",
            DifFieldNames::ShutdownOnPanic => "ShutdownOnPanic",
            DifFieldNames::RestartOnPanic => "RestartOnPanic",
            DifFieldNames::DisableIrqsOnPanic => "DisableIrqsOnPanic",
        };
    }
}

pub type DifLine = (DifFieldNames, &'static str);

