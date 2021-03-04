use iced::{button, Button, Column, Element, Row, Text, scrollable, Scrollable, Container,Length};

#[derive(Debug, Default, Clone)]
pub struct IconStyle {
    pub apply: button::State,
    pub text: String,
    pub scroller: Option<ShowSrollBar>,
    pub scroll_content: scrollable::State,
}
#[derive(Debug, Clone, Copy)]
pub enum IconMsg {
    ApplyClicked,
    ScrollChanged(ShowSrollBar), 

}
impl IconStyle {
    pub fn new() -> Self {
        Self{
            apply: button::State::new(),
            text: String::new(),
            scroller: Some(ShowSrollBar::MouseTouchPad),
            scroll_content: scrollable::State::new(),
        }  
    }
    pub fn update(&mut self, message: IconMsg) {
        match message {
            IconMsg::ApplyClicked => {}
            IconMsg::ScrollChanged(scroll) => {
                self.scroller = Some(scroll);
            }
        }
    }
    pub fn view(&mut self) -> Element<IconMsg> {
        let IconStyle{
            apply,
            text,
            scroll_content,
            scroller,
        }=self;
        
        let btn = Row::new()
            .spacing(20)
            .push(Button::new(&mut self.apply, Text::new("Apply")).on_press(IconMsg::ApplyClicked));

        let whole_container = Column::new()
            .spacing(10)
            .push(btn);

        let scroll_list = Scrollable::new(scroll_content).push(whole_container);

        Container::new(scroll_list)
        .center_x   ()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum ShowSrollBar {
    MouseTouchPad,
    Scrolling,
    Always,
}
impl ShowSrollBar {
    fn all() -> [ShowSrollBar; 3] {
        [ShowSrollBar::MouseTouchPad, ShowSrollBar::Scrolling, ShowSrollBar::Always]
    }
}
