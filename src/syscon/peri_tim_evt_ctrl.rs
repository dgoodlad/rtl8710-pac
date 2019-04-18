#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERI_TIM_EVT_CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT3_ENR {
    bits: bool,
}
impl PERI_GT_EVT3_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT3_SRC_SELR {
    bits: u8,
}
impl PERI_GT_EVT3_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT3_PULSE_DURR {
    bits: u8,
}
impl PERI_GT_EVT3_PULSE_DURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT2_ENR {
    bits: bool,
}
impl PERI_GT_EVT2_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT2_SRC_SELR {
    bits: u8,
}
impl PERI_GT_EVT2_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT2_PULSE_DURR {
    bits: u8,
}
impl PERI_GT_EVT2_PULSE_DURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT1_ENR {
    bits: bool,
}
impl PERI_GT_EVT1_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT1_SRC_SELR {
    bits: u8,
}
impl PERI_GT_EVT1_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT1_PULSE_DURR {
    bits: u8,
}
impl PERI_GT_EVT1_PULSE_DURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT0_ENR {
    bits: bool,
}
impl PERI_GT_EVT0_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT0_SRC_SELR {
    bits: u8,
}
impl PERI_GT_EVT0_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_GT_EVT0_PULSE_DURR {
    bits: u8,
}
impl PERI_GT_EVT0_PULSE_DURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT3_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT3_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT3_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT3_SRC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT3_PULSE_DURW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT3_PULSE_DURW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT2_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT2_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT2_SRC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT2_PULSE_DURW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT2_PULSE_DURW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT1_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT1_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT1_SRC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT1_PULSE_DURW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT1_PULSE_DURW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT0_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT0_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT0_SRC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_GT_EVT0_PULSE_DURW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_GT_EVT0_PULSE_DURW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn peri_gt_evt3_en(&self) -> PERI_GT_EVT3_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERI_GT_EVT3_ENR { bits }
    }
    #[doc = "Bits 28:30"]
    #[inline]
    pub fn peri_gt_evt3_src_sel(&self) -> PERI_GT_EVT3_SRC_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT3_SRC_SELR { bits }
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn peri_gt_evt3_pulse_dur(&self) -> PERI_GT_EVT3_PULSE_DURR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT3_PULSE_DURR { bits }
    }
    #[doc = "Bit 23"]
    #[inline]
    pub fn peri_gt_evt2_en(&self) -> PERI_GT_EVT2_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERI_GT_EVT2_ENR { bits }
    }
    #[doc = "Bits 20:22"]
    #[inline]
    pub fn peri_gt_evt2_src_sel(&self) -> PERI_GT_EVT2_SRC_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT2_SRC_SELR { bits }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn peri_gt_evt2_pulse_dur(&self) -> PERI_GT_EVT2_PULSE_DURR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT2_PULSE_DURR { bits }
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn peri_gt_evt1_en(&self) -> PERI_GT_EVT1_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERI_GT_EVT1_ENR { bits }
    }
    #[doc = "Bits 12:14"]
    #[inline]
    pub fn peri_gt_evt1_src_sel(&self) -> PERI_GT_EVT1_SRC_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT1_SRC_SELR { bits }
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn peri_gt_evt1_pulse_dur(&self) -> PERI_GT_EVT1_PULSE_DURR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT1_PULSE_DURR { bits }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn peri_gt_evt0_en(&self) -> PERI_GT_EVT0_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERI_GT_EVT0_ENR { bits }
    }
    #[doc = "Bits 4:6"]
    #[inline]
    pub fn peri_gt_evt0_src_sel(&self) -> PERI_GT_EVT0_SRC_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT0_SRC_SELR { bits }
    }
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn peri_gt_evt0_pulse_dur(&self) -> PERI_GT_EVT0_PULSE_DURR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_GT_EVT0_PULSE_DURR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn peri_gt_evt3_en(&mut self) -> _PERI_GT_EVT3_ENW {
        _PERI_GT_EVT3_ENW { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline]
    pub fn peri_gt_evt3_src_sel(&mut self) -> _PERI_GT_EVT3_SRC_SELW {
        _PERI_GT_EVT3_SRC_SELW { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn peri_gt_evt3_pulse_dur(&mut self) -> _PERI_GT_EVT3_PULSE_DURW {
        _PERI_GT_EVT3_PULSE_DURW { w: self }
    }
    #[doc = "Bit 23"]
    #[inline]
    pub fn peri_gt_evt2_en(&mut self) -> _PERI_GT_EVT2_ENW {
        _PERI_GT_EVT2_ENW { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline]
    pub fn peri_gt_evt2_src_sel(&mut self) -> _PERI_GT_EVT2_SRC_SELW {
        _PERI_GT_EVT2_SRC_SELW { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn peri_gt_evt2_pulse_dur(&mut self) -> _PERI_GT_EVT2_PULSE_DURW {
        _PERI_GT_EVT2_PULSE_DURW { w: self }
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn peri_gt_evt1_en(&mut self) -> _PERI_GT_EVT1_ENW {
        _PERI_GT_EVT1_ENW { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline]
    pub fn peri_gt_evt1_src_sel(&mut self) -> _PERI_GT_EVT1_SRC_SELW {
        _PERI_GT_EVT1_SRC_SELW { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn peri_gt_evt1_pulse_dur(&mut self) -> _PERI_GT_EVT1_PULSE_DURW {
        _PERI_GT_EVT1_PULSE_DURW { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn peri_gt_evt0_en(&mut self) -> _PERI_GT_EVT0_ENW {
        _PERI_GT_EVT0_ENW { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline]
    pub fn peri_gt_evt0_src_sel(&mut self) -> _PERI_GT_EVT0_SRC_SELW {
        _PERI_GT_EVT0_SRC_SELW { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn peri_gt_evt0_pulse_dur(&mut self) -> _PERI_GT_EVT0_PULSE_DURW {
        _PERI_GT_EVT0_PULSE_DURW { w: self }
    }
}
