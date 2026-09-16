pub struct Component<S, E> {
    inner: ComponentKind<S, E>,
}

enum ComponentKind<S, E> {
    Standard(S),
    Extension(E),
}

pub trait WithComponent {
    type Standard;
    type Extension;

    fn from_std(std: Self::Standard) -> Self;
    fn from_ext(ext: Self::Extension) -> Self;
}

impl<S, E> Component<S, E> {
    pub fn standard(s: S) -> Self {
        Self { inner: ComponentKind::Standard(s) }
    }

    pub fn extension(e: E) -> Self {
        Self { inner: ComponentKind::Extension(e) }
    }

    pub fn ext(&self) -> Option<&E> {
        match &self.inner {
            ComponentKind::Extension(e) => Some(e),
            _ => None,
        }
    }

    pub fn ext_mut(&mut self) -> Option<&mut E> {
        match &mut self.inner {
            ComponentKind::Extension(e) => Some(e),
            _ => None,
        }
    }
}
