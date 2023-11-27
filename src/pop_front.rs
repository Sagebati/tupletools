pub trait PopFront {
    type Output;
    fn pop_front(self) -> Self::Output;
}

impl<T1> PopFront for (T1,) {
    type Output = ();
    fn pop_front(self) -> Self::Output {
        ()
    }
}

impl<T1, T2> PopFront for (T1, T2) {
    type Output = (T2,);
    fn pop_front(self) -> Self::Output {
        (self.1,)
    }
}

impl<T1, T2, T3> PopFront for (T1, T2, T3) {
    type Output = (T2, T3);
    fn pop_front(self) -> Self::Output {
        (self.1, self.2)
    }
}

impl<T1, T2, T3, T4> PopFront for (T1, T2, T3, T4) {
    type Output = (T2, T3, T4);
    fn pop_front(self) -> Self::Output {
        (self.1, self.2, self.3)
    }
}

impl<T1, T2, T3, T4, T5> PopFront for (T1, T2, T3, T4, T5) {
    type Output = (T2, T3, T4, T5);
    fn pop_front(self) -> Self::Output {
        (self.1, self.2, self.3, self.4)
    }
}

impl<T1, T2, T3, T4, T5, T6> PopFront for (T1, T2, T3, T4, T5, T6) {
    type Output = (T2, T3, T4, T5, T6);
    fn pop_front(self) -> Self::Output {
        (self.1, self.2, self.3, self.4, self.5)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> PopFront for (T1, T2, T3, T4, T5, T6, T7) {
    type Output = (T2, T3, T4, T5, T6, T7);
    fn pop_front(self) -> Self::Output {
        (self.1, self.2, self.3, self.4, self.5, self.6)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> PopFront for (T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (T2, T3, T4, T5, T6, T7, T8);
    fn pop_front(self) -> Self::Output {
        (self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> PopFront for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> PopFront
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> PopFront
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> PopFront
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
            self.11,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> PopFront
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
            self.11, self.12,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> PopFront
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
{
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
            self.11, self.12, self.13,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> PopFront
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
    type Output = (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
            self.11, self.12, self.13, self.14,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> PopFront
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
    fn pop_front(self) -> Self::Output {
        (
            self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
            self.11, self.12, self.13, self.14, self.15,
        )
    }
}
