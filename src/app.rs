use crate::portfolio::{self, *};
use yew::{html::IntoPropValue, prelude::*, props};

pub struct App {
    portfolio: Portfolio,
}

pub enum AppMsg {}

impl Component for App {
    type Message = AppMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let portfolio = serde_json::from_str::<Portfolio>(include_str!("../portfolio.json"))
            .expect("Could not load portfolio.json");

        Self { portfolio }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let scroll_sections = self
            .portfolio
            .projects
            .iter()
            .map(|p| p.title.clone())
            .collect::<Vec<_>>();

        let projects = self.portfolio.projects.iter().enumerate().map(|(n, p)| {
            let props = props! { ProjectProps { id: n, project: p.clone() } };
            html! {
                <div class="centered">
                    <Project ..props />
                </div>
            }
        });

        html! {
            <>
                <main id="main" class="flex-column">
                    {for projects}
                </main>


                //<Scrollbar />
            </>
        }
    }
}

/*
#[derive(Properties, PartialEq)]
struct ScrollbarProps {
    scroll_sections: Vec<String>,
    selected_section: usize,
}

#[function_component(Scrollbar)]
fn scrollbar(props: &ScrollbarProps) -> Html {
    html! {
        <div id="scrollbar">

        </div>
    }
}
*/

#[derive(Properties, PartialEq)]
struct ProjectProps {
    id: usize,
    project: portfolio::Project,
}

#[function_component(Project)]
fn project(props: &ProjectProps) -> Html {
    let links = props.project.links.iter().map(|l| {
        let props = props! {
            LinkProps {
                link: l.clone(),
            }
        };

        html! {
            <Link ..props />
        }
    });

    let carousel = if !props.project.images.is_empty() {
        let carousel_props = props! {
            ImageCarouselProps {
                images: props.project.images.clone()
            }
        };
        html! { <ImageCarousel ..carousel_props /> }
    } else {
        html! {}
    };

    html! {
        <div class="project-card">
            <div>
                <span class="title">{ &props.project.title }</span>
                <span class="status">
                    <span class={ format!("indicator {}", props.project.status.to_string().to_lowercase()) } /> { props.project.status.to_string() }
                </span>
            </div>

            { carousel }

            <p>{ &props.project.description }</p>
            <div class="links">
                { for links }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ImageCarouselProps {
    images: Vec<Image>,
}

struct ImageCarousel {
    current_img: usize,
}

enum ImageCarouselMsg {
    Left,
    Right,
}

impl Component for ImageCarousel {
    type Message = ImageCarouselMsg;

    type Properties = ImageCarouselProps;

    fn create(ctx: &Context<Self>) -> Self {
        ImageCarousel { current_img: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageCarouselMsg::Left => {
                self.current_img = if self.current_img == 0 {
                    ctx.props().images.len() - 1
                } else {
                    self.current_img - 1
                };
            }
            ImageCarouselMsg::Right => {
                self.current_img = if self.current_img == ctx.props().images.len() - 1 {
                    0
                } else {
                    self.current_img + 1
                };
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let n_images = ctx.props().images.len();
        let current_n = self.current_img + 1;
        let progress_text = format!("{current_n}/{n_images}");

        let current_img = ctx.props().images[self.current_img].src.clone();

        let link = ctx.link();

        let click_left = link.callback(|_| ImageCarouselMsg::Left);
        let click_right = link.callback(|_| ImageCarouselMsg::Right);

        let buttons = if ctx.props().images.len() == 1 {
            html! {}
        } else {
            html! {
                <div class="carousel-buttons">
                    <div class="carousel-button left hover-out" onclick={click_left}>{ "<" }</div>
                    <div class="carousel-progress">{ progress_text }</div>
                    <div class="carousel-button right hover-out" onclick={click_right}>{ ">" }</div>
                </div>
            }
        };

        html! {
            <div class="carousel-container">
                <img src={current_img} />

                { buttons }
            </div>
        }
    }
}

//#[function_component(ImageCarousel)]
/*
fn image_carousel(props: &ImageCarouselProps) -> Html {
    if props.images.is_empty() {
        return html! {};
    }

    let current_slide = use_state(|| 0);

    let id = format!("--{}", props.id);

    let slide_buttons = props.images.iter().enumerate().map(|(n, img)| {
        html! {
            <a href={format!("#{id}")} />
        }
    });

    let slides = props.images.iter().enumerate().map(|(n, img)| {
        html! {
            <div class="slide" id={ format!("{id}-slide-{n}") }>
                <img src={ img.src.clone() } />
            </div>
        }
    });

    html! {
        <div class="carousel-container">
            <div class="carousel-buttons">
                {for slide_buttons}
            </div>
            <div class="carousel-slides">
                {for slides}
            </div>
        </div>
    }
}
 */

#[derive(Properties, PartialEq)]
struct LinkProps {
    link: portfolio::Link,
}

#[function_component(Link)]
fn link(props: &LinkProps) -> Html {
    let link_icon_src = props.link.icon.icon_src();
    let href = props.link.href.clone();

    html! {
        <a {href} class="link hover-out">
            <img class="link-icon" src={link_icon_src} />
            { &props.link.display }
        </a>
    }
}
