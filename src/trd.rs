pub trait Trd {
    type Ret;
    fn trd(&self) -> &Self::Ret;
    fn into_trd(self) -> Self::Ret;
}




macro_rules! impl_trd {
    (
        $( $t:tt ),+ ;$r:ty
    ) => {
        impl < $($t),+ > Trd for ($($t),+) {
            type Ret = $r;
            fn trd(&self) -> &Self::Ret {
                &self.2
            }

            fn into_trd(self) -> Self::Ret {
                self.2
            }
        }
    };
}


impl_trd!(T0,T1, T2;T2);
impl_trd!(T0,T1, T2 , T3;T2);
impl_trd!(T0,T1, T2, T3, T4;T2);
impl_trd!(T0,T1, T2, T3, T4, T5;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15;T2);
impl_trd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16;T2);


