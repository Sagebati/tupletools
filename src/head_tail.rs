pub trait HeadTail {
    type Head;
    type Tail;
    fn head_tail(self) -> (Self::Head, Self::Tail);
}

impl<T> HeadTail for (T,) {
    type Head = T;
    type Tail = ();
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), ())
    }
}

impl<T, T1> HeadTail for (T, T1) {
    type Head = T;
    type Tail = (T1,);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), (self.1,))
    }
}

impl<T, T1, T2> HeadTail for (T, T1, T2) {
    type Head = T;
    type Tail = (T1, T2);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), (self.1, self.2))
    }
}

impl<T, T1, T2, T3> HeadTail for (T, T1, T2, T3) {
    type Head = T;
    type Tail = (T1, T2, T3);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), (self.1, self.2, self.3))
    }
}

impl<T, T1, T2, T3, T4> HeadTail for (T, T1, T2, T3, T4) {
    type Head = T;
    type Tail = (T1, T2, T3, T4);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), (self.1, self.2, self.3, self.4))
    }
}

impl<T, T1, T2, T3, T4, T5> HeadTail for (T, T1, T2, T3, T4, T5) {
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), (self.1, self.2, self.3, self.4, self.5))
    }
}

impl<T, T1, T2, T3, T4, T5, T6> HeadTail for (T, T1, T2, T3, T4, T5, T6) {
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        ((self.0), (self.1, self.2, self.3, self.4, self.5, self.6))
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7> HeadTail for (T, T1, T2, T3, T4, T5, T6, T7) {
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (self.1, self.2, self.3, self.4, self.5, self.6, self.7),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8> HeadTail for (T, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9> HeadTail for (T, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> HeadTail
    for (T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> HeadTail
    for (T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
                self.11,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> HeadTail
    for (T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
                self.11, self.12,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> HeadTail
    for (T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
                self.11, self.12, self.13,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> HeadTail
    for (
        T,
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
    )
{
    type Head = T;
    type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
                self.11, self.12, self.13, self.14,
            ),
        )
    }
}

impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> HeadTail
    for (
        T,
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
    type Head = T;
    type Tail = (
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
    fn head_tail(self) -> (Self::Head, Self::Tail) {
        (
            (self.0),
            (
                self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
                self.11, self.12, self.13, self.14, self.15,
            ),
        )
    }
}
