use iced::{
   pick_list, button, scrollable, Element, Align, Space, Length, Svg,
   Container, Checkbox, Row, Text, Button, Column, Scrollable, PickList,
};
use iced_custom_widget::Grid;
use super::super::styles::{CustomButton, CustomContainer, CustomCheckbox, CustomSelect};
use smart_default::SmartDefault;

#[derive(Debug, Clone)]
pub enum DesktopMessage {
   TabChanged(usize),
   CategoryFolderSelected(usize),
   WallpaperModeChanged(WallpaperType),
   WallpaperSelected(usize),
   ChangeWallpaperToggled(bool),
   ChangeWallpaperDurChanged(ChangeDuration),
   ScreenSaverSelected(usize),
   StartAfterDurChanged(ChangeDuration),
   ShowClockToggled(bool),
   RandomScreenSaverToggled(bool),
   RandomOrderToggled(bool),
}

#[derive(Debug, Clone)]
pub struct DesktopPage {
   tabbar_state: Vec<(&'static str, button::State)>,
   current_tab_idx: usize,
   desktop_tab: DesktopTab,
   screen_saver_tab: ScreenSaverTab,
}

impl DesktopPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  Desktop  ", button::State::new()),
            ("  Screen Saver  ", button::State::new()),
         ],
         current_tab_idx: 0,
         desktop_tab: DesktopTab::new(),
         screen_saver_tab: ScreenSaverTab::new(),
      }
   }

   pub fn update(&mut self, msg: DesktopMessage) {
      use DesktopMessage::*;
      match msg {
         TabChanged(idx) => self.current_tab_idx = idx,
         CategoryFolderSelected(idx) => self.desktop_tab.selected_category_folder = idx,
         WallpaperModeChanged(val) => {
            self.desktop_tab.categories_wallpaper_map.get_mut(self.desktop_tab.selected_category_folder).unwrap().get_mut(self.desktop_tab.selected_wallpaper).unwrap().kind = val;
         },
         WallpaperSelected(idx) => self.desktop_tab.selected_wallpaper = idx,
         ChangeWallpaperToggled(is_checked) => self.desktop_tab.change_wallpaper = is_checked,
         ChangeWallpaperDurChanged(val) => self.desktop_tab.selected_change_dur = val,
         ScreenSaverSelected(idx) => self.screen_saver_tab.selected_wallpaper = idx,
         StartAfterDurChanged(val) => self.screen_saver_tab.start_after_dur_val = val,
         ShowClockToggled(is_checked) => self.screen_saver_tab.show_clock = is_checked,
         RandomScreenSaverToggled(is_checked) => self.screen_saver_tab.random_screen_saver = is_checked,
         RandomOrderToggled(is_checked) => self.desktop_tab.random_order = is_checked,
      }
   }

   pub fn view(&mut self) -> Element<DesktopMessage> {
      let DesktopPage {
         tabbar_state,
         current_tab_idx,
         desktop_tab,
         screen_saver_tab,
      } = self;

      // របារផ្ទាំង
      let tabbar = tabbar_state.iter_mut().enumerate().fold(Row::new().spacing(2).align_items(Align::Center), |tabbar, (idx, (title, state))| {
         let btn = Button::new(state, Text::new(*title)).padding(5).on_press(DesktopMessage::TabChanged(idx)).style(if *current_tab_idx == idx {CustomButton::SelectedTab} else {CustomButton::Tab});
         tabbar.push(btn)
      }); 
      let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
      let tabbar_section = Container::new(tabbar_con).padding(7).width(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let DesktopTab {
               categories_folder,
               selected_category_folder,
               categories_wallpaper_map,
               selected_wallpaper,
               change_wallpaper,
               change_dur,
               selected_change_dur,
               random_order,
               category_folder_scroll,
               wallpaper_scroll,
            } = desktop_tab;

            // ផ្ទាំងខាងឆ្វេង
            let (default_category_col, photos_category_col, folders_category_col) = categories_folder.iter_mut().enumerate().fold((Column::new().padding(7).spacing(4), Column::new().padding(7).spacing(4), Column::new().padding(7).spacing(4)), |(default_col, photos_col, folders_col), (idx, (path, title, state, kind))| {
               let btn = Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Svg::from_path(format!("assets/images/{}.svg", *path)).width(Length::Units(20)).height(Length::Units(20))).push(Text::new(*title))).width(Length::Fill).on_press(DesktopMessage::CategoryFolderSelected(idx)).style(if *selected_category_folder == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar});
               
               match *kind {
                  CategoryFolder::Default => (default_col.push(btn), photos_col, folders_col),
                  CategoryFolder::Photos => (default_col, photos_col.push(btn), folders_col),
                  CategoryFolder::Folders => (default_col, photos_col, folders_col.push(btn))
               }
            });
            let category_folder_sec = Container::new(
               Scrollable::new(category_folder_scroll).padding(5)
               .push(Text::new("Default").size(12))
               .push(default_category_col)
               .push(Text::new("Photos").size(12))
               .push(photos_category_col)
               .push(Text::new("Folders").size(12))
               .push(folders_category_col)
            ).width(Length::FillPortion(3)).height(Length::Fill).style(CustomContainer::ForegroundWhite);
            
            // ផ្ទាំងខាងស្ដាំ
            let (current_wallpaper_row, wallpaper_grid_view) = categories_wallpaper_map.get_mut(*selected_category_folder).unwrap().iter_mut().enumerate().fold((Row::new().spacing(20).align_items(Align::Center), Grid::new().column_width(127).padding(10).spacing(10)), |(row, grid), (idx, wallpaper)| {
               let wallpaper_svg = Svg::from_path(format!("assets/images/wallpaper/{}.svg", wallpaper.path)).width(Length::Fill).height(Length::Units(55));

               (
                  if *selected_wallpaper == idx {
                     row
                     .push(Svg::from_path(format!("assets/images/wallpaper/{}.svg", wallpaper.path)).width(Length::Units(150)).height(Length::Units(100)))
                     .push(
                        Column::new().spacing(10)
                        .push(Text::new(wallpaper.name))
                        .push(PickList::new(&mut wallpaper.pl_state, &WallpaperType::ALL[..], Some(wallpaper.kind), DesktopMessage::WallpaperModeChanged).style(CustomSelect::Primary))
                     )
                  }
                  else {
                     row
                  },
                  grid.push(Button::new(&mut wallpaper.btn_state, wallpaper_svg).on_press(DesktopMessage::WallpaperSelected(idx)).style(if *selected_wallpaper == idx {CustomButton::Selected} else {CustomButton::Text}))
               )
            });
            let wallpaper_sec = Container::new(
               Scrollable::new(wallpaper_scroll).width(Length::Fill).push(wallpaper_grid_view)
            ).width(Length::FillPortion(7)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្នែកខាងលើ
            let current_wallpaper_sec = Container::new(current_wallpaper_row).padding(20);

            // ផ្នែកខាងក្រោម
            let chb_change_wall = Checkbox::new(*change_wallpaper, "Change wallpaper", DesktopMessage::ChangeWallpaperToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_change_wall_dur = PickList::new(change_dur, &ChangeDuration::ALL[..], Some(*selected_change_dur), DesktopMessage::ChangeWallpaperDurChanged).style(CustomSelect::Primary);
            let chb_random_order = Checkbox::new(*random_order, "Random order", DesktopMessage::RandomOrderToggled).spacing(10).style(CustomCheckbox::Default);

            let bottom_sec = Container::new(
               Row::new().spacing(15)
               .push(Space::with_width(Length::FillPortion(3)))
               .push(
                  Column::new().spacing(5).width(Length::FillPortion(7))
                  .push(
                     Row::new().spacing(10).align_items(Align::Center)
                     .push(chb_change_wall)
                     .push(pl_change_wall_dur)
                  )
                  .push(chb_random_order)
               )
            );
            
            Container::new(
               Column::new().width(Length::Fill).spacing(10).align_items(Align::Start)
               .push(current_wallpaper_sec)
               .push(
                  Row::new().spacing(15).height(Length::Fill)
                  .push(category_folder_sec)
                  .push(wallpaper_sec)
               )
               .push(bottom_sec)
            ).width(Length::Fill).height(Length::Fill)
         },
         1 => {
            let ScreenSaverTab {
               screen_saver_wallpapers,
               selected_wallpaper,
               start_after_dur_state,
               start_after_dur_val,
               show_clock,
               random_screen_saver,
               wallpaper_scroll,
            } = screen_saver_tab;

            // ផ្ទាំងខាងឆ្វេង
            let (main_view, screensaver_grid_view) = screen_saver_wallpapers.iter_mut().enumerate().fold((Column::new().padding(20), Grid::new().column_width(127).padding(10).spacing(10)), |(col, grid), (idx, wallpaper)| {
               let wallpaper_svg = Svg::from_path(format!("assets/images/screensaver/{}.svg", wallpaper.path)).width(Length::Fill).height(Length::Units(55));
               let wallpaper_title = Text::new(wallpaper.name);
               let wallpaper_con = Container::new(
                  Column::new().spacing(5).align_items(Align::Center)
                  .push(wallpaper_svg)
                  .push(wallpaper_title)
               );

               (
                  if *selected_wallpaper == idx {
                     col.push(Svg::from_path(format!("assets/images/screensaver/{}.svg", wallpaper.path)).width(Length::Units(300)).height(Length::Units(210)))
                  }
                  else {
                     col
                  },
                  grid.push(Button::new(&mut wallpaper.btn_state, wallpaper_con).on_press(DesktopMessage::ScreenSaverSelected(idx)).style(if *selected_wallpaper == idx {CustomButton::Selected} else {CustomButton::Text}))
               )
            });
            let screen_saver_sec = Container::new(
               Scrollable::new(wallpaper_scroll).width(Length::Fill).push(screensaver_grid_view)
            ).width(Length::FillPortion(4)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្ទាំងខាងស្ដាំ
            let main_view_sec = Container::new(main_view).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite).center_x().center_y();

            // ផ្នែកខាងក្រោម
            let lb_start_after = Text::new("Start after:");
            let pl_start_after_dur = PickList::new(start_after_dur_state, &ChangeDuration::ALL[..], Some(*start_after_dur_val), DesktopMessage::StartAfterDurChanged).style(CustomSelect::Primary);
            let chb_show_clock = Checkbox::new(*show_clock, "Show with clock", DesktopMessage::ShowClockToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_random_screen_saver = Checkbox::new(*random_screen_saver, "Use random screen saver", DesktopMessage::RandomScreenSaverToggled).spacing(10).style(CustomCheckbox::Default);

            let bottom_sec = Container::new(
               Row::new().spacing(15)
               .push(
                  Row::new().spacing(10).align_items(Align::Center).width(Length::FillPortion(4))
                  .push(lb_start_after)
                  .push(pl_start_after_dur)
               )
               .push(
                  Column::new().spacing(5).width(Length::FillPortion(6))
                  .push(chb_show_clock)
                  .push(chb_random_screen_saver)
               )
            );

            Container::new(
               Column::new().width(Length::Fill).spacing(10).align_items(Align::Start)
               .push(
                  Row::new().spacing(15).width(Length::Fill).height(Length::Fill)
                  .push(screen_saver_sec)
                  .push(main_view_sec)
               )
               .push(bottom_sec)
            ).width(Length::Fill).height(Length::Fill)
         },
         _ => Container::new(Space::with_height(Length::Fill))
      };
      
      // មាតិកា   
      let content = Column::new().width(Length::Fill).align_items(Align::Center)
         .push(tabbar_section)
         .push(tabview.height(Length::Fill).padding(20).style(CustomContainer::ForegroundGray));

      Container::new(content).width(Length::FillPortion(15)).padding(20).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone, Default)]
pub(self) struct DesktopTab {
   categories_folder: Vec<(&'static str, &'static str, button::State, CategoryFolder)>,
   selected_category_folder: usize,
   categories_wallpaper_map: Vec<Vec<Wallpaper>>,
   selected_wallpaper: usize,
   change_wallpaper: bool,
   change_dur: pick_list::State<ChangeDuration>,
   selected_change_dur: ChangeDuration,
   random_order: bool,
   category_folder_scroll: scrollable::State,
   wallpaper_scroll: scrollable::State,
}

#[derive(Debug, Clone, Default)]
pub(self) struct ScreenSaverTab {
   screen_saver_wallpapers: Vec<Wallpaper>,
   selected_wallpaper: usize,
   start_after_dur_state: pick_list::State<ChangeDuration>,
   start_after_dur_val: ChangeDuration,
   show_clock: bool,
   random_screen_saver: bool,
   wallpaper_scroll: scrollable::State,
}

impl DesktopTab {
   pub fn new() -> Self {
      Self {
         categories_folder: vec![
            ("default-folder", "Desktop Pictures", button::State::new(), CategoryFolder::Default), 
            ("color", "Color", button::State::new(), CategoryFolder::Default),
            ("folder-favorites", "Favorites", button::State::new(), CategoryFolder::Photos), 
            ("folder-share", "Shares", button::State::new(), CategoryFolder::Photos), 
            ("folder-album", "Album", button::State::new(), CategoryFolder::Photos), 
            ("folder-pictures", "Pictures", button::State::new(), CategoryFolder::Folders), 
         ],
         categories_wallpaper_map: vec![
            vec![
               Wallpaper::new("Flat-Mountains", "Flat Mountains"),
               Wallpaper::new("Hollowed-Boxes", "Hollowed Boxes"),
               Wallpaper::new("Abstract-Timekeeper", "Abstract Timekeeper"),
               Wallpaper::new("Geometric-Intersection", "Geometric Intersection"),
               Wallpaper::new("Vanishing-Stripes", "Vanishing Stripes"),
               Wallpaper::new("Slanted-Gradient", "Slanted Gradient"),
               Wallpaper::new("Large-Triangles", "Large Triangles"),
               Wallpaper::new("Rainbow-Vortex", "Rainbow Vortex"),
               Wallpaper::new("Subtle-Prism", "Subtle Prism"),
               Wallpaper::new("Sun-Tornado", "Sun Tornado"),
               Wallpaper::new("Page-Turner", "Page Turner"),
            ],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
         ],
         ..Default::default()
      }
   }
}

impl ScreenSaverTab {
   pub fn new() -> Self {
      Self {
         screen_saver_wallpapers: vec![
            Wallpaper::new("Cornered-Stairs", "Cornered Stairs"),
            Wallpaper::new("Alternating-Arrowhead", "Alternating Arrowhead"),
            Wallpaper::new("Protruding-Squares", "Protruding Squares"),
            Wallpaper::new("Zig-Zag", "Zig Zag"),
            Wallpaper::new("Bermuda-Diamond", "Bermuda Diamond"),
            Wallpaper::new("Confetti-Doodles", "Confetti Doodles"),
            Wallpaper::new("Liquid-Cheese", "Liquid Cheese"),
            Wallpaper::new("Radiant-Gradient", "Radiant Gradient"),
            Wallpaper::new("Scattered-Forcefields", "Scattered Forcefields"),
            Wallpaper::new("Wintery-Sunburst", "Wintery Sunburst"),
            Wallpaper::new("Repeating-Chevrons", "Repeating Chevrons"),
            Wallpaper::new("Pattern-Randomized", "Pattern Randomized"),
            Wallpaper::new("Protruding-Squares", "Protruding Squares"),
            Wallpaper::new("Bermuda-Traingle", "Bermuda Traingles"),
         ],
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, SmartDefault)]
pub enum CategoryFolder {
   #[default]
   Default,
   Photos,
   Folders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum WallpaperType {
   #[default]
   Dyn,
   Light,
   Dark
}

impl WallpaperType {
   const ALL: [WallpaperType; 3] = [
      WallpaperType::Dyn,
      WallpaperType::Light,
      WallpaperType::Dark,
   ];
}

impl std::fmt::Display for WallpaperType {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      use WallpaperType::*;
      write!(f, "{}", match self {
         Dyn => "Dynamic",
         Light => "Light",
         Dark => "Dark",
      })
   }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum ChangeDuration {
   _15m,
   #[default]
   _30m,
   _1h,
   _2h,
   Everyday
}

impl ChangeDuration {
   const ALL: [ChangeDuration; 5] = [
      ChangeDuration::_15m,
      ChangeDuration::_30m,
      ChangeDuration::_1h,
      ChangeDuration::_2h,
      ChangeDuration::Everyday
   ];
}

impl std::fmt::Display for ChangeDuration {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      use ChangeDuration::*;
      write!(f, "{}", match self {
         _15m => "Every 15 minutes",
         _30m => "Every 30 minutes",
         _1h => "Every hour",
         _2h => "Every 2 hours",
         Everyday => "Everyday",
      })
   }
}

#[derive(Debug, Clone, Default)]
struct Wallpaper {
   path: &'static str,
   name: &'static str,
   btn_state: button::State,
   pl_state: pick_list::State<WallpaperType>,
   kind: WallpaperType,
}

impl Wallpaper {
   pub fn new(path: &'static str, name: &'static str) -> Self {
      Self {
         path,
         name,
         ..Default::default()
      }
   }
}