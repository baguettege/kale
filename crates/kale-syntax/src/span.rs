use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct Spanned<T> {
    span: Span,
    inner: T,
}

impl Span {
    fn from_parts(start: usize, end: usize) -> Self {
        debug_assert!(start <= end, "`Span` start ({start}) must be <= end ({end})");
        Self { start, end }
    }

    pub fn new(start: usize, end: usize) -> Self {
        Self::from_parts(
            usize::min(start, end),
            usize::max(start, end),
        )
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self::from_parts(
            usize::min(self.start, other.start),
            usize::max(self.end, other.end),
        )
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn into_parts(self) -> (usize, usize) {
        (self.start, self.end)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl<T> Spanned<T> {
    pub fn new(span: Span, inner: T) -> Self {
        Self { span, inner }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T: Clone> Clone for Spanned<T> {
    fn clone(&self) -> Self {
        Self::new(self.span, self.inner.clone())
    }
}

impl<T: Copy> Copy for Spanned<T> {}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
