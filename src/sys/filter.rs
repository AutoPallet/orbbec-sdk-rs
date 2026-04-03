//! Filter related operations
use std::ffi::CStr;

use super::frame::OBFrame;
use super::orb::OBFilterConfigValueType;
use super::stream::OBStreamProfile;
use super::{OBError, call_ob_function, drop_ob_object, impl_ob_method, orb};

/// Filter Config Schema Item
pub struct OBFilterConfigSchemaItem {
    inner: orb::ob_filter_config_schema_item,
}

impl OBFilterConfigSchemaItem {
    /// Name of the configuration item
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.name) }
    }

    /// Value type of the configuration item
    pub fn value_type(&self) -> OBFilterConfigValueType {
        OBFilterConfigValueType::from(self.inner.type_)
    }

    /// Minimum value casted to double
    pub fn minimum(&self) -> f64 {
        self.inner.min
    }

    /// Maximum value casted to double
    pub fn maximum(&self) -> f64 {
        self.inner.max
    }

    /// Step value casted to double
    pub fn step(&self) -> f64 {
        self.inner.step
    }

    /// Default value casted to double
    pub fn default(&self) -> f64 {
        self.inner.def
    }

    /// Description of the configuration item
    pub fn description(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.desc) }
    }
}

impl From<orb::ob_filter_config_schema_item> for OBFilterConfigSchemaItem {
    fn from(item: orb::ob_filter_config_schema_item) -> Self {
        OBFilterConfigSchemaItem { inner: item }
    }
}

/// Filter Config Schema List
pub struct OBFilterConfigSchemaList {
    inner: *mut orb::ob_filter_config_schema_list,
}

drop_ob_object!(
    OBFilterConfigSchemaList,
    ob_delete_filter_config_schema_list
);

impl OBFilterConfigSchemaList {
    pub(crate) fn new(inner: *mut orb::ob_filter_config_schema_list) -> Self {
        OBFilterConfigSchemaList { inner }
    }

    impl_ob_method!(
        /// Get the number of filter config schemas in the list
        get_count => u32,
        orb::ob_filter_config_schema_list_get_count,
    );

    /// Get the config schema item at the specified index
    pub fn get_filter_config_item(&self, index: u32) -> Result<OBFilterConfigSchemaItem, OBError> {
        let item = call_ob_function!(orb::ob_filter_config_schema_list_get_item, self.inner, index)?;
        Ok(OBFilterConfigSchemaItem::from(item))
    }
}

/// Generic Filter Class
pub struct OBFilter {
    inner: *mut orb::ob_filter,
}

drop_ob_object!(OBFilter, ob_delete_filter);

impl OBFilter {
    /// Create a filter object by name
    pub fn new(name: &CStr) -> Result<Option<Self>, OBError> {
        let filter = call_ob_function!(orb::ob_create_filter, name.as_ptr())?;
        Ok(if filter.is_null() {
            None
        } else {
            Some(OBFilter { inner: filter })
        })
    }

    /// Process the input frame and return the processed frame
    pub fn process(&self, input_frame: &OBFrame) -> Result<OBFrame, OBError> {
        let frame = call_ob_function!(orb::ob_filter_process, self.inner, input_frame.inner())?;
        Ok(OBFrame::new(frame))
    }

    /// Get the filter config schema list of the filter
    pub fn get_config_schema_list(&self) -> Result<OBFilterConfigSchemaList, OBError> {
        let list = call_ob_function!(orb::ob_filter_get_config_schema_list, self.inner)?;
        Ok(OBFilterConfigSchemaList::new(list))
    }

    /// Get the filter config value by name and cast to double
    pub fn get_config_value(&self, name: &CStr) -> Result<f64, OBError> {
        call_ob_function!(orb::ob_filter_get_config_value, self.inner, name.as_ptr())
    }

    /// Set the filter config value by name.
    /// The pass into value type is double, which will be cast to the actual type inside the filter
    pub fn set_config_value(&self, name: &CStr, value: f64) -> Result<(), OBError> {
        call_ob_function!(orb::ob_filter_set_config_value, self.inner, name.as_ptr(), value)
    }

    /// Set the stream profile to which the filter will align to.
    /// Only valid for Align filter
    pub fn set_align_to(&self, stream_profile: &OBStreamProfile) -> Result<(), OBError> {
        call_ob_function!(
            orb::ob_align_filter_set_align_to_stream_profile,
            self.inner,
            stream_profile.inner()
        )
    }
}
