use crate::dsl::Component;

pub trait ExtensionClause {
    type S;
    type E;

    fn component(&self) -> &Option<Component<Self::S, Self::E>>;
    fn component_mut(&mut self) -> &mut Option<Component<Self::S, Self::E>>;

    fn ext(&self) -> Option<&Self::E> {
        self.component().as_ref().and_then(|c| c.ext())
    }

    fn ext_mut(&mut self) -> Option<&mut Self::E> {
        self.component_mut().as_mut().and_then(|c| c.ext_mut())
    }

    fn ext_mut_or_default(&mut self) -> Option<&mut Self::E>
    where
        Self::E: Default,
    {
        let opt = self.component_mut();
        if opt.is_none() {
            *opt = Some(Component::Extension(Self::E::default()));
        }
        opt.as_mut().and_then(|c| c.ext_mut())
    }
}
