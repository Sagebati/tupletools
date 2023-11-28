use crate::head_tail::HeadTail;

pub trait PopFront {
    type Output;
    fn pop_front(self) -> Self::Output;
}

impl<T> PopFront for T
where
    T: HeadTail,
{
    type Output = T::Tail;

    fn pop_front(self) -> Self::Output {
        self.head_tail().1
    }
}
