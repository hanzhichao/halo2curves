macro_rules! impl_add_binop_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> ::core::ops::Add<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: &'b $rhs) -> $output {
                &self + rhs
            }
        }

        impl<'a> ::core::ops::Add<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                self + &rhs
            }
        }

        impl ::core::ops::Add<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                &self + &rhs
            }
        }
    };
}

macro_rules! impl_sub_binop_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> ::core::ops::Sub<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: &'b $rhs) -> $output {
                &self - rhs
            }
        }

        impl<'a> ::core::ops::Sub<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs) -> $output {
                self - &rhs
            }
        }

        impl ::core::ops::Sub<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs) -> $output {
                &self - &rhs
            }
        }
    };
}

macro_rules! impl_binops_additive_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl_add_binop_specify_output!($lhs, $rhs, $output);
        impl_sub_binop_specify_output!($lhs, $rhs, $output);
    };
}

macro_rules! impl_binops_multiplicative_mixed {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> ::core::ops::Mul<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: &'b $rhs) -> $output {
                &self * rhs
            }
        }

        impl<'a> ::core::ops::Mul<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs) -> $output {
                self * &rhs
            }
        }

        impl ::core::ops::Mul<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs) -> $output {
                &self * &rhs
            }
        }
    };
}

macro_rules! impl_binops_additive {
    ($lhs:ident, $rhs:ident) => {
        impl_binops_additive_specify_output!($lhs, $rhs, $lhs);

        impl ::core::ops::SubAssign<$rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = &*self - &rhs;
            }
        }

        impl ::core::ops::AddAssign<$rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                *self = &*self + &rhs;
            }
        }

        impl<'b> ::core::ops::SubAssign<&'b $rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self - rhs;
            }
        }

        impl<'b> ::core::ops::AddAssign<&'b $rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self + rhs;
            }
        }
    };
}

macro_rules! impl_binops_multiplicative {
    ($lhs:ident, $rhs:ident) => {
        impl_binops_multiplicative_mixed!($lhs, $rhs, $lhs);

        impl ::core::ops::MulAssign<$rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = &*self * &rhs;
            }
        }

        impl<'b> ::core::ops::MulAssign<&'b $rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self * rhs;
            }
        }
    };
}

macro_rules! new_curve_impl {
    (($($privacy:tt)*), $name:ident, $name_affine:ident, $base:ident, $scalar:ident, $size:expr,
     $curve_id:literal) => {
        /// Represents a point in the projective coordinate space.
        #[derive(Copy, Clone, Debug)]
        $($privacy)* struct $name {
            x: $base,
            y: $base,
            z: $base,
        }

        /// Represents a point in the affine coordinate space (or the point at
        /// infinity).
        #[derive(Copy, Clone)]
        $($privacy)* struct $name_affine {
            x: $base,
            y: $base,
            infinity: Choice,
        }

        impl std::fmt::Debug for $name_affine {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.infinity.into() {
                    write!(f, "Infinity")
                } else {
                    write!(f, "({:?}, {:?})", self.x, self.y)
                }
            }
        }

        impl group::Group for $name {
            type Scalar = $scalar;

            fn random(mut rng: impl RngCore) -> Self {
                loop {
                    let x = $base::random(&mut rng);
                    let ysign = (rng.next_u32() % 2) as u8;

                    let x3 = x.square() * x;
                    let y = (x3 + $name::curve_constant_b()).sqrt();
                    if let Some(y) = Option::<$base>::from(y) {
                        let sign = y.to_bytes()[0] & 1;
                        let y = if ysign ^ sign == 0 { y } else { -y };

                        let p = $name_affine {
                            x,
                            y,
                            infinity: Choice::from(0u8),
                        };
                        break p.to_curve();
                    }
                }
            }

            impl_projective_curve_specific!($name, $base);

            fn generator() -> Self {
                $name::generator()
            }

            fn identity() -> Self {
                Self {
                    x: $base::zero(),
                    y: $base::zero(),
                    z: $base::zero(),
                }
            }

            fn is_identity(&self) -> Choice {
                self.z.ct_is_zero()
            }
        }

        impl CurveExt for $name {
            type ScalarExt = $scalar;
            type Base = $base;
            type AffineExt = $name_affine;

            const CURVE_ID: &'static str = $curve_id;

            // it is about hash to curve
            // impl_projective_curve_ext!($name, $iso, $base, $curve_type);

            fn b() -> Self::Base {
                $name::curve_constant_b()
            }

            fn new_jacobian(x: Self::Base, y: Self::Base, z: Self::Base) -> CtOption<Self> {
                let p = $name { x, y, z };
                CtOption::new(p, p.is_on_curve())
            }

            fn jacobian_coordinates(&self) -> ($base, $base, $base) {
               (self.x, self.y, self.z)
            }

            fn is_on_curve(&self) -> Choice {

                let z2 = self.z.square();
                let z4 = z2.square();
                let z6 = z4 * z2;
                (self.y.square() - (self.x.square() * z4) * self.x)
                    .ct_eq(&(z6 * $name::curve_constant_b()))
                    | self.z.ct_is_zero()
            }
        }

        impl group::Curve for $name {
            type AffineRepr = $name_affine;

            fn batch_normalize(p: &[Self], q: &mut [Self::AffineRepr]) {
                assert_eq!(p.len(), q.len());

                let mut acc = $base::one();
                for (p, q) in p.iter().zip(q.iter_mut()) {
                    // We use the `x` field of $name_affine to store the product
                    // of previous z-coordinates seen.
                    q.x = acc;

                    // We will end up skipping all identities in p
                    acc = $base::conditional_select(&(acc * p.z), &acc, p.is_identity());
                }

                // This is the inverse, as all z-coordinates are nonzero and the ones
                // that are not are skipped.
                acc = acc.invert().unwrap();

                for (p, q) in p.iter().rev().zip(q.iter_mut().rev()) {
                    let skip = p.is_identity();

                    // Compute tmp = 1/z
                    let tmp = q.x * acc;

                    // Cancel out z-coordinate in denominator of `acc`
                    acc = $base::conditional_select(&(acc * p.z), &acc, skip);

                    // Set the coordinates to the correct value
                    let tmp2 = tmp.square();
                    let tmp3 = tmp2 * tmp;

                    q.x = p.x * tmp2;
                    q.y = p.y * tmp3;
                    q.infinity = Choice::from(0u8);

                    *q = $name_affine::conditional_select(&q, &$name_affine::identity(), skip);
                }
            }

            fn to_affine(&self) -> Self::AffineRepr {
                let zinv = self.z.invert().unwrap_or($base::zero());
                let zinv2 = zinv.square();
                let x = self.x * zinv2;
                let zinv3 = zinv2 * zinv;
                let y = self.y * zinv3;

                let tmp = $name_affine {
                    x,
                    y,
                    infinity: Choice::from(0u8),
                };

                $name_affine::conditional_select(&tmp, &$name_affine::identity(), zinv.ct_is_zero())
            }
        }

        impl PrimeGroup for $name {}

        impl CofactorGroup for $name {
            type Subgroup = $name;

            fn clear_cofactor(&self) -> Self {
                // This is a prime-order group, with a cofactor of 1.
                *self
            }

            fn into_subgroup(self) -> CtOption<Self::Subgroup> {
                // Nothing to do here.
                CtOption::new(self, 1.into())
            }

            fn is_torsion_free(&self) -> Choice {
                // Shortcut: all points in a prime-order group are torsion free.
                1.into()
            }
        }

        impl PrimeCurve for $name {
            type Affine = $name_affine;
        }

        impl CofactorCurve for $name {
            type Affine = $name_affine;
        }

        // impl GroupEncoding for $name {
        //     type Repr = [u8; 32];

        //     fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        //         $name_affine::from_bytes(bytes).map(Self::from)
        //     }

        //     fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        //         // We can't avoid curve checks when parsing a compressed encoding.
        //         $name_affine::from_bytes(bytes).map(Self::from)
        //     }

        //     fn to_bytes(&self) -> Self::Repr {
        //         $name_affine::from(self).to_bytes()
        //     }
        // }

        impl<'a> From<&'a $name_affine> for $name {
            fn from(p: &'a $name_affine) -> $name {
                p.to_curve()
            }
        }

        impl From<$name_affine> for $name {
            fn from(p: $name_affine) -> $name {
                p.to_curve()
            }
        }

        impl Default for $name {
            fn default() -> $name {
                $name::identity()
            }
        }

        impl ConstantTimeEq for $name {
            fn ct_eq(&self, other: &Self) -> Choice {
                // Is (xz^2, yz^3, z) equal to (x'z'^2, yz'^3, z') when converted to affine?

                let z = other.z.square();
                let x1 = self.x * z;
                let z = z * other.z;
                let y1 = self.y * z;
                let z = self.z.square();
                let x2 = other.x * z;
                let z = z * self.z;
                let y2 = other.y * z;

                let self_is_zero = self.is_identity();
                let other_is_zero = other.is_identity();

                (self_is_zero & other_is_zero) // Both point at infinity
                            | ((!self_is_zero) & (!other_is_zero) & x1.ct_eq(&x2) & y1.ct_eq(&y2))
                // Neither point at infinity, coordinates are the same
            }

        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.ct_eq(other).into()
            }
        }

        impl cmp::Eq for $name {}

        impl ConditionallySelectable for $name {
            fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
                $name {
                    x: $base::conditional_select(&a.x, &b.x, choice),
                    y: $base::conditional_select(&a.y, &b.y, choice),
                    z: $base::conditional_select(&a.z, &b.z, choice),
                }
            }
        }

        impl<'a> Neg for &'a $name {
            type Output = $name;

            fn neg(self) -> $name {
                $name {
                    x: self.x,
                    y: -self.y,
                    z: self.z,
                }
            }
        }

        impl Neg for $name {
            type Output = $name;

            fn neg(self) -> $name {
                -&self
            }
        }

        impl<T> Sum<T> for $name
        where
            T: core::borrow::Borrow<$name>,
        {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = T>,
            {
                iter.fold(Self::identity(), |acc, item| acc + item.borrow())
            }
        }

        impl<'a, 'b> Add<&'a $name> for &'b $name {
            type Output = $name;

            fn add(self, rhs: &'a $name) -> $name {
                if bool::from(self.is_identity()) {
                    *rhs
                } else if bool::from(rhs.is_identity()) {
                    *self
                } else {
                    let z1z1 = self.z.square();
                    let z2z2 = rhs.z.square();
                    let u1 = self.x * z2z2;
                    let u2 = rhs.x * z1z1;
                    let s1 = self.y * z2z2 * rhs.z;
                    let s2 = rhs.y * z1z1 * self.z;

                    if u1 == u2 {
                        if s1 == s2 {
                            self.double()
                        } else {
                            $name::identity()
                        }
                    } else {
                        let h = u2 - u1;
                        let i = (h + h).square();
                        let j = h * i;
                        let r = s2 - s1;
                        let r = r + r;
                        let v = u1 * i;
                        let x3 = r.square() - j - v - v;
                        let s1 = s1 * j;
                        let s1 = s1 + s1;
                        let y3 = r * (v - x3) - s1;
                        let z3 = (self.z + rhs.z).square() - z1z1 - z2z2;
                        let z3 = z3 * h;

                        $name {
                            x: x3, y: y3, z: z3
                        }
                    }
                }
            }
        }

        impl<'a, 'b> Add<&'a $name_affine> for &'b $name {
            type Output = $name;

            fn add(self, rhs: &'a $name_affine) -> $name {
                if bool::from(self.is_identity()) {
                    rhs.to_curve()
                } else if bool::from(rhs.is_identity()) {
                    *self
                } else {
                    let z1z1 = self.z.square();
                    let u2 = rhs.x * z1z1;
                    let s2 = rhs.y * z1z1 * self.z;

                    if self.x == u2 {
                        if self.y == s2 {
                            self.double()
                        } else {
                            $name::identity()
                        }
                    } else {
                        let h = u2 - self.x;
                        let hh = h.square();
                        let i = hh + hh;
                        let i = i + i;
                        let j = h * i;
                        let r = s2 - self.y;
                        let r = r + r;
                        let v = self.x * i;
                        let x3 = r.square() - j - v - v;
                        let j = self.y * j;
                        let j = j + j;
                        let y3 = r * (v - x3) - j;
                        let z3 = (self.z + h).square() - z1z1 - hh;

                        $name {
                            x: x3, y: y3, z: z3
                        }
                    }
                }
            }
        }

        impl<'a, 'b> Sub<&'a $name> for &'b $name {
            type Output = $name;

            fn sub(self, other: &'a $name) -> $name {
                self + (-other)
            }
        }

        impl<'a, 'b> Sub<&'a $name_affine> for &'b $name {
            type Output = $name;

            fn sub(self, other: &'a $name_affine) -> $name {
                self + (-other)
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl<'a, 'b> Mul<&'b $scalar> for &'a $name {
            type Output = $name;

            fn mul(self, other: &'b $scalar) -> Self::Output {
                // TODO: make this faster

                let mut acc = $name::identity();

                // This is a simple double-and-add implementation of point
                // multiplication, moving from most significant to least
                // significant bit of the scalar.
                //
                // NOTE: We skip the leading bit because it's always unset.
                for bit in other
                    .to_bytes()
                    .iter()
                    .rev()
                    .flat_map(|byte| (0..8).rev().map(move |i| Choice::from((byte >> i) & 1u8)))
                    .skip(1)
                {
                    acc = acc.double();
                    acc = $name::conditional_select(&acc, &(acc + self), bit);
                }

                acc
            }
        }

        impl<'a> Neg for &'a $name_affine {
            type Output = $name_affine;

            fn neg(self) -> $name_affine {
                $name_affine {
                    x: self.x,
                    y: -self.y,
                    infinity: self.infinity,
                }
            }
        }

        impl Neg for $name_affine {
            type Output = $name_affine;

            fn neg(self) -> $name_affine {
                -&self
            }
        }

        impl<'a, 'b> Add<&'a $name> for &'b $name_affine {
            type Output = $name;

            fn add(self, rhs: &'a $name) -> $name {
                rhs + self
            }
        }

        impl<'a, 'b> Add<&'a $name_affine> for &'b $name_affine {
            type Output = $name;

            fn add(self, rhs: &'a $name_affine) -> $name {
                if bool::from(self.is_identity()) {
                    rhs.to_curve()
                } else if bool::from(rhs.is_identity()) {
                    self.to_curve()
                } else {
                    if self.x == rhs.x {
                        if self.y == rhs.y {
                            self.to_curve().double()
                        } else {
                            $name::identity()
                        }
                    } else {
                        let h = rhs.x - self.x;
                        let hh = h.square();
                        let i = hh + hh;
                        let i = i + i;
                        let j = h * i;
                        let r = rhs.y - self.y;
                        let r = r + r;
                        let v = self.x * i;
                        let x3 = r.square() - j - v - v;
                        let j = self.y * j;
                        let j = j + j;
                        let y3 = r * (v - x3) - j;
                        let z3 = h + h;

                        $name {
                            x: x3, y: y3, z: z3
                        }
                    }
                }
            }
        }

        impl<'a, 'b> Sub<&'a $name_affine> for &'b $name_affine {
            type Output = $name;

            fn sub(self, other: &'a $name_affine) -> $name {
                self + (-other)
            }
        }

        impl<'a, 'b> Sub<&'a $name> for &'b $name_affine {
            type Output = $name;

            fn sub(self, other: &'a $name) -> $name {
                self + (-other)
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl<'a, 'b> Mul<&'b $scalar> for &'a $name_affine {
            type Output = $name;

            fn mul(self, other: &'b $scalar) -> Self::Output {
                // TODO: make this faster

                let mut acc = $name::identity();

                // This is a simple double-and-add implementation of point
                // multiplication, moving from most significant to least
                // significant bit of the scalar.
                //
                // NOTE: We skip the leading bit because it's always unset.
                for bit in other
                    .to_bytes()
                    .iter()
                    .rev()
                    .flat_map(|byte| (0..8).rev().map(move |i| Choice::from((byte >> i) & 1u8)))
                    .skip(1)
                {
                    acc = acc.double();
                    acc = $name::conditional_select(&acc, &(acc + self), bit);
                }

                acc
            }
        }

        impl PrimeCurveAffine for $name_affine {
            type Curve = $name;
            type Scalar = $scalar;


            fn generator() -> Self {
                $name_affine::generator()
            }

            fn identity() -> Self {
                Self {
                    x: $base::zero(),
                    y: $base::zero(),
                    infinity: Choice::from(1u8),
                }
            }

            fn is_identity(&self) -> Choice {
                self.infinity
            }

            fn to_curve(&self) -> Self::Curve {
                $name {
                    x: self.x,
                    y: self.y,
                    z: $base::conditional_select(&$base::one(), &$base::zero(), self.infinity),
                }
            }
        }

        impl group::cofactor::CofactorCurveAffine for $name_affine {
            type Curve = $name;
            type Scalar = $scalar;

            fn identity() -> Self {
                <Self as PrimeCurveAffine>::identity()
            }

            fn generator() -> Self {
                <Self as PrimeCurveAffine>::generator()
            }

            fn is_identity(&self) -> Choice {
                <Self as PrimeCurveAffine>::is_identity(self)
            }

            fn to_curve(&self) -> Self::Curve {
                <Self as PrimeCurveAffine>::to_curve(self)
            }
        }

        // impl GroupEncoding for $name_affine {
        //     type Repr = [u8; $size];

        //     fn from_bytes(bytes: &[u8; $size]) -> CtOption<Self> {
        //         let mut tmp = *bytes;
        //         let ysign = Choice::from(tmp[$size-1] >> 7);
        //         tmp[$size-1] &= 0b0111_1111;

        //         $base::from_bytes(&tmp).and_then(|x| {
        //             CtOption::new(Self::identity(), x.ct_is_zero() & (!ysign)).or_else(|| {
        //                 let x3 = x.square() * x;
        //                 (x3 + $name::curve_constant_b()).sqrt().and_then(|y| {
        //                     let sign = Choice::from(y.to_bytes()[0] & 1);

        //                     let y = $base::conditional_select(&y, &-y, ysign ^ sign);

        //                     CtOption::new(
        //                         $name_affine {
        //                             x,
        //                             y,
        //                             infinity: Choice::from(0u8),
        //                         },
        //                         Choice::from(1u8),
        //                     )
        //                 })
        //             })
        //         })
        //     }

        //     fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        //         // We can't avoid curve checks when parsing a compressed encoding.
        //         Self::from_bytes(bytes)
        //     }

        //     fn to_bytes(&self) -> [u8; $size] {
        //         // TODO: not constant time
        //         if bool::from(self.is_identity()) {
        //             [0; $size]
        //         } else {
        //             let (x, y) = (self.x, self.y);
        //             let sign = (y.to_bytes()[0] & 1) << 7;
        //             let mut xbytes = x.to_bytes();
        //             xbytes[$size-1] |= sign;
        //             xbytes
        //         }
        //     }
        // }

        impl CurveAffine for $name_affine {
            type ScalarExt = $scalar;
            type Base = $base;
            type CurveExt = $name;

            fn is_on_curve(&self) -> Choice {
                // y^2 - x^3 - ax ?= b
                (self.y.square() - self.x.square() * self.x).ct_eq(&$name::curve_constant_b())
                    | self.infinity
            }

            fn coordinates(&self) -> CtOption<Coordinates<Self>> {
                CtOption::new(Coordinates { x: self.x, y: self.y }, !self.is_identity())
            }

            fn from_xy(x: Self::Base, y: Self::Base) -> CtOption<Self> {
                let p = $name_affine {
                    x, y, infinity: 0u8.into()
                };
                CtOption::new(p, p.is_on_curve())
            }

            fn b() -> Self::Base {
                $name::curve_constant_b()
            }
        }

        impl Default for $name_affine {
            fn default() -> $name_affine {
                $name_affine::identity()
            }
        }

        impl<'a> From<&'a $name> for $name_affine {
            fn from(p: &'a $name) -> $name_affine {
                p.to_affine()
            }
        }

        impl From<$name> for $name_affine {
            fn from(p: $name) -> $name_affine {
                p.to_affine()
            }
        }

        impl ConstantTimeEq for $name_affine {
            fn ct_eq(&self, other: &Self) -> Choice {
                let z1 = self.infinity;
                let z2 = other.infinity;

                (z1 & z2) | ((!z1) & (!z2) & (self.x.ct_eq(&other.x)) & (self.y.ct_eq(&other.y)))
            }
        }

        impl PartialEq for $name_affine {
            fn eq(&self, other: &Self) -> bool {
                self.ct_eq(other).into()
            }
        }

        impl cmp::Eq for $name_affine {}

        impl ConditionallySelectable for $name_affine {
            fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
                $name_affine {
                    x: $base::conditional_select(&a.x, &b.x, choice),
                    y: $base::conditional_select(&a.y, &b.y, choice),
                    infinity: Choice::conditional_select(&a.infinity, &b.infinity, choice),
                }
            }
        }

        impl_binops_additive!($name, $name);
        impl_binops_additive!($name, $name_affine);
        impl_binops_additive_specify_output!($name_affine, $name_affine, $name);
        impl_binops_additive_specify_output!($name_affine, $name, $name);
        impl_binops_multiplicative!($name, $scalar);
        impl_binops_multiplicative_mixed!($name_affine, $scalar, $name);

        impl Group for $name {
            type Scalar = $scalar;

            fn group_zero() -> Self {
                Self::identity()
            }
            fn group_add(&mut self, rhs: &Self) {
                *self += *rhs;
            }
            fn group_sub(&mut self, rhs: &Self) {
                *self -= *rhs;
            }
            fn group_scale(&mut self, by: &Self::Scalar) {
                *self *= *by;
            }
        }
    };
  }

macro_rules! impl_projective_curve_specific {
    ($name:ident, $base:ident) => {
        fn double(&self) -> Self {
            // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
            //
            // There are no points of order 2.

            let a = self.x.square();
            let b = self.y.square();
            let c = b.square();
            let d = self.x + b;
            let d = d.square();
            let d = d - a - c;
            let d = d + d;
            let e = a + a + a;
            let f = e.square();
            let z3 = self.z * self.y;
            let z3 = z3 + z3;
            let x3 = f - (d + d);
            let c = c + c;
            let c = c + c;
            let c = c + c;
            let y3 = e * (d - x3) - c;

            let tmp = $name {
                x: x3,
                y: y3,
                z: z3,
            };

            $name::conditional_select(&tmp, &$name::identity(), self.is_identity())
        }
    };
}
