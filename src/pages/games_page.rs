use yew::{prelude::*, Component};

pub enum Msg {}

pub struct GamesToday {
    link: ComponentLink<Self>,
}

impl Component for GamesToday {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container mt-4">
            </div>
        }
    }
}
