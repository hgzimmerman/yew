
use yew::prelude::*;
use Context;
use yew::services::route::RouteInfo;
use yew::services::route::RouteSection;
use yew::services::route::Router;

use button::Button;


pub struct Forums {
    route: Route
}

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    CatForum,
    DogForum,
    ForumsList
}

pub enum Msg {
    Navigate(Route)
}

impl Default for Route {
    fn default() -> Self {
        Route::ForumsList
    }
}



impl Router for Route {
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node{segment}) = route.next() {
            match segment.as_str() {
                "cat" => Some(Route::CatForum),
                "dog" => Some(Route::DogForum),
                _ => Some(Route::ForumsList)
            }
        } else {
            None
        }
    }
    fn to_route(&self) -> RouteInfo {
        match *self {
            Route::CatForum => RouteInfo::parse("/cat").unwrap(), // TODO I would like to refactor this into a macro that will fail at compile time if the parse fails
            Route::DogForum => RouteInfo::parse("/dog").unwrap(),
            Route::ForumsList => RouteInfo::parse("/").unwrap()
        }
    }
}


#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub route: Route
}

impl Component<Context> for Forums {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Forums {
            route: props.route
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Navigate(route) => {

                println!("Forums: Navigating");
                // This will inform the Model component that the url has changed, and will cause it to
                // update its route field, and therefore, this component's props.
                context.routing.set_route(super::Route::Forums(route));
                true
            }
        }
    }
    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        println!("change() called in Forums with route");
        self.route = props.route;
        true
    }
}

impl Renderable<Context, Forums> for Forums {
    fn view(&self) -> Html<Context, Self> {
        match self.route {
            Route::CatForum => {
                html! {
                    // Conceptually, these could also be components to which routing props can be passed
                    <>
                        {"I'm the forum for talking about cats"}
                    </>
                }
            }
            Route::DogForum => {
                html! {
                    <>
                        {"I'm the forum for talking about dogs"}
                    </>
                }
            }
            Route::ForumsList => {
                html!{
                    <div>
                        <div>
                            <Button: title="Dog forum", onsignal=|_| Msg::Navigate(Route::DogForum) ,/>
                        </div>
                        <div>
                            <Button: title="Cat forum", onsignal=|_| Msg::Navigate(Route::CatForum) ,/>
                        </div>
                    </div>
                }
            }
        }
    }
}
