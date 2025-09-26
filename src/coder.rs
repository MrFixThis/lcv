pub mod ami;
pub mod hdb3;
pub mod manch;
pub mod mlt3;
pub mod nrz;
pub mod rz;

use std::any::{Any, TypeId};

const GLOB_BASE_TB: f64 = 1.0;
const GLOB_BASE_V: f64 = 1.0;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SigElement {
    ti: f64,
    tf: f64,
    lvl: f64,
}

impl SigElement {
    pub fn new(ti: f64, tf: f64, lvl: f64) -> Self {
        Self { ti, tf, lvl }
    }

    #[inline(always)]
    pub fn ti(&self) -> f64 {
        self.ti
    }

    #[inline(always)]
    pub fn tf(&self) -> f64 {
        self.tf
    }

    #[inline(always)]
    pub fn lvl(&self) -> f64 {
        self.lvl
    }
}

pub trait LineCoder: 'static {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]>;

    fn boxed(self) -> Box<dyn LineCoder + 'static>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    unsafe fn downcast_raw_ref(&self, id: TypeId) -> Option<*const ()> {
        if TypeId::of::<Self>() == id {
            Some(self as *const Self as *const ())
        } else {
            None
        }
    }
}

#[allow(unused)]
impl dyn LineCoder {
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        self.downcast_ref::<T>().is_some()
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        unsafe {
            let ptr = self.downcast_raw_ref(TypeId::of::<T>())?;
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *const T))
            }
        }
    }
}
