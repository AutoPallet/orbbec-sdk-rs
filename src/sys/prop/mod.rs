//! Module containing the property ID traits and types
#![allow(missing_docs)]
#![allow(unused)]

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
        #[allow(clippy::empty_docs)]
        pub struct $name;

        impl $crate::sys::prop::Property for $name {
            type Value = $value;
            const ID: $crate::sys::orb::OBPropertyID = $crate::sys::orb::OBPropertyID::$name;
        }
    };
}

macro_rules! define_bool_property {
    ($name:ident, $doc:literal) => {
        define_property_base!($name, bool, $doc);

        impl $crate::sys::prop::SetProperty for $name {
            fn set_on(
                device: &mut $crate::device::Device,
                value: <$name as $crate::sys::prop::Property>::Value,
            ) -> Result<(), $crate::error::OrbbecError> {
                device
                    .inner
                    .set_bool_property(<$name as $crate::sys::prop::Property>::ID, value)
                    .map_err($crate::error::OrbbecError::from)
            }
        }

        impl $crate::sys::prop::GetProperty for $name {
            fn get_from(
                device: &$crate::device::Device,
            ) -> Result<<$name as $crate::sys::prop::Property>::Value, $crate::error::OrbbecError>
            {
                device
                    .inner
                    .get_bool_property(<$name as $crate::sys::prop::Property>::ID)
                    .map_err($crate::error::OrbbecError::from)
            }
        }
    };
}

macro_rules! define_int_property {
    ($name:ident, $doc:literal) => {
        define_property_base!($name, i32, $doc);

        impl $crate::sys::prop::SetProperty for $name {
            fn set_on(
                device: &mut $crate::device::Device,
                value: <$name as $crate::sys::prop::Property>::Value,
            ) -> Result<(), $crate::error::OrbbecError> {
                device
                    .inner
                    .set_int_property(<$name as $crate::sys::prop::Property>::ID, value)
                    .map_err($crate::error::OrbbecError::from)
            }
        }

        impl $crate::sys::prop::GetProperty for $name {
            fn get_from(
                device: &$crate::device::Device,
            ) -> Result<<$name as $crate::sys::prop::Property>::Value, $crate::error::OrbbecError>
            {
                device
                    .inner
                    .get_int_property(<$name as $crate::sys::prop::Property>::ID)
                    .map_err($crate::error::OrbbecError::from)
            }
        }
    };
}

macro_rules! define_float_property {
    ($name:ident, $doc:literal) => {
        define_property_base!($name, f32, $doc);

        impl $crate::sys::prop::SetProperty for $name {
            fn set_on(
                device: &mut $crate::device::Device,
                value: <$name as $crate::sys::prop::Property>::Value,
            ) -> Result<(), $crate::error::OrbbecError> {
                device
                    .inner
                    .set_float_property(<$name as $crate::sys::prop::Property>::ID, value)
                    .map_err($crate::error::OrbbecError::from)
            }
        }

        impl $crate::sys::prop::GetProperty for $name {
            fn get_from(
                device: &$crate::device::Device,
            ) -> Result<<$name as $crate::sys::prop::Property>::Value, $crate::error::OrbbecError>
            {
                device
                    .inner
                    .get_float_property(<$name as $crate::sys::prop::Property>::ID)
                    .map_err($crate::error::OrbbecError::from)
            }
        }
    };
}

macro_rules! define_struct_property {
    ($name:ident, $value:ty, $doc:literal) => {
        define_property_base!($name, $value, $doc);

        impl $crate::sys::prop::StructProperty for $name {}

        impl $crate::sys::prop::GetProperty for $name {
            fn get_from(
                device: &$crate::device::Device,
            ) -> Result<<$name as $crate::sys::prop::Property>::Value, $crate::error::OrbbecError>
            {
                device
                    .inner
                    .get_struct_property::<$name>()
                    .map_err($crate::error::OrbbecError::from)
            }
        }

        impl $crate::sys::prop::SetProperty for $name {
            fn set_on(
                device: &mut $crate::device::Device,
                value: <$name as $crate::sys::prop::Property>::Value,
            ) -> Result<(), $crate::error::OrbbecError> {
                device
                    .inner
                    .set_struct_property::<$name>(value)
                    .map_err($crate::error::OrbbecError::from)
            }
        }
    };
}

mod property_id_types;
mod structs;

pub use property_id_types::*;
pub use structs::*;
