macro_rules! handler {
    (
        $context:path,
        $(#[doc = $doc:literal])+
        $name:ident,
    ) => {
        $(#[doc = $doc])+
        pub fn $name<H, F>(&mut self, handler: H)
        where
            H: (Fn(std::sync::Arc<$context>) -> F) + Send + Sync + 'static,
            F: std::future::Future<Output = ()> + Send + 'static,
        {
            let set: fn(&mut Self, H) = paste::expr!(|event_loop, handler| {
                event_loop.[<$name _handlers>].push(Box::new(move |context| {
                    tokio::spawn(handler(context));
                }))
            });

            set(self, handler)
        }

        paste::item! {
            #[allow(dead_code)]
            fn [<will_handle_ $name>](&self) -> bool {
                !self.[<$name _handlers>].is_empty()
            }
        }

        paste::item! {
            fn [<run_ $name _handlers>](&self, context: std::sync::Arc<$context>) {
                &self.[<$name _handlers>].iter().for_each(|handler| {
                    handler(context.clone());
                });
            }
        }
    };
}
