use core::ops::Div;

pub trait Division {
    type Output;
    fn div(self, other: Self) -> Self::Output;
}

impl<T1: Div<Output = T1>> Division for (T1,) {
    type Output = (T1,);
    fn div(self, other: Self) -> Self::Output {
        (self.0 / other.0,)
    }
}

impl<T1: Div<Output = T1>, T2: Div<Output = T2>> Division for (T1, T2) {
    type Output = (T1, T2);
    fn div(self, other: Self) -> Self::Output {
        (self.0 / other.0, self.1 / other.1)
    }
}

impl<T1: Div<Output = T1>, T2: Div<Output = T2>, T3: Div<Output = T3>> Division for (T1, T2, T3) {
    type Output = (T1, T2, T3);
    fn div(self, other: Self) -> Self::Output {
        (self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl<T1: Div<Output = T1>, T2: Div<Output = T2>, T3: Div<Output = T3>, T4: Div<Output = T4>>
    Division for (T1, T2, T3, T4)
{
    type Output = (T1, T2, T3, T4);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
    > Division for (T1, T2, T3, T4, T5)
{
    type Output = (T1, T2, T3, T4, T5);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
    > Division for (T1, T2, T3, T4, T5, T6)
{
    type Output = (T1, T2, T3, T4, T5, T6);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
    > Division for (T1, T2, T3, T4, T5, T6, T7)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
        T11: Div<Output = T11>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
        T11: Div<Output = T11>,
        T12: Div<Output = T12>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
        T11: Div<Output = T11>,
        T12: Div<Output = T12>,
        T13: Div<Output = T13>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
        T11: Div<Output = T11>,
        T12: Div<Output = T12>,
        T13: Div<Output = T13>,
        T14: Div<Output = T14>,
    > Division for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
            self.13 / other.13,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
        T11: Div<Output = T11>,
        T12: Div<Output = T12>,
        T13: Div<Output = T13>,
        T14: Div<Output = T14>,
        T15: Div<Output = T15>,
    > Division
    for (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    )
{
    type Output = (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    );
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
            self.13 / other.13,
            self.14 / other.14,
        )
    }
}

impl<
        T1: Div<Output = T1>,
        T2: Div<Output = T2>,
        T3: Div<Output = T3>,
        T4: Div<Output = T4>,
        T5: Div<Output = T5>,
        T6: Div<Output = T6>,
        T7: Div<Output = T7>,
        T8: Div<Output = T8>,
        T9: Div<Output = T9>,
        T10: Div<Output = T10>,
        T11: Div<Output = T11>,
        T12: Div<Output = T12>,
        T13: Div<Output = T13>,
        T14: Div<Output = T14>,
        T15: Div<Output = T15>,
        T16: Div<Output = T16>,
    > Division
    for (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    )
{
    type Output = (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    );
    fn div(self, other: Self) -> Self::Output {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
            self.13 / other.13,
            self.14 / other.14,
            self.15 / other.15,
        )
    }
}
