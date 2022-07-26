#[cfg(test)]
mod conversion_tests {
    use dif::{to_DifFieldNames, DIF_FIELD_NAMES, DifFieldNames};

    const DIF_FIELD_NAMES_STR: &[&'static str; 21] = &[
        "None",
        "DifName",
        "DeviceName",
        "BootMethod",
        "DisableWDT",
        "PeripheralAddress",
        "SocName",
        "AllocMemory",
        "EnableSerial",
        "EnableFrameBuffer",
        "PrintingMethod",
        "IrqChip",
        "EnableDeviceIrqs",
        "DeviceSpecificKernel",
        "StartInit",
        "InitInput",
        "InitFs",
        "InitNet",
        "ShutdownOnPanic",
        "RestartOnPanic",
        "DisableIrqsOnPanic",
    ];

    #[test]
    fn str_conversion_test() {
        for i in 0..DIF_FIELD_NAMES.len() {
            assert_eq!(DIF_FIELD_NAMES_STR[i], DIF_FIELD_NAMES[i].to_str());
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn DifFieldNames_conversion_test() {
        for i in 0..DIF_FIELD_NAMES_STR.len() {
            assert_eq!(DIF_FIELD_NAMES[i], to_DifFieldNames(DIF_FIELD_NAMES_STR[i]));
        }
    }
}
