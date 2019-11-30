#![recursion_limit="256"]
use yew::{Component, 
    ComponentLink, 
    html, 
    Html, 
    ShouldRender};

pub struct Home;
impl Component for Home{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home{}
    }

    fn update(&mut self, _msg : Self::Message) -> ShouldRender{
        true
    }

    fn view(&self) -> Html<Self> {
        html!{
            <div class={"container"} style={"height: 100vh"}>
                <div class={"row"}>
                    <div class={"col-md-4 col-md-offset-4"} style={"height:33vh"}></div>
                </div>
                <div class={"row"}>
                    <div class={"jumbotron"}>
                        <h1 class={"text-center"}>{"Come back another time 🕒"}</h1>
                        <p class={"text-center"}>{"I'm building something cool but it's not quite there yet! For updates, follow me on Twitter: @danologue"}</p>
                    </div>
                </div>
            </div>
        }
    }
}