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
    schedule_fetch: Option<FetchTask>,
}

impl GamesToday {
    fn fetch_json(&mut self) -> yew_services::fetch::FetchTask {
        let callback =
            self.link
                .batch_callback(move |response: Response<Json<Result<Schedule, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    log::info!("META: {:?}, {:?}", meta, data);
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
            schedule_fetch: None,
        };
        let schedule_fetch = gt.fetch_json();
        gt.schedule_fetch = Some(schedule_fetch);
        gt
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container mt-4">
            <h1>{"Games Today"}</h1>
            </div>
        }
    }
}
