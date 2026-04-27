use proc_macro2::Span;
use syn::Ident;

pub(crate) const VARIANT_RENAMES: &[(&str, &str)] = &[
    ("OB_HOLE_FILL_FAREST", "OB_HOLE_FILL_FARTHEST"),
    // All the other types have the common prefix of OB_EXCEPTION_TYPE_
    (
        "OB_EXCEPTION_STD_EXCEPTION",
        "OB_EXCEPTION_TYPE_STD_EXCEPTION",
    ),
];

pub(crate) fn struct_value_type(name: &str) -> Option<Ident> {
    match name {
        "DeviceTime" => Some(Ident::new("OBDeviceTime", Span::call_site())),
        "BaselineCalibrationParam" => {
            Some(Ident::new("OBBaselineCalibrationParam", Span::call_site()))
        }
        "ColorAeRoi" | "DepthAeRoi" => Some(Ident::new("OBRegionOfInterest", Span::call_site())),
        "DeviceSerialNumber" => Some(Ident::new("OBDeviceSerialNumber", Span::call_site())),
        "DeviceTemperature" => Some(Ident::new("OBDeviceTemperature", Span::call_site())),
        "DispOffsetConfig" => Some(Ident::new("OBDispOffsetConfig", Span::call_site())),
        "MultiDeviceSyncConfig" => Some(Ident::new("OBMultiDeviceSyncConfig", Span::call_site())),
        "PresetResolutionConfig" => Some(Ident::new("OBPresetResolutionConfig", Span::call_site())),
        _ => None,
    }
}
