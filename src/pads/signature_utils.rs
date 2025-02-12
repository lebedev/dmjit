pub struct ExSignature {
    pub data: &'static [Option<u8>],
    pub marker_pos: u32
}

use std::ops::{Deref, DerefMut};
use auxtools::sigscan::Scanner;
use itertools::Itertools;
use once_cell::sync::Lazy;
use dmjit_macro::ex_signature;

pub(crate) struct ForceSyncSend<T: Sized>(T);

unsafe impl<T: Sized> Sync for ForceSyncSend<T> {}
unsafe impl<T: Sized> Send for ForceSyncSend<T> {}

impl<T: Sized> Deref for ForceSyncSend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct DynamicBoundVariable<T: Sized>(Lazy<*mut T>);
unsafe impl<T: Sized> Sync for DynamicBoundVariable<T> {}
unsafe impl<T: Sized> Send for DynamicBoundVariable<T> {}

impl<T: Sized> Deref for DynamicBoundVariable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &**self.0 }
    }
}

impl<T: Sized> DerefMut for DynamicBoundVariable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut **self.0 }
    }
}

impl<T: Sized> Into<*mut T> for &DynamicBoundVariable<T> {
    fn into(self) -> *mut T {
        *self.0.deref()
    }
}

impl<T> DynamicBoundVariable<T> {
    pub(crate) const fn new(init: fn() -> *mut T) -> Self {
        Self(Lazy::new(init))
    }
    pub(crate) fn init(&self) {
        Lazy::force(&self.0);
    }
    pub(crate) fn origin(&self) -> *mut T { *self.0 }
}

pub(crate) struct DynamicBoundFunction<T: Sized>(Lazy<T>);

impl<T: Sized> Deref for DynamicBoundFunction<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> DynamicBoundFunction<T> {
    pub(crate) const fn new(init: fn() -> T) -> Self {
        Self(Lazy::new(init))
    }
    pub(crate) fn init(&self) {
        Lazy::force(&self.0);
    }
}


pub(crate) static SCANNER: Lazy<ForceSyncSend<Scanner>> = Lazy::new(|| {
    ForceSyncSend(auxtools::sigscan::Scanner::for_module(auxtools::BYONDCORE).unwrap())
});

pub fn find_by_call(scanner: &Scanner, signature: ExSignature) -> *mut u8 {
    let position = scanner.find(signature.data);
    if position == None {
        panic!("Failed to perform signature search: no matches");
    }

    let pointer = position.unwrap();
    let marker_pos = signature.marker_pos;

    return unsafe {
        pointer.add(marker_pos as usize).offset(4).offset(*(pointer.add(marker_pos as usize) as *const isize))
    }
}

pub fn find_by_reference(scanner: &Scanner, signature: ExSignature) -> *mut u8 {
    let position = scanner.find(signature.data);
    if position == None {
        panic!("Failed to perform signature search: no matches");
    }

    let pointer = position.unwrap();
    let marker_pos = signature.marker_pos;

    return unsafe {
        *(pointer.add(marker_pos as usize) as *const *mut u8)
    }
}
