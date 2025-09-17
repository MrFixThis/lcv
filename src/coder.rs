pub mod ami;
pub mod hdb3;
pub mod manch;
pub mod mlt3;
pub mod nrz;
pub mod rz;

use std::{
    any::{Any, TypeId},
    fmt,
    fmt::Display,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SignalElem {
    pub ti: f32,
    pub tf: f32,
    pub lvl: f32,
}

impl SignalElem {
    pub fn new(ti: f32, tf: f32, lvl: f32) -> Self {
        Self { ti, tf, lvl }
    }

    #[inline(always)]
    pub fn ti(&self) -> f32 {
        self.ti
    }

    #[inline(always)]
    pub fn tf(&self) -> f32 {
        self.tf
    }

    #[inline(always)]
    pub fn lvl(&self) -> f32 {
        self.lvl
    }
}

pub trait LineCoder: 'static {
    fn encode(&self, bits: &[u8]) -> Box<[SignalElem]>;

    fn on_tb(&mut self, tb: f32) -> anyhow::Result<()>;

    fn on_v(&mut self, v: f32) -> anyhow::Result<()>;

    fn on_duty(&mut self, duty: f32) -> anyhow::Result<()> {
        let _ = duty;
        Ok(())
    }

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

    unsafe fn downcast_raw_mut(&mut self, id: TypeId) -> Option<*mut ()> {
        if TypeId::of::<Self>() == id {
            Some(self as *mut Self as *mut ())
        } else {
            None
        }
    }
}

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

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        unsafe {
            let ptr = self.downcast_raw_mut(TypeId::of::<T>())?;
            if ptr.is_null() {
                None
            } else {
                Some(&mut *(ptr as *mut T))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoderInitErr {
    InvalidBitPeriod,
    BadAmplitude,
    WrongDuty,
}

impl CoderInitErr {
    const INV_BIT_PERIOD_MSG: &str = "Invalid bit period specified. Must be Tb > 0";
    const BAD_AMPLITUDE_MSG: &str = "Bad amplitude specified";
    const WRONG_DUTY_MSG: &str = "Wrong duty specified. Must be 0 < duty >= 1";
}

impl Display for CoderInitErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoderInitErr::InvalidBitPeriod => f.pad(Self::INV_BIT_PERIOD_MSG),
            CoderInitErr::BadAmplitude => f.pad(Self::BAD_AMPLITUDE_MSG),
            CoderInitErr::WrongDuty => f.pad(Self::WRONG_DUTY_MSG),
        }
    }
}

impl std::error::Error for CoderInitErr {}
