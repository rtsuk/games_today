use crate::NextGameSchedule;
use anyhow::Error;
use chrono::{Date, DateTime, Local};
use chrono_english::{parse_date_string, Dialect};
use std::time::Duration;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        interval::{IntervalService, IntervalTask},
    },
    Component,
};

pub enum Msg {
    FetchReady(Result<NextGameSchedule, Error>),
    Update,
    DateChanged(String),
    UpdateButton,
}

pub struct GamesToday {
    link: ComponentLink<Self>,
    schedule: Option<NextGameSchedule>,
    date: Date<Local>,
    date_str: String,
    schedule_fetch: Option<FetchTask>,
    refresh: Option<IntervalTask>,
    update_button_ref: NodeRef,
}

impl GamesToday {
    fn fetch_json(&mut self) {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<NextGameSchedule, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Some(Msg::FetchReady(data))
                } else {
                    None // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get(format!(
            "https://statsapi.web.nhl.com/api/v1/schedule?expand=schedule.linescore&date={}",
            self.date.format("%F")
        ))
        .body(Nothing)
        .unwrap();
        let task = FetchService::fetch(request, callback).unwrap();
        self.schedule_fetch = Some(task);
    }

    fn start_refresh_timer(&mut self) {
        let task = IntervalService::spawn(
            Duration::from_secs(30 * 60),
            self.link.callback(|_| Msg::Update),
        );
        self.refresh = Some(task);
    }
}

impl Component for GamesToday {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let date_time_now: DateTime<Local> = Local::now();
        let date = date_time_now.date();
        let mut gt = Self {
            link,
            schedule: None,
            date,
            date_str: date.format("%m/%d/%Y").to_string(),
            schedule_fetch: None,
            refresh: None,
            update_button_ref: NodeRef::default(),
        };
        gt.fetch_json();
        gt.start_refresh_timer();
        gt
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(result) => {
                if let Ok(schedule) = result {
                    log::info!("schedule = {:?}", schedule);
                    self.schedule = Some(schedule);
                    true
                } else {
                    false
                }
            }
            Msg::UpdateButton => {
                self.fetch_json();
                false
            }
            Msg::DateChanged(date) => {
                self.date_str = date.to_owned();
                let date_only = parse_date_string(&self.date_str, Local::now(), Dialect::Us);
                if let Ok(date_time) = date_only {
                    let date = date_time.date();
                    self.date = date;
                    self.fetch_json();
                } else {
                    log::info!("date = {}", self.date_str);
                }
                true
            }
            Msg::Update => true,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
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
                    <a class="btn btn-primary ms-3" ref=self.update_button_ref.clone()
                        onclick=self.link.callback(|_| Msg::UpdateButton)>{ "Update" }</a>
                </h1>
                {
                    if live.len() > 0 {
                        html! {
                            <div>
                            <h2>{"Live"}</h2>
                            <ul>
                            {
                                for live.iter().map(|game| html! {
                                    <li class=classes!(game.class())>{ game.describe(offset) }</li>
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
                                    <li class=classes!(game.class())>{ game.describe(offset) }</li>
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
                                    <li class=classes!(game.class())>{ game.describe(offset) }</li>
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
                                    <li class=classes!(game.class())>{ game.describe(offset) }</li>
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
                    <input id="date" type="date" value=self.date_str.to_string()
                        oninput=self.link.callback(|e: InputData| Msg::DateChanged(e.value))/>
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
