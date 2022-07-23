#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DifFieldNames {
    /// Empty line. This line's value doesn't matter.
    None,

    /// The name of the file. This line's value is a string.
    DifName,

    /// Name of the device. This line's value is a string.
    DeviceName,

    /// Name of the boot method/loader. This line's value is a string.
    BootMethod,

    /// The peripheral base address. This line's value should be a decimal/integer of the address.
    PeripheralAddress,

    /// The SOC's name. This line's value is a string.
    SocName,

    /// Allocate memory with the ``alloc`` library. This line's value is a boolean.
    AllocMemory,

    /// Enable serial I/O. This line's value is a boolean.
    EnableSerial,

    /// Enable a framebuffer. This line's value is a boolean.
    EnableFrameBuffer,

    /// The main printing method. This line's value is a boolean.
    PrintingMethod,

    /// The IRQ chip name. This line's value is a string.
    IrqChip,

    /// Enable device specific IRQs. This line's value is a boolean.
    EnableDeviceIrqs,

    /// Start a device specific kernel (used by Novusk). This line's value is a boolean.
    DeviceSpecificKernel,

    /// Start main kernel initialization (used by Novusk). This line's value is a boolean.
    StartInit,

    /// Initialize input. This line's value is a boolean.
    InitInput,

    /// Initialize filesystem. This line's value is a boolean.
    InitFs,

    /// Initialize networking. This line's value is a boolean.
    InitNet,

    /// Shutdown on panic. This line's value is a boolean.
    ShutdownOnPanic,

    /// Disable IRQs on panic. This line's value is a boolean.
    DisableIrqsOnPanic,
}

impl DifFieldNames {
    pub fn to_str(&self) -> &str {
        return match self {
            DifFieldNames::None => "None",
            DifFieldNames::DifName => "DifName",
            DifFieldNames::DeviceName => "DeviceName",
            DifFieldNames::BootMethod => "BootMethod",
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
            DifFieldNames::InitInput => "InitInput",
            DifFieldNames::InitFs => "InitFs",
            DifFieldNames::InitNet => "InitNet",
            DifFieldNames::ShutdownOnPanic => "ShutdownOnPanic",
            DifFieldNames::DisableIrqsOnPanic => "DisableIrqsOnPanic",
        };
    }
}

pub type DifLine = (DifFieldNames, &'static str);
