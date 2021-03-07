use crate::gui::addon_widgets::icon_btn;
use crate::gui::styles::{CustomButton, CustomContainer, CustomTextInput};
use iced::{button, scrollable, text_input, Button, Column, Container, Element, Length, Row, Scrollable, Space, Text, TextInput};
use iced_custom_widget::Icons;
#[derive(Debug, Clone, Default)]
pub struct AddInputSrcSec {
    pub search_state: text_input::State,
    pub search_val: String,
    pub ls_inp_srcs: Vec<(String, button::State)>,
    pub filtered_ls_inp_srcs: Vec<(String, button::State)>,
    pub selected_inp_src: Option<String>,
    pub btn_add_state: button::State,
    pub btn_cancel_state: button::State,
    pub scroll_inp_src: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum AddInputSrcMessage {
    SearchChanged(String),
    InpSrcChanged(String),
    AddClicked(String),
    CancelClicked,
}

#[allow(non_upper_case_globals)]
impl AddInputSrcSec {
    const ls_inp_srcs: [&'static str; 10] = [
        "English - QWERTY",
        "English - QWERTZ",
        "English - AZERTY",
        "English - QZERTY",
        "English - QUERTY",
        "English - AWERTY",
        "Khmer - QWERTY",
        "Chinese - QWERTY",
        "Japenese - QWERTY",
        "French - QWERTY",
    ];

    pub fn new() -> Self {
        Self {
            ls_inp_srcs: Self::ls_inp_srcs.iter().map(|layout| (layout.to_string(), button::State::new())).collect(),
            filtered_ls_inp_srcs: Self::ls_inp_srcs.iter().map(|layout| (layout.to_string(), button::State::new())).collect(),
            ..Self::default()
        }
    }

    pub fn update(&mut self, msg: AddInputSrcMessage) {
        use AddInputSrcMessage::*;
        match msg {
            SearchChanged(val) => {
                self.search_val = val;
                self.filtered_ls_inp_srcs = self.ls_inp_srcs.iter().filter(|inp_src| inp_src.0.to_lowercase().contains(&self.search_val.to_lowercase())).cloned().collect();
            }
            InpSrcChanged(val) => self.selected_inp_src = Some(val),
            AddClicked(..) | CancelClicked => self.selected_inp_src = None,
        }
    }

    pub fn view(&mut self) -> Element<AddInputSrcMessage> {
        let AddInputSrcSec {
            search_state,
            search_val,
            filtered_ls_inp_srcs,
            selected_inp_src,
            btn_add_state,
            btn_cancel_state,
            scroll_inp_src,
            ..
        } = self;

        let input_search = TextInput::new(search_state, "Search input source", &search_val, AddInputSrcMessage::SearchChanged).padding(10).style(CustomTextInput::Default);
        let scrollable_inp_src = filtered_ls_inp_srcs
            .iter_mut()
            .fold(Scrollable::new(scroll_inp_src).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (inp_src, state)| {
                let btn = Button::new(state, Text::new(inp_src.as_str()))
                    .width(Length::Fill)
                    .on_press(AddInputSrcMessage::InpSrcChanged(inp_src.clone()))
                    .style(if let Some(selected) = selected_inp_src {
                        if selected == inp_src {
                            CustomButton::Selected
                        } else {
                            CustomButton::Text
                        }
                    } else {
                        CustomButton::Text
                    });
                scrollable.push(btn)
            });
        let inp_src_pane = Container::new(Column::new().push(Container::new(Text::new("Add Input Source")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_inp_src))
            .height(Length::Fill)
            .style(CustomContainer::ForegroundWhite);
        let mut btn_add = icon_btn(btn_add_state, Icons::Ad, "Add", None).style(CustomButton::Primary);
        let btn_cancel = icon_btn(btn_cancel_state, Icons::ArrowLeft, "Cancel", None).on_press(AddInputSrcMessage::CancelClicked).style(CustomButton::Hovered);
        if let Some(layout) = selected_inp_src {
            btn_add = btn_add.on_press(AddInputSrcMessage::AddClicked(layout.clone()));
        }

        Container::new(
            Column::new()
                .spacing(10)
                .push(input_search)
                .push(inp_src_pane)
                .push(Row::new().spacing(10).push(Space::with_width(Length::Fill)).push(btn_cancel).push(btn_add)),
        )
        .width(Length::FillPortion(6))
        .into()
    }
}
