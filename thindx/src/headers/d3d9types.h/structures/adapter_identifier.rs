use winapi::ctypes::c_char;
use winapi::shared::d3d9types::D3DADAPTER_IDENTIFIER9;
use winapi::shared::guiddef::GUID;
use winapi::shared::ntdef::LARGE_INTEGER;

use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter, UpperHex};
use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dadapter-identifier9)\]
/// D3DADAPTER_IDENTIFIER9
///
/// Contains various information about an adapter
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct AdapterIdentifier(D3DADAPTER_IDENTIFIER9);

impl AdapterIdentifier {
    /// Used for presentation to the user. This should not be used to identify particular drivers, because many different strings might be associated with the same device and driver from different vendors.
    pub fn driver_lossy_utf8(&self) -> Cow<str> { String::from_utf8_lossy(self.driver()) }

    /// Used for presentation to the user.
    pub fn description_lossy_utf8(&self) -> Cow<str> { String::from_utf8_lossy(self.description()) }

    /// Device name for GDI.
    pub fn device_name_lossy_utf8(&self) -> Cow<str> { String::from_utf8_lossy(self.device_name()) }
}

impl AdapterIdentifier {
    /// Used for presentation to the user. This should not be used to identify particular drivers, because many different strings might be associated with the same device and driver from different vendors.
    pub fn driver(&self) -> &[u8] { until0(&self.0.Driver[..]) }

    /// Used for presentation to the user.
    pub fn description(&self) -> &[u8] { until0(&self.0.Description[..]) }

    /// Device name for GDI.
    pub fn device_name(&self) -> &[u8] { until0(&self.0.DeviceName[..]) }

    /// Identify the version of the Direct3D driver.
    /// It is legal to do less than and greater than comparisons on the 64-bit signed integer value.
    /// However, exercise caution if you use this element to identify problematic drivers.
    /// Instead, you should use DeviceIdentifier.
    pub fn driver_version(&self) -> i64 { let dv = self.0.DriverVersion; unsafe { *dv.QuadPart() } }
}

impl Default for AdapterIdentifier {
    fn default() -> Self {
        Self(D3DADAPTER_IDENTIFIER9 {
            Driver:                 [0; 512],
            Description:            [0; 512],
            DeviceName:             [0; 32],
            DriverVersion:          unsafe { std::mem::zeroed::<LARGE_INTEGER>() },
            VendorId:               0,
            DeviceId:               0,
            SubSysId:               0,
            Revision:               0,
            DeviceIdentifier:       GUID { Data1: 0, Data2: 0, Data3: 0, Data4: [0,0,0,0,0,0,0,0] },
            WHQLLevel:              0,
        })
    }
}

impl Debug for AdapterIdentifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let whql_level = self.WHQLLevel;
        f.debug_struct("AdapterIdentifier")
            .field("Driver",            &self.driver_lossy_utf8())
            .field("Description",       &self.description_lossy_utf8())
            .field("DeviceName",        &self.device_name_lossy_utf8())
            .field("DriverVersion",     &Hexify(0, self.driver_version()))
            .field("VendorId",          &Hexify(4, self.VendorId))
            .field("DeviceId",          &Hexify(4, self.DeviceId))
            .field("SubSysId",          &Hexify(8, self.SubSysId))
            .field("Revision",          &Hexify(8, self.Revision))
            .field("DeviceIdentifier",  &"{...}") // TODO
            .field("WHQLLevel",         &whql_level)
            .finish()
    }
}

impl Deref for AdapterIdentifier {
    type Target = D3DADAPTER_IDENTIFIER9;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for AdapterIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<D3DADAPTER_IDENTIFIER9> for AdapterIdentifier {
    fn from(value: D3DADAPTER_IDENTIFIER9) -> Self { Self(value) }
}

impl From<AdapterIdentifier> for D3DADAPTER_IDENTIFIER9 {
    fn from(value: AdapterIdentifier) -> Self { value.0 }
}

fn until0(slice: &[c_char]) -> &[u8] {
    let bytes = slice.split(|n| *n == 0).next().unwrap_or(&[]);
    unsafe { std::mem::transmute(bytes) } // i8 -> u8
}

struct Hexify<T: UpperHex>(usize, T);
impl<T: UpperHex> Debug for Hexify<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:0>1$X}", self.1, self.0)
    }
}

//#cpp2rust D3DADAPTER_IDENTIFIER9 = d3d::AdapterIdentifier
