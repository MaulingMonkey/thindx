#![allow(missing_docs)]

use bytemuck::*;

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::*;



#[allow(non_camel_case_types)] #[cfg(    target_arch = "x86" )] type Pack4OnX86_Inner<T> = Pack4<T>;
#[allow(non_camel_case_types)] #[cfg(not(target_arch = "x86"))] type Pack4OnX86_Inner<T> = PackN<T>;

#[repr(C, packed)]      pub struct Pack1<T>(T);
#[repr(C, align(4))]    pub struct Pack4<T>(Pack1<T>);
#[repr(transparent)]    pub struct PackN<T>(T); // "N" for "N"ative
#[repr(transparent)]    pub struct Pack4OnX86<T>(Pack4OnX86_Inner<T>);



impl<T> Pack1<T> {
    pub const fn new(value: T) -> Self { Self(value) }
    pub fn set(&mut self, value: T) { self.0 = value; }
    pub fn into_inner(self) -> T { self.0 }
}

impl<T> Pack4<T> {
    pub const fn new(value: T) -> Self { Self(Pack1::new(value)) }
    pub fn set(&mut self, value: T) { self.0.0 = value; }
    pub fn into_inner(self) -> T { self.0.0 }
}

impl<T> PackN<T> {
    pub const fn new(value: T) -> Self { Self(value) }
    pub fn set(&mut self, value: T) { self.0 = value; }
    pub fn into_inner(self) -> T { self.0 }
}

impl<T> Pack4OnX86<T> {
    pub const fn new(value: T) -> Self { Self(Pack4OnX86_Inner::new(value)) }
    pub fn set(&mut self, value: T) { self.0.set(value) }
    pub fn into_inner(self) -> T { self.0.into_inner() }
}

impl<T: Copy> Pack1     <T> { pub fn get(&self) -> T { self.into_inner() } }
impl<T: Copy> Pack4     <T> { pub fn get(&self) -> T { self.into_inner() } }
impl<T: Copy> PackN     <T> { pub fn get(&self) -> T { self.into_inner() } }
impl<T: Copy> Pack4OnX86<T> { pub fn get(&self) -> T { self.into_inner() } }

// `self.0.clone()` uses an unaligned ref, so `Clone for Pack...` requires `T: Copy`, not merely `T: Clone`.
impl<T: Copy> Clone for Pack1       <T> { fn clone(&self) -> Self { Self(self.0) } }
impl<T: Copy> Clone for Pack4       <T> { fn clone(&self) -> Self { Self(self.0) } }
impl<T: Copy> Clone for PackN       <T> { fn clone(&self) -> Self { Self(self.0) } }
impl<T: Copy> Clone for Pack4OnX86  <T> { fn clone(&self) -> Self { Self(self.0) } }
impl<T: Copy> Copy  for Pack1       <T> {}
impl<T: Copy> Copy  for Pack4       <T> {}
impl<T: Copy> Copy  for PackN       <T> {}
impl<T: Copy> Copy  for Pack4OnX86  <T> {}

impl<T: Default> Default for Pack1      <T> { fn default() -> Self { Self::new(T::default()) } }
impl<T: Default> Default for Pack4      <T> { fn default() -> Self { Self::new(T::default()) } }
impl<T: Default> Default for PackN      <T> { fn default() -> Self { Self::new(T::default()) } }
impl<T: Default> Default for Pack4OnX86 <T> { fn default() -> Self { Self::new(T::default()) } }

impl<T: Copy + PartialEq> PartialEq for Pack1      <T> { fn eq(&self, other: &Self) -> bool { self.get() == other.get() } }
impl<T: Copy + PartialEq> PartialEq for Pack4      <T> { fn eq(&self, other: &Self) -> bool { self.get() == other.get() } }
impl<T: Copy + PartialEq> PartialEq for PackN      <T> { fn eq(&self, other: &Self) -> bool { self.get() == other.get() } }
impl<T: Copy + PartialEq> PartialEq for Pack4OnX86 <T> { fn eq(&self, other: &Self) -> bool { self.get() == other.get() } }

impl<T: Copy + Eq> Eq for Pack1      <T> {}
impl<T: Copy + Eq> Eq for Pack4      <T> {}
impl<T: Copy + Eq> Eq for PackN      <T> {}
impl<T: Copy + Eq> Eq for Pack4OnX86 <T> {}

impl<T: Copy + PartialOrd> PartialOrd for Pack1      <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.get().partial_cmp(&other.get()) } }
impl<T: Copy + PartialOrd> PartialOrd for Pack4      <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.get().partial_cmp(&other.get()) } }
impl<T: Copy + PartialOrd> PartialOrd for PackN      <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.get().partial_cmp(&other.get()) } }
impl<T: Copy + PartialOrd> PartialOrd for Pack4OnX86 <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.get().partial_cmp(&other.get()) } }

impl<T: Copy + Ord> Ord for Pack1      <T> { fn cmp(&self, other: &Self) -> Ordering { self.get().cmp(&other.get()) } }
impl<T: Copy + Ord> Ord for Pack4      <T> { fn cmp(&self, other: &Self) -> Ordering { self.get().cmp(&other.get()) } }
impl<T: Copy + Ord> Ord for PackN      <T> { fn cmp(&self, other: &Self) -> Ordering { self.get().cmp(&other.get()) } }
impl<T: Copy + Ord> Ord for Pack4OnX86 <T> { fn cmp(&self, other: &Self) -> Ordering { self.get().cmp(&other.get()) } }

impl<T: Copy + Hash> Hash for Pack1      <T> { fn hash<H: Hasher>(&self, hasher: &mut H) { self.get().hash(hasher) } }
impl<T: Copy + Hash> Hash for Pack4      <T> { fn hash<H: Hasher>(&self, hasher: &mut H) { self.get().hash(hasher) } }
impl<T: Copy + Hash> Hash for PackN      <T> { fn hash<H: Hasher>(&self, hasher: &mut H) { self.get().hash(hasher) } }
impl<T: Copy + Hash> Hash for Pack4OnX86 <T> { fn hash<H: Hasher>(&self, hasher: &mut H) { self.get().hash(hasher) } }

impl<T: Copy + Debug> Debug for Pack1      <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.get(), fmt) } }
impl<T: Copy + Debug> Debug for Pack4      <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.get(), fmt) } }
impl<T: Copy + Debug> Debug for PackN      <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.get(), fmt) } }
impl<T: Copy + Debug> Debug for Pack4OnX86 <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.get(), fmt) } }

impl<T: Copy + Display> Display for Pack1      <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.get(), fmt) } }
impl<T: Copy + Display> Display for Pack4      <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.get(), fmt) } }
impl<T: Copy + Display> Display for PackN      <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.get(), fmt) } }
impl<T: Copy + Display> Display for Pack4OnX86 <T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.get(), fmt) } }

unsafe impl<T: Zeroable> Zeroable for Pack1      <T> {}
unsafe impl<T: Zeroable> Zeroable for Pack4      <T> {}
unsafe impl<T: Zeroable> Zeroable for PackN      <T> {}
unsafe impl<T: Zeroable> Zeroable for Pack4OnX86 <T> {}

unsafe impl<T: Pod> Pod for Pack1      <T> {}
unsafe impl<T: Pod> Pod for Pack4      <T> {}
unsafe impl<T: Pod> Pod for PackN      <T> {}
unsafe impl<T: Pod> Pod for Pack4OnX86 <T> {}
