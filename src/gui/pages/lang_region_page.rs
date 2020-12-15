use chrono::prelude::*;
use num_format::{SystemLocale, Locale};
use num_format::{Buffer, CustomFormat, Grouping};
use super::super::styles::{CustomButton, CustomContainer, CustomCheckbox, CustomSelect, HOVERED};
use iced::{
   button, scrollable, pick_list, Align, Length, Space, Svg, HorizontalAlignment,
   Button, Checkbox, Column, Container, Element, Row, Scrollable, Text, PickList, 
};
use iced_custom_widget::{Icon, IconBrand};
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum LangRegionMessage {
   TabChanged(usize),
   BtnAddClicked,
   BtnRemoveClicked,
   BtnUpClicked,
   BtnDownClicked,
   LangSelected(usize),
   RegionChanged(SystemLocale),
   FirstDayChanged(Weekdays),
   TimeFormatToggled(bool),
   TempChanged(Temperature),
   NumSepGroupingChanged(GroupSeperator),
   NumSepDecimalChanged(DecimalSeperator),
   CurrencySepGroupingChanged(GroupSeperator),
   CurrencySepDecimalChanged(DecimalSeperator),
   ShortDateFormatChanged(DateFormat),
   LongDateFormatChanged(DateFormat),
   ShortTimeFormatChanged(TimeFormat),
   LongTimeFormatChanged(TimeFormat),
   BtnAddAppClicked,
   BtnRemoveAppClicked,
   AppSelected(usize),
   AppLangChanged(AppLang),
   DefaultsClicked,
   ApplyClicked,
   CancelClicked,
}

#[derive(Debug)]
pub struct LangRegionPage {
   tabbar_state: Vec<(String, button::State)>,
   current_tab_idx: usize,
   general_tab: GeneralTab,
   formats_tab: FormatsTab,
   apps_tab: AppsTab,
   defaults_state: button::State,
   cancel_state: button::State,
   appply_state: button::State,
   is_changed: bool,
   short_date_format: DateFormat,
   long_date_format: DateFormat,
   short_time_format: TimeFormat,
   long_time_format: TimeFormat,
}

impl LangRegionPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  General  ".to_string(), button::State::new()),
            ("  Formats  ".to_string(), button::State::new()),
            ("  Apps  ".to_string(), button::State::new()),
         ],
         current_tab_idx: 0,
         general_tab: GeneralTab::new(),
         formats_tab: FormatsTab::new(),
         apps_tab: AppsTab::new(),
         defaults_state: button::State::new(),
         cancel_state: button::State::new(),
         appply_state: button::State::new(),
         is_changed: false,
         short_date_format: DateFormat::_dmy,
         long_date_format: DateFormat::_dMMMMyyyy,
         short_time_format: TimeFormat::_hma,
         long_time_format: TimeFormat::_hmsaz,
      }
   }

   pub fn update(&mut self, msg: LangRegionMessage) {
      use LangRegionMessage::*;
      match msg {
         TabChanged(idx) => self.current_tab_idx = idx,
         BtnAddClicked => {
            self.general_tab.prefered_langs.push(("Other".to_string(), "Other".to_string(), button::State::new()));
            self.is_changed = true;
         },
         BtnRemoveClicked => {
            if let Some(selected_idx) = self.general_tab.selected_lang {
               self.general_tab.prefered_langs.remove(selected_idx);
               self.general_tab.selected_lang = None;
               self.is_changed = true;
            }
         },
         BtnUpClicked => {
            if let Some(selected_idx) = self.general_tab.selected_lang {
               self.general_tab.prefered_langs.swap(selected_idx, selected_idx-1);
               self.general_tab.selected_lang = Some(selected_idx-1);
               self.is_changed = true;
            }
         },
         BtnDownClicked => {
            if let Some(selected_idx) = self.general_tab.selected_lang {
               self.general_tab.prefered_langs.swap(selected_idx, selected_idx+1);
               self.general_tab.selected_lang = Some(selected_idx+1);
               self.is_changed = true;
            }
         },
         LangSelected(idx) => self.general_tab.selected_lang = Some(idx),
         RegionChanged(val) => {self.general_tab.selected_region = val; self.is_changed = true;},
         FirstDayChanged(val) => {self.general_tab.selected_firstday = val; self.is_changed = true;},
         TimeFormatToggled(is_checked) => {self.general_tab.is_24_hours_format = is_checked; self.is_changed = true;},
         TempChanged(val) => {self.general_tab.selected_temp = val; self.is_changed = true;},
         NumSepGroupingChanged(val) => {self.general_tab.selected_num_sep_grouping = val; self.is_changed = true;},
         NumSepDecimalChanged(val) => {self.general_tab.selected_num_sep_decimal = val; self.is_changed = true;},
         CurrencySepGroupingChanged(val) => {self.general_tab.selected_currency_sep_grouping = val; self.is_changed = true;},
         CurrencySepDecimalChanged(val) => {self.general_tab.selected_currency_sep_decimal = val; self.is_changed = true;},
         ShortDateFormatChanged(val) => {self.formats_tab.selected_short_date_format = val; self.is_changed = true;},
         LongDateFormatChanged(val) => {self.formats_tab.selected_long_date_format = val; self.is_changed = true;},
         ShortTimeFormatChanged(val) => {self.formats_tab.selected_short_time_format = val; self.is_changed = true;},
         LongTimeFormatChanged(val) => {self.formats_tab.selected_long_time_format = val; self.is_changed = true;},
         BtnAddAppClicked => {self.apps_tab.app_list.push(('\u{f120}', "Terminal".to_string(), pick_list::State::default(), AppLang::default(), button::State::new())); self.is_changed = true;},
         BtnRemoveAppClicked => {
            if let Some(selected_idx) = self.apps_tab.selected_app {
               self.apps_tab.app_list.remove(selected_idx);
               self.apps_tab.selected_app = None;
               self.is_changed = true;
            }
         },
         AppSelected(idx) => self.apps_tab.selected_app = Some(idx),
         AppLangChanged(val) => {
            if let Some(selected_idx) = self.apps_tab.selected_app {
               self.apps_tab.app_list.get_mut(selected_idx).unwrap().3 = val;
               self.is_changed = true;
            }
         },
         DefaultsClicked => *self = Self::new(),
         ApplyClicked => {
            self.short_date_format = self.formats_tab.selected_short_date_format;
            self.long_date_format = self.formats_tab.selected_long_date_format;
            self.short_time_format = self.formats_tab.selected_short_time_format;
            self.long_time_format = self.formats_tab.selected_long_time_format;
            self.is_changed = false;
         },
         CancelClicked => self.is_changed = false,
      }
   }

   pub fn view(&mut self) -> Element<LangRegionMessage> {
      let LangRegionPage {
         tabbar_state,
         current_tab_idx,
         general_tab,
         formats_tab,
         apps_tab,
         defaults_state,
         cancel_state,
         appply_state,
         is_changed,
         short_date_format,
         long_date_format,
         short_time_format,
         long_time_format,
      } = self;

      // ផ្នែកក្បាល
      let icon = Svg::from_path("assets/images/language.svg").width(Length::Units(75)).height(Length::Units(75));
      let txt_lang = Text::new("Language & Region preferences control the language you see in menus and dialogs, formats of dates, times and numbers.");
      let header_sec = Container::new(
         Row::new().spacing(20).align_items(Align::Center)
         .push(icon)
         .push(txt_lang)
      );

      // របារផ្ទាំង
      let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
      for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
         let mut btn = Button::new(btn_state, Text::new(name.as_str())).padding(5).on_press(LangRegionMessage::TabChanged(idx));
         if *current_tab_idx == idx {
            btn = btn.style(CustomButton::SelectedTab);
         } else {
            btn = btn.style(CustomButton::Tab);
         }
         tabbar = tabbar.push(btn);
      }
      let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
      let tabbar_sec = Container::new(tabbar_con).padding(7).width(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let GeneralTab {
               prefered_langs,
               selected_lang,
               prefered_lang_scroll,
               add_state,
               remove_state,
               up_state,
               down_state,
               region_state,
               selected_region,
               firstday_state,
               selected_firstday,
               is_24_hours_format,
               temp_state,
               selected_temp,
               num_sep_grouping,
               selected_num_sep_grouping,
               num_sep_decimal,
               selected_num_sep_decimal,
               currency_sep_grouping,
               selected_currency_sep_grouping,
               currency_sep_decimal,
               selected_currency_sep_decimal,
               now,
               number,
               currency,
            } = general_tab;

            // ផ្ទាំងខាងឆ្វេង
            let lb_prefered_lang = Text::new("Preferred Languages:");
            let btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(LangRegionMessage::BtnAddClicked).style(CustomButton::Text);
            let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
            if selected_lang.is_some() && prefered_langs.len() > 1 {
               btn_remove = btn_remove.on_press(LangRegionMessage::BtnRemoveClicked);
            }

            let mut btn_up = Button::new(up_state, Icon::new('\u{f106}').size(23)).padding(2).style(CustomButton::Text);
            let mut btn_down = Button::new(down_state, Icon::new('\u{f107}').size(23)).padding(2).style(CustomButton::Text);
            if let Some(selected_idx) = selected_lang {
               if *selected_idx != 0 {
                  btn_up = btn_up.on_press(LangRegionMessage::BtnUpClicked);
               }
               if *selected_idx != (prefered_langs.len()-1) {
                  btn_down = btn_down.on_press(LangRegionMessage::BtnDownClicked);
               }
            }
            let btn_shift_group = Container::new(
               Row::new().push(btn_up).push(btn_down)
            );

            let btn_group = Container::new(
               Row::new().push(btn_add).push(btn_remove).push(Space::with_width(Length::Fill)).push(btn_shift_group)
            ).width(Length::Fill).style(CustomContainer::Header);
            let lang_group = prefered_langs.iter_mut().enumerate().fold(Scrollable::new(prefered_lang_scroll).height(Length::Fill).padding(7).spacing(4), |scroll, (idx, (title, subtitle, state))| {
               let content = Column::new().spacing(4)
                  .push(Text::new(title.as_str()))
                  .push(Text::new(subtitle.as_str()).size(12).color(HOVERED));
               let mut btn = Button::new(state, content).width(Length::Fill).on_press(LangRegionMessage::LangSelected(idx));
               btn = if let Some(selected_idx) = selected_lang {
                  btn.style(if *selected_idx == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
               } else {
                  btn.style(CustomButton::Sidebar)
               };
               scroll.push(btn)
            });
            
            let left_pane = Container::new(
               Row::new().spacing(10).align_items(Align::Center)
               .push(
                  Column::new().spacing(10)
                  .push(lb_prefered_lang)
                  .push(
                     Container::new(
                        Column::new()
                        .push(lang_group)
                        .push(btn_group)
                     ).height(Length::Fill).style(CustomContainer::ForegroundWhite)
                  )
               )
               // .push(btn_shift_group)
            ).width(Length::FillPortion(3));

            // ផ្ទាំងខាងស្ដាំ
               // ផ្នែកស្លាក
            // let lb_region = Text::new("Region:");
            let lb_first_day = Text::new("First day of week:");
            let lb_time_format = Text::new("Time Format:");
            let lb_temp = Text::new("Temperature:");
            let lb_num_sep = Text::new("Number Seperators:");
            let lb_currency_sep = Text::new("Currency:");
            let label_sec = Container::new(
               Column::new().spacing(20).align_items(Align::End)
               // .push(lb_region)
               .push(lb_first_day)
               .push(lb_time_format)
               .push(lb_temp)
               .push(lb_num_sep)
               .push(lb_currency_sep)
            ).width(Length::FillPortion(3)).align_x(Align::End);

               // ផ្នែកព័ត៌មាន
            // let pl_region = PickList::new(region_state, &SystemLocale::available_names(), Some(*selected_region), LangRegionMessage::RegionChanged);
            let pl_first_day = PickList::new(firstday_state, &Weekdays::ALL[..], Some(*selected_firstday), LangRegionMessage::FirstDayChanged).style(CustomSelect::Primary);
            let chb_time_format = Checkbox::new(*is_24_hours_format, "24-Hours Format", LangRegionMessage::TimeFormatToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_temp = PickList::new(temp_state, &Temperature::ALL[..], Some(*selected_temp), LangRegionMessage::TempChanged).style(CustomSelect::Primary);
            let lb_num_sep_grouping = Text::new("Grouping:");
            let pl_num_sep_grouping = PickList::new(num_sep_grouping, &GroupSeperator::ALL[..], Some(*selected_num_sep_grouping), LangRegionMessage::NumSepGroupingChanged).style(CustomSelect::Primary);
            let lb_num_sep_decimal = Text::new("Decimal:");
            let pl_num_sep_decimal = PickList::new(num_sep_decimal, &DecimalSeperator::ALL[..], Some(*selected_num_sep_decimal), LangRegionMessage::NumSepDecimalChanged).width(Length::Units(65)).style(CustomSelect::Primary);
            let num_sep_row = Row::new().spacing(10).align_items(Align::Center)
               .push(lb_num_sep_grouping)
               .push(pl_num_sep_grouping)
               .push(lb_num_sep_decimal)
               .push(pl_num_sep_decimal);
            let lb_currency_sep_grouping = Text::new("Grouping:");
            let pl_currency_sep_grouping = PickList::new(currency_sep_grouping, &GroupSeperator::ALL[..], Some(*selected_currency_sep_grouping), LangRegionMessage::CurrencySepGroupingChanged).style(CustomSelect::Primary);
            let lb_currency_sep_decimal = Text::new("Decimal:");
            let pl_currency_sep_decimal = PickList::new(currency_sep_decimal, &DecimalSeperator::ALL[..], Some(*selected_currency_sep_decimal), LangRegionMessage::CurrencySepDecimalChanged).width(Length::Units(65)).style(CustomSelect::Primary);
            let currency_sep_row = Row::new().spacing(10).align_items(Align::Center)
               .push(lb_currency_sep_grouping)
               .push(pl_currency_sep_grouping)
               .push(lb_currency_sep_decimal)
               .push(pl_currency_sep_decimal);
            let info_sec = Container::new(
               Column::new().spacing(12)
               // .push(pl_region)
               .push(pl_first_day)
               .push(chb_time_format)
               .push(pl_temp)
               .push(num_sep_row)
               .push(currency_sep_row)
            ).width(Length::FillPortion(7));

            let number_format = CustomFormat::builder()
               .grouping(Grouping::Standard)
               .decimal(selected_num_sep_decimal.as_str())
               .separator(selected_num_sep_grouping.as_str())
               .build().unwrap();

            let mut buf_number_formatted = Buffer::new();
            buf_number_formatted.write_formatted(number, &number_format);

            let currency_format = CustomFormat::builder()
               .grouping(Grouping::Standard)
               .decimal(selected_currency_sep_decimal.as_str())
               .separator(selected_currency_sep_grouping.as_str())
               .build().unwrap();

            let mut buf_currency_formatted = Buffer::new();
            buf_currency_formatted.write_formatted(currency, &currency_format);

            let right_pane = Container::new(
               Column::new().spacing(15).align_items(Align::Center)
               .push(
                  Row::new().spacing(10).align_items(Align::Center)
                  .push(label_sec)
                  .push(info_sec)
               )
               .push(
                  Container::new(
                     Column::new().spacing(10)
                     .push(Text::new(format!("{} at {}", now.format(long_date_format.as_str()), now.format(long_time_format.as_str()))).horizontal_alignment(HorizontalAlignment::Center))
                     .push(Text::new(format!("{}, {}  {}  {}$", now.format(short_date_format.as_str()), now.format(short_time_format.as_str()), buf_number_formatted.as_str(), buf_currency_formatted.as_str())).horizontal_alignment(HorizontalAlignment::Center))
                  ).height(Length::Fill).center_y()
               )
            ).width(Length::FillPortion(7));

            Container::new(
               Row::new().spacing(20)
               .push(left_pane)
               .push(right_pane)
            ).width(Length::Fill).height(Length::Fill)
         }
         1 => {
            let FormatsTab {
               short_date_format,
               selected_short_date_format,
               long_date_format,
               selected_long_date_format,
               short_time_format,
               selected_short_time_format,
               long_time_format,
               selected_long_time_format,
               now,
            } = formats_tab;

            // ផ្នែកស្លាក
            let lb_short_date = Text::new("Short Date:");
            let lb_long_date = Text::new("Long Date:");
            let lb_short_time = Text::new("Short Time:");
            let lb_long_time = Text::new("Long Time:");
            let label_sec = Container::new(
               Column::new().spacing(20)
               .push(lb_short_date)
               .push(lb_long_date)
               .push(lb_short_time)
               .push(lb_long_time)
            );

            // ផ្នែកព័ត៌មាន
            let pl_short_date = PickList::new(short_date_format, &DateFormat::ALL[..], Some(*selected_short_date_format), LangRegionMessage::ShortDateFormatChanged).style(CustomSelect::Primary);
            let pl_long_date = PickList::new(long_date_format, &DateFormat::ALL[..], Some(*selected_long_date_format), LangRegionMessage::LongDateFormatChanged).style(CustomSelect::Primary);
            let pl_short_time = PickList::new(short_time_format, &TimeFormat::ALL[..], Some(*selected_short_time_format), LangRegionMessage::ShortTimeFormatChanged).style(CustomSelect::Primary);
            let pl_long_time = PickList::new(long_time_format, &TimeFormat::ALL[..], Some(*selected_long_time_format), LangRegionMessage::LongTimeFormatChanged).style(CustomSelect::Primary);
            let info_sec = Container::new(
               Column::new().spacing(12)
               .push(pl_short_date)
               .push(pl_long_date)
               .push(pl_short_time)
               .push(pl_long_time)
            );

            let top_section = Container::new(
               Row::new().spacing(70).align_items(Align::Center)
               .push(label_sec)
               .push(info_sec)
            );

            // ផ្នែកមើលជាមុន
            let lb_preview = Text::new("Preview");

            // ផ្នែកស្លាក
            let lb_short_date = Text::new("Short Date:");
            let lb_long_date = Text::new("Long Date:");
            let lb_short_time = Text::new("Short Time:");
            let lb_long_time = Text::new("Long Time:");
            let label_preview_sec = Container::new(
               Column::new().spacing(20)
               .push(lb_short_date)
               .push(lb_long_date)
               .push(lb_short_time)
               .push(lb_long_time)
            );

            // ផ្នែកព័ត៌មាន
            let preview_short_date = Text::new(now.format(selected_short_date_format.as_str()).to_string());
            let preview_long_date = Text::new(now.format(selected_long_date_format.as_str()).to_string());
            let preview_short_time = Text::new(now.format(selected_short_time_format.as_str()).to_string());
            let preview_long_time = Text::new(now.format(selected_long_time_format.as_str()).to_string());
            let info_preview_sec = Container::new(
               Column::new().spacing(20)
               .push(preview_short_date)
               .push(preview_long_date)
               .push(preview_short_time)
               .push(preview_long_time)
            );
            let preview_sec = Container::new(
               Row::new().spacing(50)
               .push(label_preview_sec)
               .push(info_preview_sec)
            ).padding(20).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundWhite).center_y();

            Container::new(
               Column::new().spacing(20)
               .push(top_section)
               .push(
                  Column::new().spacing(10)
                  .push(lb_preview)
                  .push(preview_sec)
               )
            ).width(Length::Fill).height(Length::Fill)
         }
         2 => {
            let AppsTab {
               app_list,
               selected_app,
               add_state,
               remove_state,
               scroll,
            } = apps_tab;

            let lb_customize = Text::new("Customize language settings for the apps below:");
            let btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(LangRegionMessage::BtnAddAppClicked).style(CustomButton::Text);
            let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
            if selected_app.is_some() && app_list.len() > 1 {
               btn_remove = btn_remove.on_press(LangRegionMessage::BtnRemoveAppClicked);
            }
            let btn_group = Container::new(
               Row::new().push(btn_add).push(btn_remove)
            );

            let apps_group = app_list.iter_mut().enumerate().fold(Scrollable::new(scroll).height(Length::Fill).width(Length::Fill).padding(7).spacing(4), |scroll, (idx, (icon, title, pl_state, selected_lang, state))| {
               let content = Row::new().spacing(7).padding(4).align_items(Align::Center)
                  .push(IconBrand::new(*icon).size(30))
                  .push(Text::new(title.as_str()))
                  .push(Space::with_width(Length::Fill))
                  .push(PickList::new(pl_state, &AppLang::ALL[..], Some(*selected_lang), LangRegionMessage::AppLangChanged).style(CustomSelect::Primary))
                  .push(Button::new(state, Icon::new('\u{f138}').size(20)).padding(2).on_press(LangRegionMessage::AppSelected(idx)).style(CustomButton::Text));
               let mut con = Container::new(content).width(Length::Fill);
               con = if let Some(selected_idx) = selected_app {
                  con.style(if *selected_idx == idx {CustomContainer::FadedBrightForeground} else {CustomContainer::ForegroundWhite})
               } else {
                  con.style(CustomContainer::ForegroundWhite)
               };
               scroll.push(con)
            });

            Container::new(
               Column::new().spacing(10)
               .push(lb_customize)
               .push(
                  Container::new(apps_group).height(Length::Fill).padding(7).style(CustomContainer::ForegroundWhite)
               )
               .push(btn_group)
            ).width(Length::Fill).height(Length::Fill)
         }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // ផ្នែកខាងក្រោម
      let btn_defaults = Button::new(defaults_state, Text::new("  Defaults  ")).on_press(LangRegionMessage::DefaultsClicked).style(CustomButton::Default);
      let mut btn_cancel = Button::new(cancel_state, Text::new("  Cancel  ")).style(CustomButton::Hovered);
      let mut btn_apply = Button::new(appply_state, Text::new("  Apply  ")).style(CustomButton::Primary);
      if *is_changed {
         btn_apply = btn_apply.on_press(LangRegionMessage::ApplyClicked);
         btn_cancel = btn_cancel.on_press(LangRegionMessage::CancelClicked);
      }

      let bottom_sec = Container::new(
         Row::new().padding(15).spacing(10).align_items(Align::Center)
         .push(btn_defaults)
         .push(Space::with_width(Length::Fill))
         .push(btn_cancel)
         .push(btn_apply)
      ).width(Length::Fill).align_x(Align::End);

      // មាតិកា
      let content = Column::new().width(Length::Fill)
         .push(header_sec)
         .push(tabbar_sec)
         .push(tabview.height(Length::Fill).padding(25).style(CustomContainer::ForegroundGray))
         .push(bottom_sec);

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone)]
struct GeneralTab {
   prefered_langs: Vec<(String, String, button::State)>,
   selected_lang: Option<usize>,
   prefered_lang_scroll: scrollable::State,
   add_state: button::State, 
   remove_state: button::State, 
   up_state: button::State,
   down_state: button::State,
   region_state: pick_list::State<SystemLocale>,
   selected_region: SystemLocale,
   firstday_state: pick_list::State<Weekdays>,
   selected_firstday: Weekdays,
   is_24_hours_format: bool,
   temp_state: pick_list::State<Temperature>,
   selected_temp: Temperature,
   num_sep_grouping: pick_list::State<GroupSeperator>,
   selected_num_sep_grouping: GroupSeperator,
   num_sep_decimal: pick_list::State<DecimalSeperator>,
   selected_num_sep_decimal: DecimalSeperator,
   currency_sep_grouping: pick_list::State<GroupSeperator>,
   selected_currency_sep_grouping: GroupSeperator,
   currency_sep_decimal: pick_list::State<DecimalSeperator>,
   selected_currency_sep_decimal: DecimalSeperator,
   now: DateTime<Local>,
   number: i32,
   currency: i32,
}

#[derive(Debug, Clone)]
struct FormatsTab {
   short_date_format: pick_list::State<DateFormat>,
   selected_short_date_format: DateFormat,
   long_date_format: pick_list::State<DateFormat>,
   selected_long_date_format: DateFormat,
   short_time_format: pick_list::State<TimeFormat>,
   selected_short_time_format: TimeFormat,
   long_time_format: pick_list::State<TimeFormat>,
   selected_long_time_format: TimeFormat,
   now: DateTime<Local>,
}

#[derive(Debug, Clone, Default)]
struct AppsTab {
   app_list: Vec<(char, String, pick_list::State<AppLang>, AppLang, button::State)>,
   selected_app: Option<usize>,
   add_state: button::State,
   remove_state: button::State,
   scroll: scrollable::State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum Weekdays {
   #[default]
   Sun,
   Mon,
   Tue,
   Wed,
   Thu,
   Fri,
   Sat
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum Temperature {
   #[default]
   Celsius,
   Fahrenheit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum GroupSeperator {
   #[default]
   Dot,
   Comma,
   Space,
   None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum DecimalSeperator {
   Dot,
   #[default]
   Comma,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum DateFormat {
   #[default]
   _dmy,
   _dMy,
   _dMMMMy,
   _dMMMMyyyy,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum TimeFormat {
   #[default]
   _hma,
   _Hm,
   _hmsa,
   _Hms,
   _hmsaz,
   _Hmsz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum AppLang {
   #[default]
   Sys,
   En,
   Kh,
   Ch,
   Fr,
   Sp,
   Arabic
}

impl GeneralTab {
   pub fn new() -> Self {
      Self {
         prefered_langs: vec![
            ("Khmer".to_string(), "Khmer".to_string(), button::State::new()),
            ("English".to_string(), "English".to_string(), button::State::new()),
         ],
         selected_lang: None,
         prefered_lang_scroll: scrollable::State::new(),
         add_state: button::State::new(), 
         remove_state: button::State::new(), 
         up_state: button::State::new(),
         down_state: button::State::new(), 
         region_state: pick_list::State::default(),
         selected_region: SystemLocale::default().unwrap(),
         firstday_state: pick_list::State::default(),
         selected_firstday: Weekdays::default(),
         is_24_hours_format: false,
         temp_state: pick_list::State::default(),
         selected_temp: Temperature::default(),
         num_sep_grouping: pick_list::State::default(),
         selected_num_sep_grouping: GroupSeperator::default(),
         num_sep_decimal: pick_list::State::default(),
         selected_num_sep_decimal: DecimalSeperator::default(),
         currency_sep_grouping: pick_list::State::default(),
         selected_currency_sep_grouping: GroupSeperator::default(),
         currency_sep_decimal: pick_list::State::default(),
         selected_currency_sep_decimal: DecimalSeperator::default(),
         now: Local::now(),
         number: 12345678,
         currency: 4567890,
      }
   }
}

impl FormatsTab {
   pub fn new() -> Self {
      Self {
         short_date_format: pick_list::State::default(),
         selected_short_date_format: DateFormat::_dmy,
         long_date_format: pick_list::State::default(),
         selected_long_date_format: DateFormat::_dMMMMyyyy,
         short_time_format: pick_list::State::default(),
         selected_short_time_format: TimeFormat::_hma,
         long_time_format: pick_list::State::default(),
         selected_long_time_format: TimeFormat::_hmsaz,
         now: Local::now(),
      }
   }
}

impl AppsTab {
   pub fn new() -> Self {
      Self::default()
   }
}

impl Weekdays {
   const ALL:[Weekdays; 7] = [
      Weekdays::Sun,
      Weekdays::Mon,
      Weekdays::Tue,
      Weekdays::Wed,
      Weekdays::Thu,
      Weekdays::Fri,
      Weekdays::Sat,
   ];
}

impl Display for Weekdays {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}", match self {
         Weekdays::Sun => "Sunday",
         Weekdays::Mon => "Monday",
         Weekdays::Tue => "Tuesday",
         Weekdays::Wed => "Wednesday",
         Weekdays::Thu => "Thursday",
         Weekdays::Fri => "Friday",
         Weekdays::Sat => "Saturday",
      })
   }
}

impl Temperature {
   const ALL: [Temperature; 2] = [
      Temperature::Celsius,
      Temperature::Fahrenheit
   ];
}

impl Display for Temperature {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}", match self {
         Temperature::Celsius => "°C - Celsius",
         Temperature::Fahrenheit => "°F - Fahrenheit"
      })
   }
}

impl GroupSeperator {
   const ALL: [GroupSeperator; 4] = [
      GroupSeperator::Dot,
      GroupSeperator::Comma,
      GroupSeperator::Space,
      GroupSeperator::None
   ];

   pub fn as_str(&self) -> &'static str {
      use GroupSeperator::*;
      match self {
         Dot => ".",
         Comma => ",",
         Space => " ",
         None => ""
      }
   }
}

impl Display for GroupSeperator {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      use GroupSeperator::*;
      write!(f, "{}", match self {
         Dot => ".",
         Comma => ",",
         Space => "Space",
         None => "None"
      })
   }
}

impl DecimalSeperator {
   const ALL:[DecimalSeperator; 2] = [
      DecimalSeperator::Dot,
      DecimalSeperator::Comma,
   ];

   pub fn as_str(&self) -> &'static str {
      use DecimalSeperator::*;
      match self {
         Dot => ".",
         Comma => ",",
      }
   }
}

impl Display for DecimalSeperator {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      use DecimalSeperator::*;
      write!(f, "{}", match self {
         Dot => ".",
         Comma => ","
      })
   }
}

impl DateFormat {
   const ALL:[DateFormat; 4] = [
      DateFormat::_dmy,
      DateFormat::_dMy,
      DateFormat::_dMMMMy,
      DateFormat::_dMMMMyyyy,
   ];

   pub fn as_str(&self) -> &'static str {
      use DateFormat::*;
      match self {
         _dmy => "%e/%m/%y",
         _dMy => "%e/%b/%y",
         _dMMMMy => "%e/%B/%y",
         _dMMMMyyyy => "%e/%B/%Y"
      }
   }
}

impl Display for DateFormat {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      use DateFormat::*;
      write!(f, "{}", match self {
         _dmy => "d/m/y",
         _dMy => "d/M/y",
         _dMMMMy => "d/MMMM/y",
         _dMMMMyyyy => "d/MMMM/yyyy"
      })
   }
}

impl TimeFormat {
   const ALL:[TimeFormat; 6] = [
      TimeFormat::_hma,
      TimeFormat::_Hm,
      TimeFormat::_hmsa,
      TimeFormat::_Hms,
      TimeFormat::_hmsaz,
      TimeFormat::_Hmsz,
   ];

   pub fn as_str(&self) -> &'static str {
      use TimeFormat::*;
      match self {
         _hma => "%I:%M %p",
         _Hm => "%R",
         _hmsa => "%r",
         _Hms => "%X",
         _hmsaz => "%I:%M:%S %p %Z",
         _Hmsz => "%H:%M:%S %Z",
      }
   }
}

impl Display for TimeFormat {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      use TimeFormat::*;
      write!(f, "{}", match self {
         _hma => "h:m A",
         _Hm => "H:m",
         _hmsa => "h:m:s A",
         _Hms => "H:m:s",
         _hmsaz => "h:m:s A Z",
         _Hmsz => "H:m:s Z",
      })
   }
}

impl AppLang {
   const ALL:[AppLang; 7] = [
      AppLang::Sys,
      AppLang::En,
      AppLang::Kh,
      AppLang::Ch,
      AppLang::Fr,
      AppLang::Sp,
      AppLang::Arabic
   ];
}

impl Display for AppLang {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      use AppLang::*;
      write!(f, "{}", match self {
         Sys => "System Language",
         En => "English",
         Kh => "Khmer",
         Ch => "Chinese",
         Fr => "French",
         Sp => "Spanish",
         Arabic => "Arabic"
      })
   }
}