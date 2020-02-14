pub trait Fst {
    type Ret;
    fn fst(&self) -> &Self::Ret;
    fn into_fst(self) -> Self::Ret;
}

macro_rules! impl_fst {
    (
        $( $t:tt ),+ ;$r:ty
    ) => {
        impl < $($t),+ > Fst for ($($t),+) {
            type Ret = $r;
            fn fst(&self) -> &Self::Ret {
                &self.0
            }

            fn into_fst(self) -> Self::Ret {
                self.0
            }
        }
    };
}

impl<T> Fst for (T,) {
    type Ret = T;

    fn fst(&self) -> &Self::Ret {
        &self.0
    }

    fn into_fst(self) -> Self::Ret {
        self.0
    }
}

impl_fst!(T0,T1;T0);
impl_fst!(T0,T1, T2;T0);
impl_fst!(T0,T1, T2 , T3;T0);
impl_fst!(T0,T1, T2, T3, T4;T0);
impl_fst!(T0,T1, T2, T3, T4, T5;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15;T0);
impl_fst!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16;T0);
