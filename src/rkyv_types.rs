// This file is unusual because it is intended to be shared by build and regular code.

use rkyv::Archive;
#[cfg(not(exe))]
use std::vec::Vec;
#[cfg(exe)]
use alloc::vec::Vec;

#[derive(Archive, Debug, PartialEq)]
#[cfg_attr(exe, derive(rkyv::Deserialize))]
#[cfg_attr(not(exe), derive(rkyv::Serialize))]
#[archive(compare(PartialEq))]
pub struct RawImage {
    pub w:u16, pub h:u16,
    pub pixels: Vec<u16>,
    pub flippable: bool // If false, reversing face doesn't change sprite
}
