use yew::{Component,
    ComponentLink,
    Html,
    html,
    ShouldRender    
};

pub struct Footer;

impl Component for Footer{
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self{
        Footer{}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender{
        true
    }

    fn view(&self) -> Html<Self>{
        html!{
            <div class="jumbotron footer">
                <div class="container">
                    <div class="row">
                        <p class="text-center"><a href="https://twitter.com/danologue"><i class="fab fa-twitter" style="font-size:24px;color:#00acee"></i></a>{" "}<a href="https://github.com/danbugs"><i class="fab fa-github" style="font-size:24px; color:black"></i></a></p>
                    </div>
                    <div class="row">
                        <p class="text-center">{"Developed with "}<i class="fa fa-heart" style="color: red"></i>{" by Dan."}</p>
                    </div>
                </div>
            </div>             
        }        
    }
}