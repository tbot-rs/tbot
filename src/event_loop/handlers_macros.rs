macro_rules! handlers {
    (
        $(
            $(#[doc = $doc:literal])+
            $name:ident: $context:path,
        )+
    ) => {
        $(
            $(#[doc = $doc])+
            pub fn $name<H, F>(&mut self, handler: H)
            where
                H: (Fn(std::sync::Arc<$context>) -> F) + Send + Sync + 'static,
                F: std::future::Future<Output = ()> + Send + 'static,
            {
                self.add_handler(handler);
            }
        )+
    };
}
