macro_rules! handler {
    (
        #[doc = $doc:literal]
        $handlers:ident,
        $name:ident,
        $context:path,
        $run_handlers:ident,
        $($will_handle:ident,)?
    ) => {
        #[doc = $doc]
        pub fn $name(
            &mut self,
            handler: impl FnMut(& $context) + Send + Sync + 'static,
        ) {
            self.$handlers.push(Mutex::new(Box::new(handler)))
        }

        $(fn $will_handle(&self) -> bool {
            !self.$handlers.is_empty()
        })?

        fn $run_handlers(&self, context: & $context) {
            for handler in &self.$handlers {
                (&mut *handler.lock().unwrap())(context);
            }
        }
    };
}
