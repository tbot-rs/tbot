use tbot::{prelude::*, state::Chats, Bot};
use tokio::sync::Mutex;

#[derive(Debug, Default)]
struct Questionary {
    name: Option<String>,
    planet: Option<String>,
    age: Option<u8>,
    programming_languages: Option<Vec<String>>,
    fought_borrow_checker: Option<bool>,
}

impl Questionary {
    fn next_question(&self) -> &'static str {
        if self.name.is_none() {
            "I, one of The Witnesses of Our Lord and Savior, greet you, \
            and I want you to fill out our questionary. But first, introduce \
            yourself."
        } else if self.planet.is_none() {
            "What planet do you live on?"
        } else if self.age.is_none() {
            "How many years have you been spinning around your star?"
        } else if self.programming_languages.is_none() {
            "Except for The Only Moral Programming Language, what other \
            (i.e. immoral) languages have you used during your existence?"
        } else if self.fought_borrow_checker.is_none() {
            "Have you ever sinned by fighting Our Lord and Savior, \
            The Borrow Checker?"
        } else {
            "Thanks for answering our questions. Our Lord and Savior is aready \
            considering your application."
        }
    }
}

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN")
        .stateful_event_loop(Mutex::new(Chats::new()));

    bot.start(|context, state| async move {
        let questionary = Questionary::default();
        let first_question = questionary.next_question();

        state.lock().await.insert(&*context, questionary);

        context
            .send_message_in_reply(first_question)
            .call()
            .await
            .unwrap();
    });

    bot.text(|context, state| async move {
        let mut questionaries = state.lock().await;
        let questionary =
            if let Some(questionary) = questionaries.get_mut(&*context) {
                questionary
            } else {
                let questionary = Questionary::default();
                let first_question = questionary.next_question();

                questionaries.insert(&*context, questionary);

                context
                    .send_message_in_reply(first_question)
                    .call()
                    .await
                    .unwrap();

                return;
            };

        if questionary.name.is_none() {
            questionary.name.replace(context.text.value.to_owned());
        } else if questionary.planet.is_none() {
            questionary.planet.replace(context.text.value.to_owned());
        } else if questionary.age.is_none() {
            if let Ok(age) = context.text.value.parse() {
                questionary.age.replace(age);
            } else {
                context
                    .send_message_in_reply(
                        "Enter an integer in the range [0; 255].",
                    )
                    .call()
                    .await
                    .unwrap();

                return;
            }
        } else if questionary.programming_languages.is_none() {
            let languages = context
                .text
                .value
                .split_whitespace()
                .map(ToOwned::to_owned)
                .collect();
            questionary.programming_languages.replace(languages);
        } else if questionary.fought_borrow_checker.is_none() {
            let has_sinned = !context
                .text
                .value
                .eq_ignore_ascii_case("Never have I sinned this badly");
            questionary.fought_borrow_checker.replace(has_sinned);

            println!("An applicant's questionary: {:#?}", questionary);
        } else {
            context
                .send_message_in_reply(
                    "Your application is already being considered \
                     by Our Lord and Savior.",
                )
                .call()
                .await
                .unwrap();

            return;
        }

        context
            .send_message_in_reply(questionary.next_question())
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}
