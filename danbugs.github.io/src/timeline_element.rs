use yew::{Component,
    ComponentLink,
    Html,
    html, 
    ShouldRender,  
};

use yew::prelude::*;
use std::ops::Deref;

pub struct TimelineElement{
    title: String,
    subtitle: String,
    text: String,
    link1: (String, String, String),
    link2: (String, String, String),
    link3: (String, String, String),
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub title: String,
    #[props(required)]
    pub subtitle: String,    
    #[props(required)]
    pub text: String,
    pub link1: (String, String, String),
    pub link2: (String, String, String),
    pub link3: (String, String, String),
}

impl Component for TimelineElement{
    type Message = ();
    type Properties = Props; 

    fn create(props : Self::Properties, _: ComponentLink<Self>) -> Self{
        TimelineElement{
            title: props.title,
            subtitle: props.subtitle,
            text: props.text,
            link1: props.link1,
            link2: props.link2,
            link3: props.link3,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender{
        true
    }

    fn view(&self) -> Html<Self>{
        let link1: html::Href = (&self.link1.1).deref().into();
        let link2: html::Href = (&self.link2.1).deref().into();
        let link3: html::Href = (&self.link3.1).deref().into();
        html!{
            <li>
                <h3>{&self.title}</h3>
                <h6>{&self.subtitle}</h6>
                <hr/>
                <p>{&self.text}</p>
                <p>{&self.link1.0}<a href={link1}>{&self.link1.2}</a></p>                          
                <p>{&self.link2.0}<a href={link2}>{&self.link2.2}</a></p>
                <p>{&self.link3.0}<a href={link3}>{&self.link3.2}</a></p>
            </li>        
        }
    }
}