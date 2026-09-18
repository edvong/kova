pub trait ExtensionClause<E> {
    /// Core contract: Requires the implementer to provide a shared reference
    /// to the underlying optional container.
    fn ext_opt(&self) -> &Option<E>;

    /// Core contract: Requires the implementer to provide a mutable reference
    /// to the underlying optional container.
    fn ext_opt_mut(&mut self) -> &mut Option<E>;

    /// Returns a shared reference to the extension, if it is present.
    fn ext(&self) -> Option<&E> {
        self.ext_opt().as_ref()
    }

    /// Returns a mutable reference to the extension, if it is present,
    /// without performing any initialization.
    fn ext_mut(&mut self) -> Option<&mut E> {
        self.ext_opt_mut().as_mut()
    }

    /// Returns a mutable reference to the extension.
    /// If the extension is not currently present, it is initialized in place
    /// using the provided closure.
    /// This method is guaranteed to return a valid mutable reference.
    fn ext_mut_or_with<F>(&mut self, f: F) -> &mut E
    where
        F: FnOnce() -> E, {
        // Delegates directly to `Option::get_or_insert_with` for a zero-cost abstraction.
        self.ext_opt_mut().get_or_insert_with(f)
    }

    /// Unconditionally sets or replaces the extension with the provided value.
    /// This is optimal for direct overwrites where lazy initialization is unnecessary.
    fn set(&mut self, ext: E) {
        *self.ext_opt_mut() = Some(ext);
    }
}
