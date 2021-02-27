use crate::Schedule;
use anyhow::Error;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    Component,
};
use yew_services::fetch::{FetchService, FetchTask, Request, Response};

pub enum Msg {
    FetchReady(Result<Schedule, Error>),
}

pub struct GamesToday {
    link: ComponentLink<Self>,
    schedule: Schedule,
    schedule_fetch: Option<FetchTask>,
}

impl GamesToday {
    fn fetch_json(&mut self) -> yew_services::fetch::FetchTask {
        let callback =
            self.link
                .batch_callback(move |response: Response<Json<Result<Schedule, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success() {
                        Some(Msg::FetchReady(data))
                    } else {
                        None // FIXME: Handle this error accordingly.
                    }
                });
        let request = Request::get("https://statsapi.web.nhl.com/api/v1/schedule")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for GamesToday {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut gt = Self {
            link,
            schedule: Schedule::default(),
            schedule_fetch: None,
        };
        let schedule_fetch = gt.fetch_json();
        gt.schedule_fetch = Some(schedule_fetch);
        gt
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(result) => {
                if let Ok(schedule) = result {
                    self.schedule = schedule;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let offset = js_sys::Date::new_0().get_timezone_offset() * 60.0;
        let no_games = vec![];
        let games = self
            .schedule
            .dates
            .get(0)
            .and_then(|date| Some(&date.games))
            .unwrap_or(&no_games);
        let (finished, unfinished): (Vec<_>, Vec<_>) =
            games.iter().partition(|game| game.is_finished());
        html! {
            <div class="container mt-4">
            <h1>{ format!("Games Today: {}",self.schedule.total_games) }</h1>
            <h2>{"In Progress and Upcoming"}</h2>
            <ul>
            {
                for unfinished.iter().map(|game| html! {
                    <li class=classes!(game.class())>{ game.describe(offset) }</li>
                })
            }
            </ul>
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
    }
}
