pub enum Component<S, E> {
    Standard(S),
    Extension(E),
}

impl<S, E> Component<S, E> {
    pub fn ext(&self) -> Option<&E> {
        match self {
            Component::Extension(e) => Some(e),
            _ => None,
        }
    }

    pub fn ext_mut(&mut self) -> Option<&mut E> {
        match self {
            Component::Extension(e) => Some(e),
            _ => None,
        }
    }
}
