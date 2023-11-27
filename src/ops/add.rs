use core::ops::Add;

pub trait Addition {
    type Output;
    fn add(self, other: Self) -> Self::Output;
}

impl<T1: Add<Output = T1>> Addition for (T1,) {
    type Output = (T1,);
    fn add(self, other: Self) -> Self::Output {
        (self.0 + other.0,)
    }
}

impl<T1: Add<Output = T1>, T2: Add<Output = T2>> Addition for (T1, T2) {
    type Output = (T1, T2);
    fn add(self, other: Self) -> Self::Output {
        (self.0 + other.0, self.1 + other.1)
    }
}

impl<T1: Add<Output = T1>, T2: Add<Output = T2>, T3: Add<Output = T3>> Addition for (T1, T2, T3) {
    type Output = (T1, T2, T3);
    fn add(self, other: Self) -> Self::Output {
        (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<T1: Add<Output = T1>, T2: Add<Output = T2>, T3: Add<Output = T3>, T4: Add<Output = T4>>
    Addition for (T1, T2, T3, T4)
{
    type Output = (T1, T2, T3, T4);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
    > Addition for (T1, T2, T3, T4, T5)
{
    type Output = (T1, T2, T3, T4, T5);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
    > Addition for (T1, T2, T3, T4, T5, T6)
{
    type Output = (T1, T2, T3, T4, T5, T6);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
        T11: Add<Output = T11>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
            self.10 + other.10,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
        T11: Add<Output = T11>,
        T12: Add<Output = T12>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
            self.10 + other.10,
            self.11 + other.11,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
        T11: Add<Output = T11>,
        T12: Add<Output = T12>,
        T13: Add<Output = T13>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
            self.10 + other.10,
            self.11 + other.11,
            self.12 + other.12,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
        T11: Add<Output = T11>,
        T12: Add<Output = T12>,
        T13: Add<Output = T13>,
        T14: Add<Output = T14>,
    > Addition for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
            self.10 + other.10,
            self.11 + other.11,
            self.12 + other.12,
            self.13 + other.13,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
        T11: Add<Output = T11>,
        T12: Add<Output = T12>,
        T13: Add<Output = T13>,
        T14: Add<Output = T14>,
        T15: Add<Output = T15>,
    > Addition
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
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
            self.10 + other.10,
            self.11 + other.11,
            self.12 + other.12,
            self.13 + other.13,
            self.14 + other.14,
        )
    }
}

impl<
        T1: Add<Output = T1>,
        T2: Add<Output = T2>,
        T3: Add<Output = T3>,
        T4: Add<Output = T4>,
        T5: Add<Output = T5>,
        T6: Add<Output = T6>,
        T7: Add<Output = T7>,
        T8: Add<Output = T8>,
        T9: Add<Output = T9>,
        T10: Add<Output = T10>,
        T11: Add<Output = T11>,
        T12: Add<Output = T12>,
        T13: Add<Output = T13>,
        T14: Add<Output = T14>,
        T15: Add<Output = T15>,
        T16: Add<Output = T16>,
    > Addition
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
    fn add(self, other: Self) -> Self::Output {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            self.4 + other.4,
            self.5 + other.5,
            self.6 + other.6,
            self.7 + other.7,
            self.8 + other.8,
            self.9 + other.9,
            self.10 + other.10,
            self.11 + other.11,
            self.12 + other.12,
            self.13 + other.13,
            self.14 + other.14,
            self.15 + other.15,
        )
    }
}
