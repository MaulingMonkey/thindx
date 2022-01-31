mod sealed {
    /// ### ⚠️ Safety ⚠️
    /// By implementing `AsSafe<winapi::...::IWhatever>`, you assert that:
    /// *   The vtable's function pointers are valid
    /// *   The functions referenced by said vtable aren't any less sound for any given
    ///     input than the real Microsoft provided implementations of IWhatever
    /// *   The underlying refcount is at least one, if IWhatever is a COM object
    /// *   This and any similar preconditions remain valid for the lifetime of `self`
    pub unsafe trait AsSafe<T> : Sized {
        fn as_safe(&self) -> &T;
        fn as_winapi(&self) -> &T { self.as_safe() } // TODO: deprecate
        fn as_ptr(&self) -> *const T { self.as_safe() }
        // fn as_mut_ptr(&self) -> *mut T { self.as_ptr() as _ } // sketchy, make name scarier?
        // nonnull variants?
    }
}

pub(crate) use sealed::AsSafe;

unsafe impl <I: winapi::Interface> AsSafe<I> for mcom::Rc<I> {
    fn as_safe(&self) -> &I { &**self }
}
