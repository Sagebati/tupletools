pub trait CastOneToOne<Too> {
    type Ret;
    fn cast(self) -> Self::Ret;
}


impl<T, Too> CastOneToOne<Too> for (T, T)
    where Too: From<T>
{
    type Ret = (Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
            )
        )
    }
}

impl<T, Too> CastOneToOne<Too> for (T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
                Too::from(self.10),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
                Too::from(self.10),
                Too::from(self.11),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
                Too::from(self.10),
                Too::from(self.11),
                Too::from(self.12),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
                Too::from(self.10),
                Too::from(self.11),
                Too::from(self.12),
                Too::from(self.13),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
                Too::from(self.10),
                Too::from(self.11),
                Too::from(self.12),
                Too::from(self.13),
                Too::from(self.14),
            )
        )
    }
}


impl<T, Too> CastOneToOne<Too> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
    where Too: From<T>
{
    type Ret = (Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too, Too);
    fn cast(self) -> Self::Ret {
        (
            (
                Too::from(self.0),
                Too::from(self.1),
                Too::from(self.2),
                Too::from(self.3),
                Too::from(self.4),
                Too::from(self.5),
                Too::from(self.6),
                Too::from(self.7),
                Too::from(self.8),
                Too::from(self.9),
                Too::from(self.10),
                Too::from(self.11),
                Too::from(self.12),
                Too::from(self.13),
                Too::from(self.14),
                Too::from(self.15),
            )
        )
    }
}

