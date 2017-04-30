//! Types and traits for uniform variables

use std::ops::{Add, Mul};

/// Describes a type that can be interpolated with barycentric coordinates.
///
/// This is required for any rasterization to occur.
///
/// See [This document](https://classes.soe.ucsc.edu/cmps160/Fall10/resources/barycentricInterpolation.pdf) for more information
pub trait Barycentric {
    fn interpolate(u: f32, x1: &Self, v: f32, x2: &Self, w: f32, x3: &Self) -> Self;
}

#[inline(always)]
pub fn barycentric_interpolate<T>(u: f32, ux: T, v: f32, vx: T, w: f32, wx: T) -> T
    where T: Add<Output=T> + Add<f32, Output=T> + Mul<f32, Output=T> {
    ux * u + vx * v + wx * w
}

impl Barycentric for f32 {
    #[inline(always)]
    fn interpolate(u: f32, ux: &Self, v: f32, vx: &Self, w: f32, wx: &Self) -> Self {
        barycentric_interpolate(u, *ux, v, *vx, w, *wx)
    }
}

#[macro_export]
macro_rules! declare_uniforms {
    ($(#[$($struct_attrs:tt)*])* pub struct $name:ident {
        $($(#[$($field_attrs:tt)*])* pub $field:ident: $t:ty,)*
    }) => {
        $(#[$($struct_attrs)*])*
        pub struct $name {
            $(
                $(#[$($field_attrs)*])*
                pub $field: $t
            ),*
        }

        impl $crate::render::Barycentric for $name {
            fn interpolate(u: f32, ux: &Self, v: f32, vx: &Self, w: f32, wx: &Self) -> Self {
                $name {
                    $(
                        $field: $crate::render::Barycentric::interpolate(u, &ux.$field,
                                                                         v, &vx.$field,
                                                                         w, &wx.$field)
                    ),*
                }
            }
        }
    };
}

pub mod nalgebra_uniforms {
    use super::Barycentric;

    use nalgebra::Matrix;
    use nalgebra::dimension::{U1, U2, U3, U4, U5, U6};
    use nalgebra::allocator::OwnedAllocator;
    use nalgebra::storage::OwnedStorage;

    // Format of this was taken from nalgebra/core/construction.rs
    macro_rules! nalgebra_matrix_uniforms {
        ($($R: ty, $C: ty, $($args: ident:($irow: expr,$icol: expr)),*);* $(;)*) => {$(
            impl<S> Barycentric for Matrix<f32, $R, $C, S>
                where S: OwnedStorage<f32, $R, $C>,
                      S::Alloc: OwnedAllocator<f32, $R, $C, S> {
                #[inline]
                fn interpolate(u: f32, ux: &Self, v: f32, vx: &Self, w: f32, wx: &Self) -> Self {
                    unsafe {
                        let mut res = Self::new_uninitialized();

                        $(
                            *res.get_unchecked_mut($irow, $icol) = *ux.get_unchecked($irow, $icol) * u +
                                                                   *vx.get_unchecked($irow, $icol) * v +
                                                                   *wx.get_unchecked($irow, $icol) * w;
                        )*

                        res
                    }
                }
            }
        )*}
    }

    nalgebra_matrix_uniforms!(
        /*
         * Square matrices 1 .. 6.
         */
        U2, U2, m11:(0,0), m12:(0,1),
                m21:(1,0), m22:(1,1);
        U3, U3, m11:(0,0), m12:(0,1), m13:(0,2),
                m21:(1,0), m22:(1,1), m23:(1,2),
                m31:(2,0), m32:(2,1), m33:(2,2);
        U4, U4, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3);
        U5, U5, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3), m45:(3,4),
                m51:(4,0), m52:(4,1), m53:(4,2), m54:(4,3), m55:(4,4);
        U6, U6, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4), m16:(0,5),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4), m26:(1,5),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4), m36:(2,5),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3), m45:(3,4), m46:(3,5),
                m51:(4,0), m52:(4,1), m53:(4,2), m54:(4,3), m55:(4,4), m56:(4,5),
                m61:(5,0), m62:(5,1), m63:(5,2), m64:(5,3), m65:(5,4), m66:(5,5);

        /*
         * Rectangular matrices with 2 rows.
         */
        U2, U3, m11:(0,0), m12:(0,1), m13:(0,2),
                m21:(1,0), m22:(1,1), m23:(1,2);
        U2, U4, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3);
        U2, U5, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4);
        U2, U6, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4), m16:(0,5),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4), m26:(1,5);

        /*
         * Rectangular matrices with 3 rows.
         */
        U3, U2, m11:(0,0), m12:(0,1),
                m21:(1,0), m22:(1,1),
                m31:(2,0), m32:(2,1);
        U3, U4, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3);
        U3, U5, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4);
        U3, U6, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4), m16:(0,5),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4), m26:(1,5),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4), m36:(2,5);

        /*
         * Rectangular matrices with 4 rows.
         */
        U4, U2, m11:(0,0), m12:(0,1),
                m21:(1,0), m22:(1,1),
                m31:(2,0), m32:(2,1),
                m41:(3,0), m42:(3,1);
        U4, U3, m11:(0,0), m12:(0,1), m13:(0,2),
                m21:(1,0), m22:(1,1), m23:(1,2),
                m31:(2,0), m32:(2,1), m33:(2,2),
                m41:(3,0), m42:(3,1), m43:(3,2);
        U4, U5, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3), m45:(3,4);
        U4, U6, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4), m16:(0,5),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4), m26:(1,5),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4), m36:(2,5),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3), m45:(3,4), m46:(3,5);

        /*
         * Rectangular matrices with 5 rows.
         */
        U5, U2, m11:(0,0), m12:(0,1),
                m21:(1,0), m22:(1,1),
                m31:(2,0), m32:(2,1),
                m41:(3,0), m42:(3,1),
                m51:(4,0), m52:(4,1);
        U5, U3, m11:(0,0), m12:(0,1), m13:(0,2),
                m21:(1,0), m22:(1,1), m23:(1,2),
                m31:(2,0), m32:(2,1), m33:(2,2),
                m41:(3,0), m42:(3,1), m43:(3,2),
                m51:(4,0), m52:(4,1), m53:(4,2);
        U5, U4, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3),
                m51:(4,0), m52:(4,1), m53:(4,2), m54:(4,3);
        U5, U6, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4), m16:(0,5),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4), m26:(1,5),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4), m36:(2,5),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3), m45:(3,4), m46:(3,5),
                m51:(4,0), m52:(4,1), m53:(4,2), m54:(4,3), m55:(4,4), m56:(4,5);

        /*
         * Rectangular matrices with 6 rows.
         */
        U6, U2, m11:(0,0), m12:(0,1),
                m21:(1,0), m22:(1,1),
                m31:(2,0), m32:(2,1),
                m41:(3,0), m42:(3,1),
                m51:(4,0), m52:(4,1),
                m61:(5,0), m62:(5,1);
        U6, U3, m11:(0,0), m12:(0,1), m13:(0,2),
                m21:(1,0), m22:(1,1), m23:(1,2),
                m31:(2,0), m32:(2,1), m33:(2,2),
                m41:(3,0), m42:(3,1), m43:(3,2),
                m51:(4,0), m52:(4,1), m53:(4,2),
                m61:(5,0), m62:(5,1), m63:(5,2);
        U6, U4, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3),
                m51:(4,0), m52:(4,1), m53:(4,2), m54:(4,3),
                m61:(5,0), m62:(5,1), m63:(5,2), m64:(5,3);
        U6, U5, m11:(0,0), m12:(0,1), m13:(0,2), m14:(0,3), m15:(0,4),
                m21:(1,0), m22:(1,1), m23:(1,2), m24:(1,3), m25:(1,4),
                m31:(2,0), m32:(2,1), m33:(2,2), m34:(2,3), m35:(2,4),
                m41:(3,0), m42:(3,1), m43:(3,2), m44:(3,3), m45:(3,4),
                m51:(4,0), m52:(4,1), m53:(4,2), m54:(4,3), m55:(4,4),
                m61:(5,0), m62:(5,1), m63:(5,2), m64:(5,3), m65:(5,4);

        /*
         * Row vectors 1 .. 6.
         */
        U1, U1, x:(0,0);
        U1, U2, x:(0,0), y:(0,1);
        U1, U3, x:(0,0), y:(0,1), z:(0,2);
        U1, U4, x:(0,0), y:(0,1), z:(0,2), w:(0,3);
        U1, U5, x:(0,0), y:(0,1), z:(0,2), w:(0,3), a:(0,4);
        U1, U6, x:(0,0), y:(0,1), z:(0,2), w:(0,3), a:(0,4), b:(0,5);

        /*
         * Column vectors 1 .. 6.
         */
        U2, U1, x:(0,0), y:(1,0);
        U3, U1, x:(0,0), y:(1,0), z:(2,0);
        U4, U1, x:(0,0), y:(1,0), z:(2,0), w:(3,0);
        U5, U1, x:(0,0), y:(1,0), z:(2,0), w:(3,0), a:(4,0);
        U6, U1, x:(0,0), y:(1,0), z:(2,0), w:(3,0), a:(4,0), b:(5,0);
    );
}