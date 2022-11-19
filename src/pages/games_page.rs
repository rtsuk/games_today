use crate::NextGameSchedule;
use anyhow::Error;
use chrono::{DateTime, Local};
use chrono_english::{parse_date_string, Dialect};
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::{prelude::*, Component};

#[allow(unused)]
pub enum Msg {
    FetchReady(Result<NextGameSchedule, Error>),
    Update,
    DateChanged(String),
    UpdateButton,
}

pub struct GamesToday {
    schedule: Option<NextGameSchedule>,
    date: DateTime<Local>,
    date_str: String,
    // schedule_fetch: Option<FetchTask>,
    // refresh: Option<IntervalTask>,
    update_button_ref: NodeRef,
}

impl GamesToday {
    fn fetch_schedule(&mut self, ctx: &Context<Self>) {
        log::info!("fetch_schedule");
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
            date,
            date_str: date.format("%m/%d/%Y").to_string(),
            update_button_ref: NodeRef::default(),
        };
        gt.fetch_schedule(ctx);
        gt
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchReady(result) => {
                if let Ok(schedule) = result {
                    self.schedule = Some(schedule);
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
                    <a class="btn btn-primary ms-3" ref={ self.update_button_ref.clone() }
                        onclick={ ctx.link().callback(|_| Msg::UpdateButton)}>{ "Update" }</a>
                </h1>
                {
                    if live.len() > 0 {
                        html! {
                            <div>
                            <h2>{"Live"}</h2>
                            <ul>
                            {
                                for live.iter().map(|game| html! {
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
                    <input id="date" type="date" value={self.date_str.to_string()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();

                            Msg::DateChanged(input.value())})}/>
                </div>
            }
        } else {
            html! {
                <div class="container mt-4">
                <h1>{ "Games Today" }</h1>
                <h2>{ "Loading" }</h2>
                </div>
            }
        }
    }
}
