use crate::{Content, Game, NextGameSchedule};
use anyhow::Error;
use chrono::{DateTime, Local};
use chrono_english::{parse_date_string, Dialect};
use gloo_net::http::Request;
use std::collections::HashMap;
use web_sys::HtmlInputElement;
use yew::{prelude::*, Component};

type PreviewStrings = HashMap<usize, String>;

fn images_for_preview(game: &Game, previews: &PreviewStrings) -> Html {
    let preview = previews.get(&game.game_pk).cloned().unwrap_or_default();
    log::info!("previews {:?}", previews);
    let us_avail: Vec<String> = preview
        .split(",")
        .filter_map(|broadcaster| match broadcaster.trim() {
            "ESPN+" | "ESPN +" | "NHLN" | "TNT" | "NBCSCA" => {
            "ESPN+" | "ESPN +" | "ESPN PLUS" | "NHLN" | "TNT" | "NBCSCA" => {
                Some(broadcaster.replace(" ", "").to_owned())
            }
            _ => None,
        })
        .collect();
    log::info!("us_avail {:?}", us_avail);

    html! {
        <>
        {
            for us_avail.iter().map(|broadcaster| html! {
                <img alt = { format!("{}", broadcaster.trim()) } class="logo" src={ format!("/images/{}.png", broadcaster.trim().replace(" ", "_"))} />
            })
        }
        </>
    }
}

fn questions_comments() -> Html {
    html! {
        <div class="mt-3">
        { "Questions, comments? Send an email to " }
        <a href="mailto:rob@tsuk.com"> { "rob@tsuk.com" }</a>
        </div>
    }
}

#[allow(unused)]
pub enum Msg {
    FetchReady(Result<NextGameSchedule, Error>),
    PreviewReady(usize, String),
    Update,
    DateChanged(String),
    UpdateButton,
}

pub struct GamesToday {
    schedule: Option<NextGameSchedule>,
    previews: PreviewStrings,
    date: DateTime<Local>,
    date_str: String,
}

impl GamesToday {
    fn fetch_schedule(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let date = self.date;
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_schedule: NextGameSchedule = Request::get(&format!(
                "https://statsapi.web.nhl.com/api/v1/schedule?expand=schedule.linescore&date={}",
                date.format("%F")
            ))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            for date in &fetched_schedule.dates {
                for game in &date.games {
                    let game_pk = game.game_pk;
                    let uri = format!("https://statsapi.web.nhl.com/{}", game.content.link);
                    let preview_link = link.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let content: Content = Request::get(&uri)
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                        preview_link.send_message(Msg::PreviewReady(
                            game_pk,
                            content.preview_string().unwrap_or_default(),
                        ));
                    });
                }
            }

            link.send_message(Msg::FetchReady(Ok(fetched_schedule)));
        });
    }
}

impl Component for GamesToday {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let date_time_now: DateTime<Local> = Local::now();
        let date = date_time_now;
        let mut gt = Self {
            schedule: None,
            previews: Default::default(),
            date,
            date_str: date.format("%m/%d/%Y").to_string(),
        };
        gt.fetch_schedule(ctx);
        gt
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PreviewReady(game_pk, preview_string) => {
                self.previews.insert(game_pk, preview_string);
                true
            }
            Msg::FetchReady(result) => {
                if let Ok(schedule) = result {
                    self.schedule = Some(schedule);
                    self.previews = Default::default();
                    true
                } else {
                    false
                }
            }
            Msg::UpdateButton => {
                self.fetch_schedule(ctx);
                false
            }
            Msg::DateChanged(date) => {
                self.date_str = date.to_owned();
                let date_only = parse_date_string(&self.date_str, Local::now(), Dialect::Us);
                if let Ok(date_time) = date_only {
                    self.date = date_time;
                    self.fetch_schedule(ctx);
                } else {
                    log::info!("date = {}", self.date_str);
                }
                true
            }
            Msg::Update => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(schedule) = self.schedule.as_ref() {
            // let date_time_now: DateTime<Local> = Local::now();
            let offset = js_sys::Date::new_0().get_timezone_offset() * 60.0;
            let no_games = vec![];
            let games = schedule
                .dates
                .get(0)
                .and_then(|date| Some(&date.games))
                .unwrap_or(&no_games);

            let finished: Vec<_> = games.iter().filter(|game| game.is_finished()).collect();
            let live: Vec<_> = games.iter().filter(|game| game.is_live()).collect();
            let preview: Vec<_> = games.iter().filter(|game| game.is_preview()).collect();
            let postponed: Vec<_> = games.iter().filter(|game| game.is_postponed()).collect();
            html! {
                <div class="container mt-4">
                <h1>
                    { format!("{}: {} games", self.date.format("%F"), schedule.total_items) }
                    <button class="btn btn-primary ms-3" onclick={ctx.link().callback(|_| Msg::Update)}>
                        { "Update" }
                    </button>
                </h1>
                {
                    if live.len() > 0 {
                        html! {
                            <div>
                            <h2>{"Live"}</h2>
                            <ul>
                            {
                                for live.iter().map(|game| html! {
                                    <li class={classes!(game.class())}>{ game.describe(offset) }
                                    { images_for_preview(game, &self.previews) }
                                    </li>
                                })
                            }
                            </ul>
                            </div>
                        }
                        } else {
                            html! {
                                <div></div>
                            }
                        }
                }
                {
                    if preview.len() > 0 {
                        html! {
                            <div>
                            <h2>
                                {
                                    "Upcoming"
                                }
                            </h2>
                            <ul>
                            {
                                for preview .iter().map(|game| html! {
                                    <li class={classes!(game.class())}>
                                    { game.describe(offset) }
                                    { images_for_preview(game, &self.previews) }
                                    </li>
                                })
                            }
                            </ul>
                            </div>
                        }
                        } else {
                            html! {
                                <div></div>
                            }
                        }
                }
                {
                    if finished.len() > 0 {
                        html! {
                            <div>
                            <h2>{"Finished"}</h2>
                            <ul>
                            {
                                for finished.iter().map(|game| html! {
                                    <li class={classes!(game.class())}>{ game.describe(offset) }</li>
                                })
                            }
                            </ul>
                            </div>
                        }
                        } else {
                            html! {
                                <div></div>
                            }
                        }
                }
                {
                    if postponed.len() > 0 {
                        html! {
                            <div>
                            <h2>{"Postponed"}</h2>
                            <ul>
                            {
                                for postponed.iter().map(|game| html! {
                                    <li class={classes!(game.class())}>{ game.describe(offset) }</li>
                                })
                            }
                            </ul>
                            </div>
                        }
                        } else {
                            html! {
                                <div></div>
                            }
                        }
                }
                    <input class="game_date"
                           id="date"
                           type="date"
                           value={self.date.format("%F").to_string()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();

                            Msg::DateChanged(input.value())})}/>
                { questions_comments() }
                </div>
            }
        } else {
            html! {
                <div class="container mt-4">
                <h1>{ "Games Today" }</h1>
                <h2>{ "Loading" }</h2>
                { questions_comments() }
                </div>
            }
        }
    }
}
