#![recursion_limit="512"]
use yew::{Component, 
    ComponentLink, 
    html, 
    Html, 
    ShouldRender};

// Modules Import
use header::Header;
use timeline_element::TimelineElement;
use footer::Footer;

// Modules Declaration
pub mod header; 
pub mod timeline_element;  
pub mod footer; 

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
            <>
            <Header/>
            <div class="container">
                <ul class="timeline">
                    <TimelineElement 
                    title={"Earth Invaders"}
                    subtitle={"Unity + Piskel + Bosca Ceoil"}
                    text={"One of the first games I ever made! I made all the art, music, and gameplay. Fun fact: all the enemies are connected by a graph. No reason for doing it except that I had just learned data structures!"}
                    link1={(String::from("Check it out: "), String::from("https://chiarld.itch.io/earth-invaders"), String::from("here."))}
                    link2={(String::from("Code: "), String::from("https://github.com/danbugs/earth_invaders"), String::from("here."))}/>
                    <TimelineElement
                    title={"Cool Bandpass Filter"}
                    subtitle={"ReactJS + p5JS"}
                    text={"This is a bandpass filter I made for my Digital Signal Processing class' using ReactJS and p5JS."}
                    link1={(String::from("Check it out: "), String::from("https://danbugs.github.io/cool_bandpass_filter/"), String::from("here."))}
                    link2={(String::from("Code: "), String::from("https://github.com/danbugs/cool_bandpass_filter"), String::from("here."))}
                    /> 
                    <TimelineElement
                    title={"Top 10 Anime Betrayals TRUE COMBOS into Jab Lock"}
                    subtitle={"Super Smash Bros Ultimate"}
                    text={"I got saved by Pac-Man's trampoline and proceeded to get a fast fall bair into jab-lock with dair, magnet, and a finishing forward smash."}
                    link1={(String::from("Yeet Smash: "), String::from("https://www.youtube.com/watch?v=phfa84oXg-Q&feature=youtu.be&t=472"), String::from("here."))}
                    link2={(String::from("Gloomshot-Ultimate: "), String::from("https://www.youtube.com/watch?v=Vi8oQ5N-0aM&feature=youtu.be&t=840"), String::from("here."))}
                    link3={(String::from("Dragon Smash: "), String::from("https://youtu.be/G7NzEXmPytI?t=178"), String::from("here."))}
                    />                      
                    <TimelineElement
                    title={"Cool Processor"}
                    subtitle={"Verilog"}                        
                    text={"This was a simulation of a 16-bit processor with working RAM, decoding, register bank, and ALU. My team and I made testbenches for every module and utilized ModelSim for simulation and testing."}
                    link1={(String::from("Check it out: "), String::from("https://www.edaplayground.com/x/4CUA"), String::from("here."))}
                    link2={(String::from("Code: "), String::from("https://github.com/danbugs/cool_processor"), String::from("here."))}
                    /> 
                    <TimelineElement
                    title={"danbugs"}
                    subtitle={"Rust + YouTube"}                        
                    text={"danbugs is a YouTube channel I created to help out the auditory learners out there get through \"The Rust Programming Language\" book by Steve Klabnik and Carol Nichols."}
                    link1={(String::from("Check it out: "), String::from("https://www.youtube.com/playlist?list=PLK_g1a_cAfaaAO6io1Tluy7EZXhAAK1lC"), String::from("here."))}
                    link2={(String::from("Code: "), String::from("https://github.com/danbugs/danbugs"), String::from("here."))}
                    />   
                    <TimelineElement
                    title={"This website!"}
                    subtitle={"Yew + CSS"}                        
                    text={"Utilizing the Yew, a Rust framework for creating multi-threaded frontend apps, I made this website!"}
                    link1={(String::from("Code: "), String::from("https://github.com/danbugs/danbugs.github.io"), String::from("here."))}
                    />                                     
                </ul>
            </div>  
            <Footer/>
            </>         
        }
    }
}