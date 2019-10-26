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
        pub fn $name<H, F>(&mut self, handler: H)
        where
            H: (Fn(std::sync::Arc<$context>) -> F) + Send + Sync + 'static,
            F: std::future::Future<Output = ()> + Send + 'static,
        {
            self.$handlers.push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }))
        }

        $(fn $will_handle(&self) -> bool {
            !self.$handlers.is_empty()
        })?

        fn $run_handlers(&self, context: std::sync::Arc<$context>) {
            for handler in &self.$handlers {
                handler(context.clone());
            }
        }
    };
}
