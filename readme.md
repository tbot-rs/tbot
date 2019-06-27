# `tbot`

Make cool Telegram bots with Rust easily. For example, here's a simple echo bot:

```rust
use tbot::prelude::*;

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        let reply = context
            .send_message(&context.text.value)
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(reply);
    });

    bot.polling().start();
}
```

If you're a newcomer, we recommend you go through the [tutorial] first. We also
have several [How-to guides][how-to] to help you use `tbot`.

If you have a question, ask it in [our group] on Telegram. If you find a bug,
fill an issue on either our [GitLab] or [GitHub] repository.

## Features

- Full Bots API support, including media upload/download and recent API
  updates;
- Support for both polling and [webhooks];
- Type-safe API;
- Based on futures and `tokio`;
- Easy to use, while scalable and configurable.

## Installing

<!--
Add `tbot` to your Cargo.toml:

```toml
[dependencies]
tbot = "0.1"
```
-->

Add `tbot` to your `Cargo.toml`. Right now, it can only be done via git until
the first release:

```toml
[dependencies]
tbot = { git = "https://gitlab.com/SnejUgal/tbot.git" }
```

## Documentation

There are many examples in the [`examples`] directory to see `tbot` in action.
If you want to get started with `tbot`, go through the [tutorial]. When you
start making your bot, our [How-to guides][how-to] will help you. And you can
always refer to our API docs on [*docs.rs*][api-docs].

> **Note**: As `tbot` is only in development at this moment, its technical
> docs are only available locally with `cargo doc`.

## Contribution

Glad you want to contribute to `tbot`! We develop the crate on [GitLab], so
create your pull/merge request there if you can. We accept pull requests on
[GitHub] as well, but we prefer [GitLab].

[our group]: t.me/tbot_group
[webhooks]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
[tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
[how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
[GitLab]: https://gitlab.com/SnejUgal/tbot
[GitHub]: https://github.com/SnejUgal/tbot
[`examples`]: ./examples/
[api-docs]: https://docs.rs/tbot
