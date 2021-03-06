use super::lang_region_utils::*;
use crate::gui::addon_widgets::{icon_btn, tabbar};
use crate::gui::styles::{CustomButton, CustomCheckbox, CustomContainer, CustomSelect, CustomTextInput, HOVERED};
use crate::helpers::ROOT_PATH;
use iced::{button, pick_list, Align, Button, Checkbox, Column, Container, Element, Length, PickList, Row, Scrollable, Space, Svg, Text, TextInput};
use iced_custom_widget::{Icon, IconBrand, IconBrands, Icons};
use libkoompi::system_settings::locale::{LC_Keywords, LocaleManager, LS_MEASURE_UNITS};
use num_format::{Buffer, CustomFormat, Grouping};
use std::collections::HashMap;
use tauri_dialog::{DialogBuilder, DialogButtons, DialogSelection, DialogStyle};

#[derive(Debug, Clone)]
pub enum LangRegionMessage {
   TabChanged(usize),
   BtnAddClicked,
   BtnRemoveClicked,
   BtnUpClicked,
   BtnDownClicked,
   LangSelected(usize),
   RegionChanged(LCKeyVal),
   FirstDayChanged(String),
   TimeChanged(LCKeyVal),
   TimeFormatToggled(bool),
   NumFormatChanged(LCKeyVal),
   CurrencyFormatChanged(LCKeyVal),
   MeasureFormatChanged(LCKeyVal),
   BtnAddAppClicked,
   BtnRemoveAppClicked,
   AppSelected(usize),
   AppLangChanged(String),
   DefaultsClicked,
   OKClicked,
   ResetClicked,
   AddLangMsg(AddLangMessage),
}

#[derive(Debug, Clone)]
pub enum AddLangMessage {
   SearchPreferedLang(String),
   AddLangChanged(PreferedLang),
   AddClicked,
   CancelClicked,
}

#[derive(Debug, Default)]
pub struct LangRegionPage {
   locale_mn: LocaleManager,
   tabbar_state: Vec<(&'static str, button::State)>,
   current_tab_idx: usize,
   general_tab: GeneralTab,
   apps_tab: AppsTab,
   btn_defaults_state: button::State,
   btn_reset_state: button::State,
   btn_ok_state: button::State,
   is_changed: bool,
}

impl LangRegionPage {
   pub fn new() -> Self {
      let tabs = vec![("General", button::State::new()), ("Apps", button::State::new())];

      match LocaleManager::new() {
         Ok(locale_mn) => {
            let mut ls_lang_regions = locale_mn
               .list_langs_regions()
               .iter()
               .map(|(key, lang_reg)| {
                  let ls_lang = lang_reg.split("(").collect::<Vec<&str>>()[0].split("—").map(|i| i.trim().to_string()).collect::<Vec<String>>();
                  let lang = ls_lang.first().unwrap();
                  if let Some(reg) = ls_lang.last() {
                     PreferedLang::new(key, lang, reg)
                  } else {
                     PreferedLang::new(key, lang, lang)
                  }
               })
               .collect::<Vec<PreferedLang>>();
            ls_lang_regions.sort();
            let ls_prefered_langs = locale_mn
               .list_prefered_langs()
               .iter()
               .map(|(key, lang_reg)| {
                  let ls_lang = lang_reg.split("—").map(|i| i.trim().to_string()).collect::<Vec<String>>();
                  let lang = ls_lang.first().unwrap();
                  if let Some(reg) = ls_lang.last() {
                     PreferedLang::new(key, lang, reg)
                  } else {
                     PreferedLang::new(key, lang, lang)
                  }
               })
               .collect::<Vec<PreferedLang>>();
            let ls_add_prefered_lang = ls_lang_regions
               .iter()
               .filter(|lang| !ls_prefered_langs.contains(lang))
               .map(|lang| (lang.clone(), button::State::new()))
               .collect::<Vec<(PreferedLang, button::State)>>();
            let first_day = Self::get_first_day(&locale_mn);
            let region = LCKeyVal::new(locale_mn.language());
            let time_format = LCKeyVal::new(locale_mn.time());
            let num_format = LCKeyVal::new(locale_mn.numeric());
            let currency_format = LCKeyVal::new(locale_mn.monetary());
            let measure_units = LCKeyVal::new(LS_MEASURE_UNITS[locale_mn.measurement_details().measurement - 1]);

            Self {
               locale_mn,
               tabbar_state: tabs,
               general_tab: GeneralTab {
                  prefered_langs: ls_prefered_langs.iter().map(|lang| (lang.clone(), button::State::new())).collect(),
                  selected_firstday: Some(first_day),
                  selected_region: Some(region),
                  selected_time_format: Some(time_format),
                  selected_num_format: Some(num_format),
                  selected_currency_format: Some(currency_format),
                  selected_measure_format: Some(measure_units),
                  add_langs: ls_add_prefered_lang.clone(),
                  filtered_add_langs: ls_add_prefered_lang.clone(),
                  ..GeneralTab::default()
               },
               ..Self::default()
            }
         }
         Err(err) => {
            eprintln!("{}", err); // error handling here
            Self { tabbar_state: tabs, ..Self::default() }
         }
      }
   }

   pub fn update(&mut self, msg: LangRegionMessage) {
      use LangRegionMessage::*;
      match msg {
         TabChanged(idx) => self.current_tab_idx = idx,
         BtnAddClicked => {
            if !self.general_tab.is_adding {
               self.general_tab.is_adding = true;
            }
         }
         BtnRemoveClicked => {
            if let Some(selected_idx) = self.general_tab.selected_lang {
               self.general_tab.prefered_langs.remove(selected_idx);
               self.general_tab.selected_lang = None;
               match self.locale_mn.set_locale(LC_Keywords::LANGUAGE, &self.general_tab.get_formatted_prefered_lang()) {
                  Ok(()) => self.is_changed = true,
                  Err(err) => eprintln!("{:?}", err),
               }
            }
         }
         BtnUpClicked => {
            if let Some(selected_idx) = self.general_tab.selected_lang {
               self.general_tab.prefered_langs.swap(selected_idx, selected_idx - 1);
               self.general_tab.selected_lang = Some(selected_idx - 1);
               match self.locale_mn.set_locale(LC_Keywords::LANGUAGE, &self.general_tab.get_formatted_prefered_lang()) {
                  Ok(()) => self.is_changed = true,
                  Err(err) => eprintln!("{:?}", err),
               }
            }
         }
         BtnDownClicked => {
            if let Some(selected_idx) = self.general_tab.selected_lang {
               self.general_tab.prefered_langs.swap(selected_idx, selected_idx + 1);
               self.general_tab.selected_lang = Some(selected_idx + 1);
               match self.locale_mn.set_locale(LC_Keywords::LANGUAGE, &self.general_tab.get_formatted_prefered_lang()) {
                  Ok(()) => self.is_changed = true,
                  Err(err) => eprintln!("{:?}", err),
               }
            }
         }
         LangSelected(idx) => self.general_tab.selected_lang = Some(idx),
         RegionChanged(val) => {
            self.general_tab.selected_region = Some(val.clone());
            match self.locale_mn.set_locale(LC_Keywords::LANG, &val.key) {
               Ok(()) => self.is_changed = true,
               Err(err) => eprintln!("{:?}", err),
            }
         }
         FirstDayChanged(val) => {
            self.general_tab.selected_firstday = Some(val);
            self.is_changed = true;
         }
         TimeChanged(val) => {
            self.general_tab.selected_time_format = Some(val.clone());
            match self.locale_mn.set_locale(LC_Keywords::LC_TIME, &val.key) {
               Ok(()) => {
                  self.general_tab.selected_firstday = Some(Self::get_first_day(&self.locale_mn));
                  self.is_changed = true
               }
               Err(err) => eprintln!("{:?}", err),
            }
         }
         TimeFormatToggled(is_checked) => {
            self.general_tab.is_24_hours_format = is_checked;
            self.is_changed = true;
         }
         NumFormatChanged(val) => {
            self.general_tab.selected_num_format = Some(val.clone());
            match self.locale_mn.set_locale(LC_Keywords::LC_NUMERIC, &val.key) {
               Ok(()) => self.is_changed = true,
               Err(err) => eprintln!("{:?}", err),
            }
         }
         CurrencyFormatChanged(val) => {
            self.general_tab.selected_currency_format = Some(val.clone());
            match self.locale_mn.set_locale(LC_Keywords::LC_MONETARY, &val.key) {
               Ok(()) => self.is_changed = true,
               Err(err) => eprintln!("{:?}", err),
            }
         }
         MeasureFormatChanged(val) => {
            let map_measurement: HashMap<String, String> = LS_MEASURE_UNITS.iter().map(|(key, lang)| (key.to_string(), lang.to_string())).collect();
            self.general_tab.selected_measure_format = Some(val);
            match self
               .locale_mn
               .set_locale(LC_Keywords::LC_MEASUREMENT, map_measurement.get_key_value(&self.general_tab.selected_measure_format.clone().unwrap().key).unwrap().0)
            {
               Ok(()) => self.is_changed = true,
               Err(err) => eprintln!("{:?}", err),
            }
         }
         BtnAddAppClicked => {
            self.apps_tab.app_list.push((Icons::Terminal, String::from("Terminal"), pick_list::State::default(), String::from("terminal"), button::State::new()));
            self.is_changed = true;
         }
         BtnRemoveAppClicked => {
            if let Some(selected_idx) = self.apps_tab.selected_app {
               self.apps_tab.app_list.remove(selected_idx);
               self.apps_tab.selected_app = None;
               self.is_changed = true;
            }
         }
         AppSelected(idx) => self.apps_tab.selected_app = Some(idx),
         AppLangChanged(val) => {
            if let Some(selected_idx) = self.apps_tab.selected_app {
               self.apps_tab.app_list.get_mut(selected_idx).unwrap().3 = val;
               self.is_changed = true;
            }
         }
         OKClicked => {
            match self.locale_mn.write_conf() {
               Ok(()) => {
                  DialogBuilder::new()
                     .message("These changes will take effect after the next login.")
                     .title("Note")
                     .style(DialogStyle::Info)
                     .buttons(DialogButtons::Ok)
                     .build()
                     .show();
               }
               Err(err) => {
                  let res = DialogBuilder::new().message(format!("{}", err).as_str()).title("Error").style(DialogStyle::Error).buttons(DialogButtons::Quit).build().show();
                  if res == DialogSelection::Quit {
                     std::process::exit(1);
                  }
               }
            }
            self.is_changed = false;
         }
         ResetClicked | DefaultsClicked => {
            // match self.current_tab_idx {
            //    0 => self.general_tab = GeneralTab::default(),
            //    // 1 => self.formats_tab = FormatsTab::default(),
            //    1 => self.apps_tab = AppsTab::default(),
            //    _ => {},
            // }
            // self.is_changed = false;
            *self = Self::new();
         }
         AddLangMsg(add_lang_msg) => match add_lang_msg {
            AddLangMessage::SearchPreferedLang(val) => {
               self.general_tab.search_prefered_lang_val = val;
               self.filter_ls_add_lang();
            }
            AddLangMessage::AddLangChanged(val) => self.general_tab.selected_add_lang = Some(val),
            AddLangMessage::AddClicked => {
               if let Some(selected) = &self.general_tab.selected_add_lang {
                  self.general_tab.prefered_langs.push((selected.clone(), button::State::new()));
                  self.general_tab.add_langs.retain(|lang| lang.0.key != selected.key);
                  self.filter_ls_add_lang();
                  self.general_tab.selected_add_lang = None;
                  self.general_tab.is_adding = false;
                  match self.locale_mn.set_locale(LC_Keywords::LANGUAGE, &self.general_tab.get_formatted_prefered_lang()) {
                     Ok(()) => self.is_changed = true,
                     Err(err) => eprintln!("{:?}", err),
                  }
               }
            }
            AddLangMessage::CancelClicked => {
               self.general_tab.selected_add_lang = None;
               self.general_tab.is_adding = false;
            }
         },
      }
   }

   pub fn view(&mut self) -> Element<LangRegionMessage> {
      use LangRegionMessage::*;
      let LangRegionPage {
         locale_mn,
         tabbar_state,
         current_tab_idx,
         general_tab,
         apps_tab,
         btn_defaults_state,
         btn_reset_state,
         btn_ok_state,
         is_changed,
         // formats_tab,
      } = self;

      // ផ្នែកក្បាល
      let icon = Svg::from_path(format!("{}/assets/images/language.svg", ROOT_PATH())).width(Length::Units(75)).height(Length::Units(75));
      let txt_lang = Text::new("Language & Region preferences control the language you see in menus and dialogs, formats of dates, times, numbers and currency.");
      let header_sec = Container::new(Row::new().spacing(20).align_items(Align::Center).push(icon).push(txt_lang));

      // របារផ្ទាំង
      let tabbar_sec = tabbar(tabbar_state, *current_tab_idx, TabChanged);

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
               time_format,
               selected_time_format,
               is_24_hours_format,
               num_format,
               selected_num_format,
               currency_format,
               selected_currency_format,
               measure_format,
               selected_measure_format,
               content_scroll,
               now,
               number,
               currency,
               is_adding,
               filtered_add_langs,
               search_prefered_lang_state,
               search_prefered_lang_val,
               selected_add_lang,
               btn_okay_state,
               btn_cancel_state,
               ..
            } = general_tab;

            // ផ្ទាំងខាងឆ្វេង
            let lb_prefered_lang = Text::new("Preferred Languages:");
            let btn_add = Button::new(add_state, Icon::new(Icons::Ad).size(23)).padding(2).style(CustomButton::Text).on_press(BtnAddClicked);
            let mut btn_remove = Button::new(remove_state, Icon::new(Icons::RemoveUser).size(23)).padding(2).style(CustomButton::Text);
            if selected_lang.is_some() && prefered_langs.len() > 1 {
               btn_remove = btn_remove.on_press(BtnRemoveClicked);
            }

            let mut btn_up = Button::new(up_state, Icon::new(Icons::Upload).size(23)).padding(2).style(CustomButton::Hovered);
            let mut btn_down = Button::new(down_state, Icon::new(Icons::Download).size(23)).padding(2).style(CustomButton::Hovered);
            if let Some(selected_idx) = selected_lang {
               if *selected_idx != 0 {
                  btn_up = btn_up.on_press(BtnUpClicked);
               }
               if *selected_idx != (prefered_langs.len() - 1) {
                  btn_down = btn_down.on_press(BtnDownClicked);
               }
            }

            let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
            let btn_shift_group = Container::new(Column::new().spacing(10).push(btn_up).push(btn_down)).height(Length::Fill).center_y();

            let lang_group = prefered_langs.iter_mut().enumerate().fold(
               Scrollable::new(prefered_lang_scroll).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4),
               |scroll, (idx, (prefered_lang, state))| {
                  let content = Column::new()
                     .spacing(4)
                     .push(Text::new(format!("{} {}", prefered_lang.lang, if idx == 0 { "(Primary)" } else { "" })))
                     .push(Text::new(prefered_lang.reg.as_str()).size(12).color(HOVERED));
                  let mut btn = Button::new(state, content).width(Length::Fill).on_press(LangSelected(idx));
                  btn = if let Some(selected_idx) = selected_lang {
                     btn.style(if *selected_idx == idx { CustomButton::SelectedSidebar } else { CustomButton::Sidebar })
                  } else {
                     btn.style(CustomButton::Sidebar)
                  };
                  scroll.push(btn)
               },
            );
            let left_pane = Container::new(
               Row::new()
                  .spacing(10)
                  .align_items(Align::Center)
                  .push(
                     Container::new(Column::new().push(Container::new(lb_prefered_lang).padding(7).width(Length::Fill).style(CustomContainer::Header)).push(lang_group).push(btn_group))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(CustomContainer::ForegroundWhite),
                  )
                  .push(btn_shift_group),
            )
            .width(Length::FillPortion(3));

            // ផ្ទាំងខាងស្ដាំ
            let right_pane = if !(*is_adding) {
               // ផ្នែកស្លាក
               let lb_region = Text::new("Region:");
               let lb_first_day = Text::new("First day of week:");
               let lb_time = Text::new("Time:");
               let lb_time_format = Text::new("Time Format:");
               let lb_num = Text::new("Number:");
               let lb_currency = Text::new("Currency:");
               let lb_measure_unit = Text::new("Measurement Units:");
               let label_sec = Container::new(Column::new().spacing(20).push(lb_region).push(lb_first_day).push(lb_time).push(lb_time_format).push(lb_num).push(lb_currency).push(lb_measure_unit));

               // ផ្នែកព័ត៌មាន
               let mut ls_locales = locale_mn.list_langs_regions().iter().map(|item| LCKeyVal::new(item)).collect::<Vec<LCKeyVal>>();
               ls_locales.sort();
               let ls_days = locale_mn.time_details().list_days();
               let pl_region = PickList::new(region_state, ls_locales.clone(), selected_region.clone(), RegionChanged).style(CustomSelect::Primary);
               let pl_first_day = PickList::new(firstday_state, ls_days, selected_firstday.clone(), FirstDayChanged).style(CustomSelect::Primary);
               let pl_time = PickList::new(time_format, ls_locales.clone(), selected_time_format.clone(), TimeChanged).style(CustomSelect::Primary);
               let chb_time_format = Checkbox::new(*is_24_hours_format, "24-Hours Format", TimeFormatToggled).spacing(10).style(CustomCheckbox::Default);
               let pl_num_format = PickList::new(num_format, ls_locales.clone(), selected_num_format.clone(), NumFormatChanged).style(CustomSelect::Primary);
               let pl_currency_format = PickList::new(currency_format, ls_locales.clone(), selected_currency_format.clone(), CurrencyFormatChanged).style(CustomSelect::Primary);
               let pl_measure_units = PickList::new(
                  measure_format,
                  LS_MEASURE_UNITS.iter().map(|item| LCKeyVal::new(item.clone())).collect::<Vec<LCKeyVal>>().clone(),
                  selected_measure_format.clone(),
                  MeasureFormatChanged,
               )
               .style(CustomSelect::Primary);
               let info_sec = Container::new(
                  Column::new()
                     .spacing(10)
                     .push(pl_region)
                     .push(pl_first_day)
                     .push(pl_time)
                     .push(chb_time_format)
                     .push(pl_num_format)
                     .push(pl_currency_format)
                     .push(pl_measure_units),
               );

               let mut number_formatted = Buffer::new();
               let number_format = CustomFormat::builder()
                  .grouping(Grouping::Standard)
                  .decimal(locale_mn.numeric_details().decimal_point.as_str())
                  .separator(locale_mn.numeric_details().thousands_sep.as_str())
                  .build()
                  .unwrap();
               number_formatted.write_formatted(number, &number_format);

               let mut currency_formatted = Buffer::new();
               let currency_format = CustomFormat::builder()
                  .grouping(Grouping::Standard)
                  .decimal(locale_mn.monetary_details().mon_decimal_point.as_str())
                  .separator(locale_mn.monetary_details().mon_thousands_sep.as_str())
                  .build()
                  .unwrap();
               currency_formatted.write_formatted(currency, &currency_format);

               // ផ្នែកឧទាហរណ៍
               let lb_example = Text::new("Example:").size(15);
               let lb_full_time = Text::new("Full Time:");
               let lb_short_time = Text::new("Short Time:");
               let lb_first_day = Text::new("First day of week:");
               let lb_num = Text::new("Number:");
               let lb_currency = Text::new("Currency:");
               let lb_measure_unit = Text::new("Measurement Units:");
               let txt_full_time = Text::new(now.format(locale_mn.time_details().d_t_fmt.as_str()).to_string());
               let txt_short_time = Text::new(format!("{} {}", now.format(locale_mn.time_details().d_fmt.as_str()), now.format(locale_mn.time_details().t_fmt.as_str())));
               let txt_first_day = Text::new(Self::get_first_day(&locale_mn));
               let txt_num = Text::new(number_formatted.as_str());
               let txt_currency = Text::new(format!("{} {}", currency_formatted.as_str(), locale_mn.monetary_details().currency_symbol.as_str()));
               let txt_measure_unit = Text::new(LS_MEASURE_UNITS.get(locale_mn.measurement_details().measurement - 1).unwrap_or(&("", "")).1.clone());
               let label_txt = |label: Text, txt: Text| Row::new().spacing(10).push(label).push(txt);
               let example_sec = Container::new(
                  Column::new()
                     .spacing(15)
                     .push(lb_example)
                     .push(label_txt(lb_full_time, txt_full_time))
                     .push(label_txt(lb_short_time, txt_short_time))
                     .push(label_txt(lb_first_day, txt_first_day))
                     .push(label_txt(lb_num, txt_num))
                     .push(label_txt(lb_currency, txt_currency))
                     .push(label_txt(lb_measure_unit, txt_measure_unit)),
               )
               .width(Length::Fill);

               Container::new(
                  Scrollable::new(content_scroll)
                     .spacing(15)
                     .scroller_width(4)
                     .scrollbar_width(4)
                     .push(Row::new().spacing(10).align_items(Align::Center).push(label_sec).push(info_sec))
                     .push(example_sec),
               )
            } else {
               let scrollable_prefered_lang = filtered_add_langs
                  .iter_mut()
                  .fold(Scrollable::new(content_scroll).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (prefered_lang, state)| {
                     let btn = Button::new(state, Text::new(format!("{}", prefered_lang)))
                        .width(Length::Fill)
                        .on_press(AddLangMsg(AddLangMessage::AddLangChanged(prefered_lang.clone())))
                        .style(if let Some(selected) = selected_add_lang {
                           if selected == prefered_lang {
                              CustomButton::Selected
                           } else {
                              CustomButton::Text
                           }
                        } else {
                           CustomButton::Text
                        });
                     scrollable.push(btn)
                  });
               let prefered_lang = Container::new(
                  Column::new()
                     .push(Container::new(Text::new("Select a prefered language to add:")).width(Length::Fill).padding(7).style(CustomContainer::Header))
                     .push(scrollable_prefered_lang),
               )
               .height(Length::Fill)
               .style(CustomContainer::ForegroundWhite);
               let mut btn_add_lang = icon_btn(btn_okay_state, Icons::Ad, "Add", None).style(CustomButton::Primary);
               let btn_cancel = icon_btn(btn_cancel_state, Icons::ArrowLeft, "Cancel", None).on_press(AddLangMsg(AddLangMessage::CancelClicked)).style(CustomButton::Hovered);
               if selected_add_lang.is_some() {
                  btn_add_lang = btn_add_lang.on_press(AddLangMsg(AddLangMessage::AddClicked));
               }
               Container::new(
                  Row::new()
                     .push(
                        Column::new()
                           .width(Length::Fill)
                           .spacing(10)
                           .push(
                              TextInput::new(search_prefered_lang_state, "Type language name that's you wish to add", search_prefered_lang_val.as_str(), move |val| {
                                 AddLangMsg(AddLangMessage::SearchPreferedLang(val))
                              })
                              .padding(10)
                              .style(CustomTextInput::Default),
                           )
                           .push(prefered_lang)
                           .push(Row::new().spacing(10).align_items(Align::Center).push(Space::with_width(Length::Fill)).push(btn_cancel).push(btn_add_lang)),
                     )
                     .push(Space::with_width(Length::Units(10))),
               )
            };

            Container::new(
               Row::new().push(Space::with_width(Length::Units(15))).push(
                  Column::new()
                     .spacing(10)
                     .height(Length::Fill)
                     .push(Space::with_height(Length::Units(0)))
                     .push(Row::new().spacing(10).push(left_pane.width(Length::FillPortion(3))).push(right_pane.width(Length::FillPortion(7))))
                     .push(Space::with_height(Length::Units(15))),
               ),
            )
         }
         1 => {
            let AppsTab {
               app_list,
               selected_app,
               add_state,
               remove_state,
               scroll,
            } = apps_tab;

            let lb_customize = Text::new("Customize language settings for the apps below:");
            let btn_add = Button::new(add_state, Icon::new(Icons::Ad).size(23)).padding(2).on_press(BtnAddAppClicked).style(CustomButton::Text);
            let mut btn_remove = Button::new(remove_state, Icon::new(Icons::RemoveUser).size(23)).padding(2).style(CustomButton::Text);
            if selected_app.is_some() && app_list.len() > 1 {
               btn_remove = btn_remove.on_press(BtnRemoveAppClicked);
            }
            let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove));

            let ls_locales = locale_mn.list_langs_regions().iter().map(|(_, lang)| lang.to_string()).collect::<Vec<String>>();
            let apps_group = app_list.iter_mut().enumerate().fold(
               Scrollable::new(scroll).height(Length::Fill).width(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4),
               |scroll, (idx, (icon, title, pl_state, selected_lang, state))| {
                  let content = Row::new()
                     .spacing(7)
                     .padding(4)
                     .align_items(Align::Center)
                     .push(Icon::new(*icon).size(30))
                     .push(Text::new(title.as_str()))
                     .push(Space::with_width(Length::Fill))
                     .push(PickList::new(pl_state, ls_locales.clone(), Some(selected_lang.clone()), AppLangChanged).style(CustomSelect::Primary))
                     .push(Button::new(state, Icon::new(Icons::User).size(20)).padding(2).on_press(AppSelected(idx)).style(CustomButton::Text));
                  let mut con = Container::new(content).width(Length::Fill);
                  con = if let Some(selected_idx) = selected_app {
                     con.style(if *selected_idx == idx { CustomContainer::FadedBrightForeground } else { CustomContainer::ForegroundWhite })
                  } else {
                     con.style(CustomContainer::ForegroundWhite)
                  };
                  scroll.push(con)
               },
            );

            Container::new(
               Column::new()
                  .spacing(10)
                  .padding(15)
                  .push(lb_customize)
                  .push(Container::new(apps_group).height(Length::Fill).padding(7).style(CustomContainer::ForegroundWhite))
                  .push(btn_group),
            )
         }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // ផ្នែកខាងក្រោម
      let btn_defaults = icon_btn(btn_defaults_state, Icons::RemoveUser, "Defaults", None).on_press(DefaultsClicked).style(CustomButton::Default);
      let mut btn_reset = icon_btn(btn_reset_state, Icons::RegisteredTrademark, "Reset", None).style(CustomButton::Hovered);
      let mut btn_ok = icon_btn(btn_ok_state, Icons::CheckCircle, "OK", None).style(CustomButton::Primary);
      if *is_changed {
         btn_ok = btn_ok.on_press(OKClicked);
         btn_reset = btn_reset.on_press(ResetClicked);
      }

      let bottom_sec = Container::new(Row::new().padding(15).spacing(10).align_items(Align::Center).push(btn_defaults).push(btn_reset).push(Space::with_width(Length::Fill)).push(btn_ok))
         .width(Length::Fill)
         .align_x(Align::End);

      // មាតិកា
      let content = Column::new()
         .width(Length::Fill)
         .push(header_sec)
         .push(tabbar_sec)
         .push(tabview.width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundGray))
         .push(bottom_sec);

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

impl LangRegionPage {
   fn get_first_day(locale_mn: &LocaleManager) -> String {
      locale_mn.time_details().list_days().get((locale_mn.time_details().first_weekday - 1) as usize).unwrap_or(&String::from("")).clone()
   }

   fn filter_ls_add_lang(&mut self) {
      self.general_tab.filtered_add_langs = self
         .general_tab
         .add_langs
         .iter()
         .filter(|lang| lang.0.key.to_lowercase().contains(&self.general_tab.search_prefered_lang_val.to_lowercase()) || lang.0.to_string().to_lowercase().contains(&self.general_tab.search_prefered_lang_val.to_lowercase()))
         .cloned()
         .collect();
   }
}
