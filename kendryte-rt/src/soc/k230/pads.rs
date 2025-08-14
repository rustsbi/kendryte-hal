use crate::soc::k230::IOMUX;
use kendryte_hal::iomux;
use kendryte_hal::iomux::ops::PadOps;
use kendryte_hal::iomux::{FlexPad, IntoFlexPad, pad};

pub struct Pad<const N: usize>(());

impl<const N: usize> IntoFlexPad<'static> for Pad<N> {
    fn into_flex_pad(self) -> FlexPad<'static> {
        unsafe { FlexPad::new(Pad::<N>::mmio_register_block()) }
    }
}

impl<'p, const N: usize> IntoFlexPad<'p> for &'p Pad<N> {
    fn into_flex_pad(self) -> FlexPad<'p> {
        unsafe { FlexPad::new(Pad::<N>::mmio_register_block()) }
    }
}

impl<'p, const N: usize> IntoFlexPad<'p> for &'p mut Pad<N> {
    fn into_flex_pad(self) -> FlexPad<'p> {
        unsafe { FlexPad::new(Pad::<N>::mmio_register_block()) }
    }
}

impl<const N: usize> Pad<N> {
    fn new() -> Self {
        Pad(())
    }
    pub unsafe fn mmio_register_block() -> pad::MmioRegisterBlock<'static> {
        unsafe {
            let mut iomux = IOMUX::mmio_register_block();
            iomux.steal_pads_unchecked(N)
        }
    }
}

pub struct Pads {
    pub io0: Pad<0>,
    pub io1: Pad<1>,
    pub io2: Pad<2>,
    pub io3: Pad<3>,
    pub io4: Pad<4>,
    pub io5: Pad<5>,
    pub io6: Pad<6>,
    pub io7: Pad<7>,
    pub io8: Pad<8>,
    pub io9: Pad<9>,
    pub io10: Pad<10>,
    pub io11: Pad<11>,
    pub io12: Pad<12>,
    pub io13: Pad<13>,
    pub io14: Pad<14>,
    pub io15: Pad<15>,
    pub io16: Pad<16>,
    pub io17: Pad<17>,
    pub io18: Pad<18>,
    pub io19: Pad<19>,
    pub io20: Pad<20>,
    pub io21: Pad<21>,
    pub io22: Pad<22>,
    pub io23: Pad<23>,
    pub io24: Pad<24>,
    pub io25: Pad<25>,
    pub io26: Pad<26>,
    pub io27: Pad<27>,
    pub io28: Pad<28>,
    pub io29: Pad<29>,
    pub io30: Pad<30>,
    pub io31: Pad<31>,
    pub io32: Pad<32>,
    pub io33: Pad<33>,
    pub io34: Pad<34>,
    pub io35: Pad<35>,
    pub io36: Pad<36>,
    pub io37: Pad<37>,
    pub io38: Pad<38>,
    pub io39: Pad<39>,
    pub io40: Pad<40>,
    pub io41: Pad<41>,
    pub io42: Pad<42>,
    pub io43: Pad<43>,
    pub io44: Pad<44>,
    pub io45: Pad<45>,
    pub io46: Pad<46>,
    pub io47: Pad<47>,
    pub io48: Pad<48>,
    pub io49: Pad<49>,
    pub io50: Pad<50>,
    pub io51: Pad<51>,
    pub io52: Pad<52>,
    pub io53: Pad<53>,
    pub io54: Pad<54>,
    pub io55: Pad<55>,
    pub io56: Pad<56>,
    pub io57: Pad<57>,
    pub io58: Pad<58>,
    pub io59: Pad<59>,
    pub io60: Pad<60>,
    pub io61: Pad<61>,
    pub io62: Pad<62>,
    pub io63: Pad<63>,
}

impl Pads {
    pub(crate) fn new() -> Self {
        Self {
            io0: Pad::<0>::new(),
            io1: Pad::<1>::new(),
            io2: Pad::<2>::new(),
            io3: Pad::<3>::new(),
            io4: Pad::<4>::new(),
            io5: Pad::<5>::new(),
            io6: Pad::<6>::new(),
            io7: Pad::<7>::new(),
            io8: Pad::<8>::new(),
            io9: Pad::<9>::new(),
            io10: Pad::<10>::new(),
            io11: Pad::<11>::new(),
            io12: Pad::<12>::new(),
            io13: Pad::<13>::new(),
            io14: Pad::<14>::new(),
            io15: Pad::<15>::new(),
            io16: Pad::<16>::new(),
            io17: Pad::<17>::new(),
            io18: Pad::<18>::new(),
            io19: Pad::<19>::new(),
            io20: Pad::<20>::new(),
            io21: Pad::<21>::new(),
            io22: Pad::<22>::new(),
            io23: Pad::<23>::new(),
            io24: Pad::<24>::new(),
            io25: Pad::<25>::new(),
            io26: Pad::<26>::new(),
            io27: Pad::<27>::new(),
            io28: Pad::<28>::new(),
            io29: Pad::<29>::new(),
            io30: Pad::<30>::new(),
            io31: Pad::<31>::new(),
            io32: Pad::<32>::new(),
            io33: Pad::<33>::new(),
            io34: Pad::<34>::new(),
            io35: Pad::<35>::new(),
            io36: Pad::<36>::new(),
            io37: Pad::<37>::new(),
            io38: Pad::<38>::new(),
            io39: Pad::<39>::new(),
            io40: Pad::<40>::new(),
            io41: Pad::<41>::new(),
            io42: Pad::<42>::new(),
            io43: Pad::<43>::new(),
            io44: Pad::<44>::new(),
            io45: Pad::<45>::new(),
            io46: Pad::<46>::new(),
            io47: Pad::<47>::new(),
            io48: Pad::<48>::new(),
            io49: Pad::<49>::new(),
            io50: Pad::<50>::new(),
            io51: Pad::<51>::new(),
            io52: Pad::<52>::new(),
            io53: Pad::<53>::new(),
            io54: Pad::<54>::new(),
            io55: Pad::<55>::new(),
            io56: Pad::<56>::new(),
            io57: Pad::<57>::new(),
            io58: Pad::<58>::new(),
            io59: Pad::<59>::new(),
            io60: Pad::<60>::new(),
            io61: Pad::<61>::new(),
            io62: Pad::<62>::new(),
            io63: Pad::<63>::new(),
        }
    }
}
