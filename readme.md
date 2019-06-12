# `tbot`

Make cool Telegram bots with Rust easily. Here is a simple echo bot:

```rust
use tbot::prelude::*;

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        let reply = context
            .send_message(&context.text.value)
            .into_future()
            .map_err(|error| {
                dbg!(error);
            });

        tbot::spawn(reply);
    });

    bot.polling().start();
}
```

If you're a newcomer, we recommend you go through the [tutorial] first. We also
have several [How-to guides][how-to] for you to use all the power of `tbot`.

If you have a question, ask it in [our group] on Telegram. If you find a bug,
fill an issue on either our [GitLab] or [GitHub] repository.

## Why another crate?

We discuss this question [here][why-another-crate]. In a nutshell: because other
crates aren't that good.

## Installing

Add `tbot` to your `Cargo.toml`. Right now, it can only be done via GitLab as
it's in development right now (we don't recommend depending on our GitHub repo
as it's often outdated):

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
[why-another-crate]: https://gitlab.com/SnejUgal/tbot/wikis/Why-another-crate
[tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
[how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
[GitLab]: https://gitlab.com/SnejUgal/tbot
[GitHub]: https://github.com/SnejUgal/tbot
[`examples`]: ./examples/
[api-docs]: https://docs.rs/tbot
