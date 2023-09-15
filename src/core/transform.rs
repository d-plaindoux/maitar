use crate::core::hkp::HKP;

pub trait Transform<'a, A>: HKP<'a> {
    type This: HKP<'a>;

    fn from_hkp<B>(a: <Self::This as HKP<'a>>::T<B>) -> Self::T<B>;

    fn from_self<B>(a: Self::T<B>) -> <Self::This as HKP<'a>>::T<B>;

    fn to_hkp(self) -> <Self::This as HKP<'a>>::T<A>;
}
