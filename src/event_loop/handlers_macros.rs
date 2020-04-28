macro_rules! handler {
    (
        $context:path,
        $(#[doc = $doc:literal])+
        $name:ident,
        $(#[doc = $doc_if:literal])+
        $name_if:ident,
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

        $(#[doc = $doc_if])+
        pub fn $name_if<H, HF, P, PF>(
            &mut self,
            predicate: P,
            handler: H,
        ) where
            H: (Fn(Arc<$context>) -> HF)
                + Send
                + Sync
                + 'static,
            HF: Future<Output = ()> + Send + 'static,
            P: (Fn(Arc<$context>) -> PF)
                + Send
                + Sync
                + 'static,
            PF: Future<Output = bool> + Send + 'static,
        {
            let predicate = Arc::new(predicate);
            let handler = Arc::new(handler);
            self.$name(move |context| {
                let predicate = Arc::clone(&predicate);
                let handler = Arc::clone(&handler);
                async move {
                    if predicate(Arc::clone(&context)).await {
                        handler(context).await
                    }
                }
            });
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
