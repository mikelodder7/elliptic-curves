use crate::*;
use crate::{
    decaf::DecafPoint,
    field::{ConstMontyType, FieldElement},
    Scalar,
};

pub const DECAF_BASEPOINT: DecafPoint = DecafPoint(curve::twedwards::extended::ExtendedPoint {
    X: FieldElement(ConstMontyType::new(&U448::from_be_hex("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa955555555555555555555555555555555555555555555555555555555"))),
    Y: FieldElement(ConstMontyType::new(&U448::from_be_hex("ae05e9634ad7048db359d6205086c2b0036ed7a035884dd7b7e36d728ad8c4b80d6565833a2a3098bbbcb2bed1cda06bdaeafbcdea9386ed"))),
    Z: FieldElement(ConstMontyType::new(&U448::from_u64(1))),
    T: FieldElement(ConstMontyType::new(&U448::from_be_hex("696d84643374bace9d70983a12aa9d461da74d2d5c35e8d97ba72c3aba4450a5d29274229bd22c1d5e3a6474ee4ffb0e7a9e200a28eee402"))),
});

/// `BASEPOINT_ORDER` is the order of the Ed448 basepoint, i.e.,
/// $$
/// \ell = 2^\{446\} + 0x8335dc163bb124b65129c96fde933d8d723a70aadc873d6d54a7bb0d.
/// $$
pub const BASEPOINT_ORDER: Scalar = Scalar(ORDER);
