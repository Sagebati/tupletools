pub trait Snd {
    type Ret;
    fn snd(&self) -> &Self::Ret;
    fn into_snd(self) -> Self::Ret;
}


macro_rules! impl_snd {
    (
        $( $t:tt ),+ ;$r:ty
    ) => {
        impl < $($t),+ > Snd for ($($t),+) {
            type Ret = $r;
            fn snd(&self) -> &Self::Ret {
                &self.1
            }

            fn into_snd(self) -> Self::Ret {
                self.1
            }
        }
    };
}


impl_snd!(T0,T1;T1);
impl_snd!(T0,T1, T2;T1);
impl_snd!(T0,T1, T2 , T3;T1);
impl_snd!(T0,T1, T2, T3, T4;T1);
impl_snd!(T0,T1, T2, T3, T4, T5;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15;T1);
impl_snd!(T0,T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16;T1);



