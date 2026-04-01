use crate::{device::Device, error::OrbbecError};

/// All device properties must implement this trait
pub trait Property {
    type Value: Sized;
    const ID: crate::sys::orb::OBPropertyID;
}

/// All device properties that can be set must implement this trait
pub trait SetProperty: Property {
    fn set_on(device: &mut Device, value: Self::Value) -> Result<(), OrbbecError>;
}

pub trait GetProperty: Property {
    fn get_from(device: &Device) -> Result<Self::Value, OrbbecError>;
}

pub trait StructProperty: Property {}

macro_rules! define_property_base {
    ($name:ident, $value:ty, $doc:literal) => {
        #[doc = $doc]
        pub struct $name;

        impl Property for $name {
            type Value = $value;
            const ID: crate::sys::orb::OBPropertyID = crate::sys::orb::OBPropertyID::$name;
        }
    };
}

macro_rules! define_bool_property {
    ($name:ident, $doc:literal) => {
        define_property_base!($name, bool, $doc);

        impl SetProperty for $name {
            fn set_on(device: &mut Device, value: Self::Value) -> Result<(), OrbbecError> {
                device
                    .inner
                    .set_bool_property(Self::ID, value)
                    .map_err(OrbbecError::from)
            }
        }

        impl GetProperty for $name {
            fn get_from(device: &Device) -> Result<Self::Value, OrbbecError> {
                device
                    .inner
                    .get_bool_property(Self::ID)
                    .map_err(OrbbecError::from)
            }
        }
    };
}

macro_rules! define_int_property {
    ($name:ident, $doc:literal) => {
        define_property_base!($name, i32, $doc);

        impl SetProperty for $name {
            fn set_on(device: &mut Device, value: Self::Value) -> Result<(), OrbbecError> {
                device
                    .inner
                    .set_int_property(Self::ID, value)
                    .map_err(OrbbecError::from)
            }
        }

        impl GetProperty for $name {
            fn get_from(device: &Device) -> Result<Self::Value, OrbbecError> {
                device
                    .inner
                    .get_int_property(Self::ID)
                    .map_err(OrbbecError::from)
            }
        }
    };
}

macro_rules! define_float_property {
    ($name:ident, $doc:literal) => {
        define_property_base!($name, f32, $doc);

        impl SetProperty for $name {
            fn set_on(device: &mut Device, value: Self::Value) -> Result<(), OrbbecError> {
                device
                    .inner
                    .set_float_property(Self::ID, value)
                    .map_err(OrbbecError::from)
            }
        }

        impl GetProperty for $name {
            fn get_from(device: &Device) -> Result<Self::Value, OrbbecError> {
                device
                    .inner
                    .get_float_property(Self::ID)
                    .map_err(OrbbecError::from)
            }
        }
    };
}

macro_rules! define_struct_property {
    ($name:ident, $value:ty, $doc:literal) => {
        define_property_base!($name, $value, $doc);

        impl StructProperty for $name {}

        impl GetProperty for $name {
            fn get_from(device: &Device) -> Result<Self::Value, OrbbecError> {
                device
                    .inner
                    .get_struct_property::<Self>()
                    .map_err(OrbbecError::from)
            }
        }
    };
}
