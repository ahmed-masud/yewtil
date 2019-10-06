use yew::{Callback, Children, Component, ComponentLink, Html, Properties};
pub struct Fetching<M: 'static> {
    props: FetchingProps<M>,
}

#[derive(Properties)]
pub struct FetchingProps<M: 'static> {
    children: Children<Fetching<M>>,
    pub(crate) callback: Option<Callback<M>>,
}

impl<M: 'static> Component for Fetching<M> {
    type Message = M;
    type Properties = FetchingProps<M>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Fetching { props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        if let Some(callback) = &self.props.callback {
            callback.emit(msg)
        }
        false
    }

    fn view(&self) -> Html<Self> {
        self.props.children.iter().collect()
    }
}
