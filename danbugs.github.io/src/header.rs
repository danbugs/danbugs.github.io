#![recursion_limit="512"]
use yew::{Component, 
    ComponentLink, 
    html, 
    Html, 
    ShouldRender};    

pub struct Header;
impl Component for Header{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header{}
    }

    fn update(&mut self, _msg : Self::Message) -> ShouldRender{
        true
    }

    fn view(&self) -> Html<Self> {
        html!{
            <div class="jumbotron header">
                <div class="container">
                    <div class="row">
                        <h1>{"Heyo! ~ I'm Dan."}</h1>
                        <p>{"I really like Star Trek: The Next Generation, Super Smash Bros Ultimate, Gone by Michael Grant, and Rust 🦀."}</p>
                    </div>
                </div>
            </div>       
        }
    }
}