use crate::ctypes::*;
use crate::Guid;

use bytemuck::*;

use winapi::shared::d3d9types::D3DADAPTER_IDENTIFIER9;

use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter, UpperHex};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dadapter-identifier9)\]
/// D3DADAPTER_IDENTIFIER9
///
/// Contains various information about an adapter
#[derive(Clone, Copy, Default)]
#[derive(Zeroable)] // !Pod: trailing padding on x64
#[repr(C)] pub struct AdapterIdentifier {
    /// Used for presentation to the user. This should not be used to identify particular drivers, because many different strings might be associated with the same device and driver from different vendors.
    pub driver:             abistr::CStrBuf<u8, 512>,

    /// Used for presentation to the user.
    pub description:        abistr::CStrBuf<u8, 512>,

    /// Device name for GDI.
    pub device_name:        abistr::CStrBuf<u8, 32>,

    /// Identify the version of the Direct3D driver.
    /// It is legal to do less than and greater than comparisons on the 64-bit signed integer value.
    /// However, exercise caution if you use this element to identify problematic drivers.
    /// Instead, you should use DeviceIdentifier.
    pub driver_version:     Pack4OnX86<i64>,

    /// Can be used to help identify a particular chip set.
    /// Query this member to identify the manufacturer.
    /// The value can be zero if unknown.
    pub vendor_id:          u32,

    /// Can be used to help identify a particular chip set.
    /// Query this member to identify the type of chip set.
    /// The value can be zero if unknown.
    pub device_id:          u32,

    /// Can be used to help identify a particular chip set.
    /// Query this member to identify the subsystem, typically the particular board.
    /// The value can be zero if unknown.
    pub sub_sys_id:         u32,

    /// Can be used to help identify a particular chip set.
    /// Query this member to identify the revision level of the chip set.
    /// The value can be zero if unknown.
    pub revision:           u32,

    /// Can be queried to check changes in the driver and chip set.
    /// This GUID is a unique identifier for the driver and chip set pair.
    /// Query this member to track changes to the driver and chip set in order to generate a new profile for the graphics subsystem.
    /// DeviceIdentifier can also be used to identify particular problematic drivers.
    pub device_identifier:  Guid,

    /// Used to determine the Windows Hardware Quality Labs (WHQL) validation level for this driver and device pair.
    /// The DWORD is a packed date structure defining the date of the release of the most recent WHQL test passed by the driver.
    /// It is legal to perform < and > operations on this value.
    /// The following illustrates the date format:
    ///
    /// | Bits  | Description |
    /// | -----:| ----------- |
    /// | 31-16 | The year, a decimal number from 1999 upwards.
    /// |  15-8 | The month, a decimal number from 1 to 12.
    /// |   7-0 | The day, a decimal number from 1 to 31.
    ///
    /// The following values are also used:
    ///
    /// | Value | Description |
    /// | -----:| ----------- |
    /// |     0 | Not certified.
    /// |     1 | WHQL validated, but no date information is available.
    pub whql_level: u32,

    // 32 bits of implicit trailing padding on 64-bit
}

impl AdapterIdentifier {
    /// Used for presentation to the user. This should not be used to identify particular drivers, because many different strings might be associated with the same device and driver from different vendors.
    pub fn driver_lossy_utf8(&self) -> Cow<str> { self.driver.to_string_lossy() }

    /// Used for presentation to the user.
    pub fn description_lossy_utf8(&self) -> Cow<str> { self.description.to_string_lossy() }

    /// Device name for GDI.
    pub fn device_name_lossy_utf8(&self) -> Cow<str> { self.device_name.to_string_lossy() }
}

impl AdapterIdentifier {
    /// Used for presentation to the user. This should not be used to identify particular drivers, because many different strings might be associated with the same device and driver from different vendors.
    pub fn driver(&self) -> &[u8] { self.driver.to_units() }

    /// Used for presentation to the user.
    pub fn description(&self) -> &[u8] { self.description.to_units() }

    /// Device name for GDI.
    pub fn device_name(&self) -> &[u8] { self.device_name.to_units() }

    /// Identify the version of the Direct3D driver.
    /// It is legal to do less than and greater than comparisons on the 64-bit signed integer value.
    /// However, exercise caution if you use this element to identify problematic drivers.
    /// Instead, you should use DeviceIdentifier.
    pub fn driver_version(&self) -> i64 { self.driver_version.get() }
}

#[test] fn align_vs_cpp() {
    if cfg!(target_arch = "x86") { // or itanium?
        assert_eq!( 4, std::mem::align_of::<AdapterIdentifier>());
    } else {
        assert_eq!(std::mem::align_of::<D3DADAPTER_IDENTIFIER9>(), std::mem::align_of::<AdapterIdentifier>());
        if cfg!(target_arch = "x86_64") {
            assert_eq!( 8, std::mem::align_of::<AdapterIdentifier>());
        }
    }
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    #[ignore(align)] // See align_vs_cpp - winapi has alignment 1, real alignment should be 4 on x86, 8 otherwise
    AdapterIdentifier => D3DADAPTER_IDENTIFIER9 {
        driver              => Driver,
        description         => Description,
        device_name         => DeviceName,
        driver_version      => DriverVersion,
        vendor_id           => VendorId,
        device_id           => DeviceId,
        sub_sys_id          => SubSysId,
        revision            => Revision,
        device_identifier   => DeviceIdentifier,
        whql_level          => WHQLLevel,
    }
}

impl Debug for AdapterIdentifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("AdapterIdentifier")
            .field("driver",            &self.driver_lossy_utf8())
            .field("description",       &self.description_lossy_utf8())
            .field("device_name",       &self.device_name_lossy_utf8())
            .field("driver_version",    &Hexify(0, self.driver_version()))
            .field("vendor_id",         &Hexify(4, self.VendorId))
            .field("device_id",         &Hexify(4, self.DeviceId))
            .field("sub_sys_id",        &Hexify(8, self.SubSysId))
            .field("revision",          &Hexify(8, self.Revision))
            .field("device_identifier", &self.device_identifier)
            .field("whql_level",        &self.whql_level)
            .finish()
    }
}

struct Hexify<T: UpperHex>(usize, T);
impl<T: UpperHex> Debug for Hexify<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:0>1$X}", self.1, self.0)
    }
}

//#cpp2rust D3DADAPTER_IDENTIFIER9 = d3d::AdapterIdentifier
