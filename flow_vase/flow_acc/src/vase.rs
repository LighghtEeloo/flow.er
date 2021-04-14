use yew::Component;

pub struct Vase {

}

impl Component for Vase {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        todo!()
    }
}
