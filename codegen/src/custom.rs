use proc_macro2::Span;
use syn::Ident;

pub(crate) const VARIANT_RENAMES: &[(&str, &str)] = &[
    ("OB_HOLE_FILL_FAREST", "OB_HOLE_FILL_FARTHEST"),
    // All the other types have the common prefix of OB_EXCEPTION_TYPE_
    (
        "OB_EXCEPTION_STD_EXCEPTION",
        "OB_EXCEPTION_TYPE_STD_EXCEPTION",
    ),
    // Capability flag missing the conventional `_BOOL` suffix upstream — it's
    // only ever queried via `isPropertySupported`, but Bool is the correct
    // semantic shape so the typed accessor compiles.
    (
        "OB_PROP_DEVICE_OFFLINE_AFTER_IP_CONFIG_APPLY",
        "OB_PROP_DEVICE_OFFLINE_AFTER_IP_CONFIG_APPLY_BOOL",
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

/// Struct properties whose `Get`/`Set` impls are written by hand and must not
/// be emitted by codegen. The codegen still emits the `Property` impl via
/// `define_property_base!`; the manual impls live in
/// `src/sys/prop/custom_impls.rs`.
///
/// `MultiDeviceSyncConfig` is overridden because the generic structured-data
/// path returns a 16-byte legacy `OBDeviceSyncConfig` on devices that speak
/// the old sync protocol. The dedicated `ob_device_get_multi_device_sync_config`
/// / `ob_device_set_multi_device_sync_config` functions perform the protocol
/// negotiation and always exchange the new 25-byte struct.
pub(crate) fn has_custom_property_impl(name: &str) -> bool {
    matches!(name, "MultiDeviceSyncConfig")
}
