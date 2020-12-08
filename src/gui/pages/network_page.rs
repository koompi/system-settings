#![allow(unused_variables)]
#![allow(unused_imports)]
use super::super::styles::{CustomButton, CustomContainer};

use iced::{
    button, pick_list, scrollable, text_input, Align, Button, Checkbox, Column, Container, Element,
    HorizontalAlignment, Length, PickList, Radio, Row, Rule, Scrollable, Space, Svg, Text,
    TextInput,
};
use iced_custom_widget::Grid;
use vedas_core::macros::select::*;
#[derive(Debug, Clone)]
pub struct NetworkPage {
    search: text_input::State,
    value: String,
    tabbar_state: Vec<(String, button::State, Control)>,
    current_idx: usize,
    select_tab: Control,
    gencofig: GeneralConfig,
    identity: Identify,
    security: Security,
    ipv4: IPV4,
    ipv6: IPV6,
    apply: button::State,
    cancel: button::State,
    list_wifi: Vec<ListCon>,
    scr_list: scrollable::State,
}
#[derive(Debug, Copy, Clone)]
pub enum Control {
    Details,
    Wifi,
    Security,
    IPv4,
    Ipv6,
}
#[derive(Debug, Clone)]
pub enum NetMessage {
    OnSearchWif(String),
    TabChanged(usize, Control),
    GenConfigMsg(GenConfigMsg),
    IdentifyMsg(IdentifyMsg),
    SecureMsg(SecureMsg),
    IPv4Msg(IPv4Msg),
    IPv6Msg(IPv6Msg),
    ListMessage(ListMessage),
    ApplyChanged,
    CancelChanged,
}

impl NetworkPage {
    pub fn new() -> Self {
        let function = |name: &str, icon: &str, status: &str| {
            ListCon::new(name.to_string(), icon.to_string(), status.to_string())
        };
        let prefe = vec![
            function("Koompi Attic", "wireless", "connected "),
            function("Koompi OS", "wireless", "connected "),
            function("SmallWorld Venture", "wireless", "connected "),
            function("Smallworld Space", "wireless", "connected "),
            function("Kong buthon", "wireless", "connected "),
            function("Koompi lab", "wireless", "connected "),
            function("Koompi Attic", "wireless", "connected "),
            function("Koompi OS", "wireless", "connected "),
            function("SmallWorld Venture", "wireless", "connected "),
            function("Smallworld Space", "wireless", "connected "),
            function("Kong buthon", "wireless", "connected "),
            function("Koompi lab", "wireless", "connected "),
            function("Koompi Attic", "wireless", "connected "),
            function("Koompi OS", "wireless", "connected "),
            function("SmallWorld Venture", "wireless", "connected "),
            function("Smallworld Space", "wireless", "connected "),
            function("Kong buthon", "wireless", "connected "),
            function("Koompi lab", "wireless", "connected "),
            function("Koompi Attic", "wireless", "connected "),
            function("Koompi OS", "wireless", "connected "),
            function("SmallWorld Venture", "wireless", "connected "),
            function("Smallworld Space", "wireless", "connected "),
            function("Kong buthon", "wireless", "connected "),
            function("Koompi lab", "wireless", "connected "),
        ];
        Self {
            search: text_input::State::new(),
            value: String::default(),
            tabbar_state: vec![
                (
                    "  General  ".to_string(),
                    button::State::new(),
                    Control::Details,
                ),
                (
                    "  Identify  ".to_string(),
                    button::State::new(),
                    Control::Wifi,
                ),
                (
                    "  Security  ".to_string(),
                    button::State::new(),
                    Control::Security,
                ),
                ("  IPV4  ".to_string(), button::State::new(), Control::IPv4),
                ("  IPV6  ".to_string(), button::State::new(), Control::Ipv6),
            ],
            current_idx: 0,
            select_tab: Control::Details,
            gencofig: GeneralConfig::new(),
            identity: Identify::new(),
            security: Security::new(),
            ipv4: IPV4::new(),
            ipv6: IPV6::new(),
            apply: button::State::new(),
            cancel: button::State::new(),
            list_wifi: prefe,
            scr_list: scrollable::State::new(),
        }
    }
    pub fn update(&mut self, msg: NetMessage) {
        match msg {
            NetMessage::OnSearchWif(text) => {
                self.value = text;
            }
            NetMessage::TabChanged(idx, control) => {
                self.current_idx = idx;
                self.select_tab = control;
                // println!("you select: {:?}", control);
            }
            NetMessage::GenConfigMsg(msg) => {
                self.gencofig.update(msg);
            }
            NetMessage::IdentifyMsg(msg) => {
                self.identity.update(msg);
            }
            NetMessage::SecureMsg(msg) => {
                self.security.update(msg);
            }
            NetMessage::IPv4Msg(msg) => {
                self.ipv4.update(msg);
            }
            NetMessage::IPv6Msg(msg) => {
                self.ipv6.update(msg);
            }
            NetMessage::ApplyChanged => {}
            NetMessage::CancelChanged => {}
            NetMessage::ListMessage(msg) => {}
        }
    }

    pub fn view(&mut self) -> Element<NetMessage> {
        let NetworkPage {
            search,
            value,
            tabbar_state,
            current_idx,
            select_tab,
            gencofig,
            identity,
            security,
            ipv4,
            ipv6,
            apply,
            cancel,
            list_wifi,
            scr_list,
        } = self;
        let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
        for (idx, (name, btn_state, control)) in tabbar_state.iter_mut().enumerate() {
            let mut btn = Button::new(btn_state, Text::new(name.as_str()))
                .padding(5)
                .on_press(NetMessage::TabChanged(idx, *control));
            if *current_idx == idx {
                btn = btn.style(CustomButton::SelectedTab);
            } else {
                btn = btn.style(CustomButton::Tab);
            }
            tabbar = tabbar.push(btn);
        }
        let tabbar_con = Container::new(tabbar)
            .padding(2)
            .center_x()
            .style(CustomContainer::Segment);
        let tabbar_section = Container::new(tabbar_con)
            .padding(7)
            .width(Length::Fill)
            .center_x();
        let tabview = match select_tab {
            Control::Details => gencofig
                .view()
                .map(move |msg| NetMessage::GenConfigMsg(msg)),
            Control::Wifi => identity.view().map(move |msg| NetMessage::IdentifyMsg(msg)),
            Control::Security => security.view().map(move |msg| NetMessage::SecureMsg(msg)),
            Control::IPv4 => ipv4.view().map(move |msg| NetMessage::IPv4Msg(msg)),
            Control::Ipv6 => ipv6.view().map(move |msg| NetMessage::IPv6Msg(msg)),
        };
        let list_view = Scrollable::new(scr_list).push(list_wifi.iter_mut().fold(
            Column::new().spacing(5),
            |column, pref| {
                column
                    .push(Rule::horizontal(4))
                    .push(pref.view().map(move |msg| NetMessage::ListMessage(msg)))
            },
        ));
        let list_container = Container::new(list_view).center_x().center_y();
        let list_side = Column::new()
            .width(Length::FillPortion(3))
            .align_items(Align::Center)
            .spacing(10)
            .push(Text::new("Connection").size(25))
            .push(
                TextInput::new(search, "searching..", value, NetMessage::OnSearchWif)
                    .size(18)
                    .padding(5),
            )
            .push(Text::new("Wi-Fi").size(18))
            .push(list_container);
        let apply = Column::new().push(
            Row::new()
                .spacing(10)
                .push(
                    Button::new(
                        apply,
                        Text::new("Cancel").horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .padding(10)
                    .width(Length::Units(100))
                    .on_press(NetMessage::ApplyChanged),
                )
                .push(
                    Button::new(
                        cancel,
                        Text::new("Apply").horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .width(Length::Units(100))
                    .padding(10)
                    .on_press(NetMessage::CancelChanged),
                ),
        );
        let content_side = Column::new()
            .align_items(Align::Center)
            .push(
                Text::new("Network Setting")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(25),
            )
            .push(tabbar_section)
            .push(tabview)
            .push(apply.padding(10))
            .width(Length::FillPortion(7))
            .height(Length::Fill);

        let main_layout: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(list_side)
            .push(Rule::vertical(10))
            .push(content_side)
            .into();
        Container::new(main_layout)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(CustomContainer::FadedBrightForeground)
            .into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum VPN {
    KoompiVPN,
    FastVPN,
    NetVPN,
}
impl VPN {
    const ALL: [VPN; 3] = [VPN::KoompiVPN, VPN::FastVPN, VPN::NetVPN];
}
select_display!(VPN, VPN::KoompiVPN => "KoompiVPN", VPN::FastVPN => "FastVPN", VPN::NetVPN => "NetVPN");

impl Default for VPN {
    fn default() -> Self {
        VPN::KoompiVPN
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum Metered {
    Automatic,
    Yes,
    No,
}
impl Metered {
    const ALL: [Metered; 3] = [Metered::Automatic, Metered::Yes, Metered::No];
}
impl Default for Metered {
    fn default() -> Self {
        Metered::Automatic
    }
}
select_display!(Metered, Metered::Automatic => "Automatic", Metered::Yes => "Yes", Metered::No => "No");
#[derive(Debug, Clone)]
pub enum GenConfigMsg {
    AutoConnection(bool),
    PriorityChanged(String),
    FireWallChanged(String),
    AllowAllUser(bool),
    AutoVPN(bool),
    VPNChnaged(VPN),
    MeterChanged(Metered),
    AdvanceChanged,
}
#[derive(Debug, Clone)]
pub struct GeneralConfig {
    // primitive State
    is_auto: bool,
    is_alluser: bool,
    is_vpn: bool,
    priority_value: String,
    fireall_value: String,
    // Compound State
    auto_vpn: pick_list::State<VPN>,
    meters: pick_list::State<Metered>,
    select_vpn: VPN,
    select_meter: Metered,
    advance: button::State,
    priority_input: text_input::State,
    firewall: text_input::State,
}
impl GeneralConfig {
    pub fn new() -> Self {
        Self {
            is_auto: true,
            is_alluser: true,
            is_vpn: false,
            priority_value: String::default(),
            fireall_value: String::default(),
            auto_vpn: pick_list::State::default(),
            meters: pick_list::State::default(),
            select_vpn: VPN::KoompiVPN,
            select_meter: Metered::Automatic,
            advance: button::State::new(),
            priority_input: text_input::State::new(),
            firewall: text_input::State::new(),
        }
    }
    pub fn update(&mut self, msg: GenConfigMsg) {
        match msg {
            GenConfigMsg::PriorityChanged(value) => {
                self.priority_value = value;
            }
            GenConfigMsg::AutoConnection(value) => {
                self.is_auto = value;
            }
            GenConfigMsg::AllowAllUser(value) => {
                self.is_alluser = value;
            }
            GenConfigMsg::AutoVPN(value) => {
                self.is_vpn = value;
            }
            GenConfigMsg::VPNChnaged(vpn) => {
                self.select_vpn = vpn;
            }
            GenConfigMsg::AdvanceChanged => {}
            GenConfigMsg::FireWallChanged(value) => {
                self.fireall_value = value;
            }
            GenConfigMsg::MeterChanged(meter) => {
                self.select_meter = meter;
            }
        }
    }
    pub fn view(&mut self) -> Element<GenConfigMsg> {
        let GeneralConfig {
            is_auto,
            is_alluser,
            is_vpn,
            priority_value,
            fireall_value,
            auto_vpn,
            meters,
            select_vpn,
            select_meter,
            advance,
            priority_input,
            firewall,
        } = self;

        let row1 = Row::new()
            .spacing(50)
            .align_items(Align::Center)
            .push(Checkbox::new(
                *is_auto,
                "Connection automatically with priority",
                GenConfigMsg::AutoConnection,
            ))
            .push(
                TextInput::new(
                    priority_input,
                    "Enter priority",
                    &priority_value,
                    GenConfigMsg::PriorityChanged,
                )
                .padding(10),
            );
        let row2 = Row::new()
            .align_items(Align::Center)
            .spacing(52)
            .push(Checkbox::new(
                *is_alluser,
                "All users may connect to this network",
                GenConfigMsg::AllowAllUser,
            ))
            .push(
                Button::new(
                    advance,
                    Text::new("Adavance...").horizontal_alignment(HorizontalAlignment::Center),
                )
                .padding(10)
                .width(Length::Fill)
                .on_press(GenConfigMsg::AdvanceChanged),
            );
        let row3 = Row::new()
            .align_items(Align::Center)
            .spacing(98)
            .push(Checkbox::new(
                *is_vpn,
                "Automatically connect to VPN",
                GenConfigMsg::AutoVPN,
            ))
            .push(
                PickList::new(
                    auto_vpn,
                    &VPN::ALL[..],
                    Some(*select_vpn),
                    GenConfigMsg::VPNChnaged,
                )
                .padding(10)
                .width(Length::Fill),
            );
        let row4 = Row::new()
            .align_items(Align::Center)
            .spacing(222)
            .push(Text::new("Firewall zone:"))
            .push(
                TextInput::new(
                    firewall,
                    "Firewall Zone",
                    fireall_value,
                    GenConfigMsg::FireWallChanged,
                )
                .padding(10)
                .width(Length::Fill),
            );
        let row5 = Row::new()
            .align_items(Align::Center)
            .push(Text::new("Metered: "))
            .spacing(245)
            .push(
                PickList::new(
                    meters,
                    &Metered::ALL[..],
                    Some(*select_meter),
                    GenConfigMsg::MeterChanged,
                )
                .padding(10)
                .width(Length::Fill),
            );
        let general_content = Column::new()
            .padding(20)
            .spacing(10)
            .push(row1)
            .push(row2)
            .push(row3)
            .push(row4)
            .push(row5)
            .width(Length::Fill);
        Container::new(general_content)
            .style(CustomContainer::ForegroundWhite)
            .into()
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum BSSID {
    Address,
}
select_display!(BSSID, BSSID::Address => "00:33:44:6D::8D");
impl Default for BSSID {
    fn default() -> Self {
        BSSID::Address
    }
}
impl BSSID {
    const ALL: [BSSID; 1] = [BSSID::Address];
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum MAC {
    Address,
}
impl MAC {
    const ALL: [MAC; 1] = [MAC::Address];
}

impl Default for MAC {
    fn default() -> Self {
        MAC::Address
    }
}
select_display!(MAC, MAC::Address => "00:33:44:6D::8D (wlp2s0)");
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum CloneAddr {
    Preserve,
    Permenant,
    Randome,
    Stable,
}
select_display!(CloneAddr,
    CloneAddr::Preserve => "Preserve",
    CloneAddr::Permenant => "Permenant",
    CloneAddr::Randome =>"Randome",
    CloneAddr::Stable => "Stable");
impl CloneAddr {
    const ALL: [CloneAddr; 4] = [
        CloneAddr::Preserve,
        CloneAddr::Permenant,
        CloneAddr::Randome,
        CloneAddr::Stable,
    ];
}
impl Default for CloneAddr {
    fn default() -> Self {
        CloneAddr::Permenant
    }
}
#[derive(Default, Debug, Clone)]
pub struct Identify {
    ssid: String,
    bssid: BSSID,
    mac_address: MAC,
    clone_address: CloneAddr,
    ssid_ui: text_input::State,
    bssid_ui: pick_list::State<BSSID>,
    mac_addr_ui: pick_list::State<MAC>,
    clone: pick_list::State<CloneAddr>,
}
#[derive(Debug, Clone)]
pub enum IdentifyMsg {
    SsidChanged(String),
    BssidChanged(BSSID),
    MacChanged(MAC),
    CloneChanged(CloneAddr),
}

impl Identify {
    pub fn new() -> Self {
        Self {
            ssid: String::default(),
            bssid: BSSID::default(),
            mac_address: MAC::default(),
            clone_address: CloneAddr::default(),
            ssid_ui: text_input::State::new(),
            bssid_ui: pick_list::State::default(),
            mac_addr_ui: pick_list::State::default(),
            clone: pick_list::State::default(),
        }
    }
    pub fn update(&mut self, msg: IdentifyMsg) {
        match msg {
            IdentifyMsg::SsidChanged(value) => {
                self.ssid = value;
            }
            IdentifyMsg::BssidChanged(bssid) => {
                self.bssid = bssid;
            }
            IdentifyMsg::MacChanged(mac) => {
                self.mac_address = mac;
            }
            IdentifyMsg::CloneChanged(clone) => {
                self.clone_address = clone;
            }
        }
    }
    pub fn view(&mut self) -> Element<IdentifyMsg> {
        let Identify {
            ssid,
            bssid,
            mac_address,
            clone_address,
            ssid_ui,
            bssid_ui,
            mac_addr_ui,
            clone,
        } = self;
        let row = Row::new()
            .align_items(Align::Center)
            .push(Text::new("SSID").width(Length::Units(100)))
            .push(
                TextInput::new(ssid_ui, "Koompi Attic", ssid, IdentifyMsg::SsidChanged)
                    .padding(10)
                    .width(Length::Fill),
            );
        let row1 = Row::new()
            .align_items(Align::Center)
            .push(Text::new("BSSID").width(Length::Units(100)))
            .push(
                PickList::new(
                    bssid_ui,
                    &BSSID::ALL[..],
                    Some(*bssid),
                    IdentifyMsg::BssidChanged,
                )
                .padding(10)
                .width(Length::Fill),
            );
        let row2 = Row::new()
            .align_items(Align::Center)
            .push(Text::new("MAC Addres").width(Length::Units(100)))
            .push(
                PickList::new(
                    mac_addr_ui,
                    &MAC::ALL[..],
                    Some(*mac_address),
                    IdentifyMsg::MacChanged,
                )
                .padding(10)
                .width(Length::Fill),
            );
        let row3 = Row::new()
            .align_items(Align::Center)
            .push(Text::new("Clone Address").width(Length::Units(100)))
            .push(
                PickList::new(
                    clone,
                    &CloneAddr::ALL[..],
                    Some(*clone_address),
                    IdentifyMsg::CloneChanged,
                )
                .padding(10)
                .width(Length::Fill),
            );
        let content: Element<_> = Column::new()
            .spacing(10)
            .padding(20)
            .width(Length::Fill)
            .push(row)
            .push(row1)
            .push(row2)
            .push(row3)
            .into();
        Container::new(content)
            .style(CustomContainer::ForegroundWhite)
            .into()
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum SecurityType {
    WPA2Personal,
    WPA2Enterprise,
    WPA3Personal,
    Dynamic802x,
    NONE,
}

select_display!(SecurityType,
    SecurityType::WPA2Personal => "WPA/WPA2 Personal",
    SecurityType::WPA2Enterprise => "WPA/WPA2 Enterprise",
    SecurityType::WPA3Personal =>"WPA3Personal",
    SecurityType::Dynamic802x => "Dynamic WEB (802.1x)", 
    SecurityType::NONE => "None");

impl SecurityType {
    const ALL: [SecurityType; 5] = [
        SecurityType::WPA2Personal,
        SecurityType::WPA2Enterprise,
        SecurityType::WPA3Personal,
        SecurityType::Dynamic802x,
        SecurityType::NONE,
    ];
}
impl Default for SecurityType {
    fn default() -> Self {
        SecurityType::WPA2Personal
    }
}
#[derive(Debug, Clone)]
pub enum SecureMsg {
    PassChanged(String),
    ShowPassChanged(bool),
    SecureChanged(SecurityType),
}
#[derive(Debug, Clone)]
pub struct Security {
    security: SecurityType,
    pass_value: String,
    show_pass: bool,
    security_ui: pick_list::State<SecurityType>,
    pass_ui: text_input::State,
}

impl Security {
    pub fn new() -> Self {
        Self {
            security: SecurityType::default(),
            pass_value: String::from("KOOMPI123"),
            show_pass: false,
            security_ui: pick_list::State::default(),
            pass_ui: text_input::State::new(),
        }
    }

    pub fn update(&mut self, msg: SecureMsg) {
        match msg {
            SecureMsg::PassChanged(value) => {
                self.pass_value = value;
            }
            SecureMsg::SecureChanged(secure) => {
                self.security = secure;
            }
            SecureMsg::ShowPassChanged(value) => {
                self.show_pass = value;
            }
        }
    }
    pub fn view(&mut self) -> Element<SecureMsg> {
        let Security {
            security,
            pass_value,
            show_pass,
            security_ui,
            pass_ui,
        } = self;
        let row = Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(Text::new("Security: ").width(Length::Units(150)))
            .push(
                PickList::new(
                    security_ui,
                    &SecurityType::ALL[..],
                    Some(*security),
                    SecureMsg::SecureChanged,
                )
                .width(Length::Fill)
                .padding(10),
            );
        let row1 = Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(Text::new("Password: ").width(Length::Units(150)))
            .push(match *show_pass {
                true => TextInput::new(pass_ui, "password..", pass_value, SecureMsg::PassChanged)
                    .padding(10),
                false => TextInput::new(pass_ui, "password..", pass_value, SecureMsg::PassChanged)
                    .password()
                    .padding(10),
            });
        let row2 = Row::new()
            .push(Space::with_width(Length::Units(150)))
            .push(Checkbox::new(
                *show_pass,
                "Show password",
                SecureMsg::ShowPassChanged,
            ));
        let content: Element<_> = Column::new()
            .padding(10)
            .width(Length::Fill)
            .spacing(10)
            .push(row)
            .push(row1)
            .push(row2)
            .into();
        Container::new(content)
            .style(CustomContainer::ForegroundWhite)
            .into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPV4Method {
    DHCP,
    Manual,
    Shared,
    LinkLocal,
    Disable,
}
impl Default for IPV4Method {
    fn default() -> Self {
        IPV4Method::DHCP
    }
}
impl IPV4Method {
    fn all() -> [IPV4Method; 5] {
        [
            IPV4Method::DHCP,
            IPV4Method::Manual,
            IPV4Method::Shared,
            IPV4Method::LinkLocal,
            IPV4Method::Disable,
        ]
    }
}
impl From<IPV4Method> for String {
    fn from(language: IPV4Method) -> String {
        String::from(match language {
            IPV4Method::DHCP => "Automatic(DHCP)",
            IPV4Method::Manual => "Manual",
            IPV4Method::Shared => "Shared to other",
            IPV4Method::LinkLocal => "Link-Local Only",
            IPV4Method::Disable => "Disable",
        })
    }
}
#[derive(Debug, Clone)]
pub struct IPV4 {
    dns: String,
    dns_auto: bool,
    private_net: bool,
    selected: Option<IPV4Method>,
    address: Vec<String>,
    routes: Vec<String>,
    man_addr: String,
    man_netmask: String,
    man_gateway: String,
    man_addui: text_input::State,
    man_netmaskui: text_input::State,
    man_gatewayui: text_input::State,
    dns_input: text_input::State,
    delete_manual: button::State,
    route_addval: String,
    route_netval: String,
    route_gateval: String,
    route_metricval: String,
    delte_rotue: button::State,
    route_addui: text_input::State,
    route_netmaskui: text_input::State,
    route_gatewayui: text_input::State,
    route_metric: text_input::State,
}

#[derive(Debug, Clone)]
pub enum IPv4Msg {
    IPV4MethodChanged(IPV4Method),
    Add2Chagned(String),
    AddNetmastk(String),
    AddGateway(String),
    RouteChagned(String),
    RouteNetmastk(String),
    RouteGateway(String),
    RouteMetric(String),
    DeleteManual,
    DelteManualRoute,
    DnsChanged(String),
    DnsAutomatic(bool),
    PrivateNetChagned(bool),
}

impl IPV4 {
    pub fn new() -> Self {
        Self {
            dns: String::default(),
            dns_auto: true,
            private_net: false,
            selected: Some(IPV4Method::default()),
            address: vec![
                "Address".to_string(),
                "Netmask".to_string(),
                "Gateway".to_string(),
            ],
            routes: vec![
                "Address".to_string(),
                "Netmask".to_string(),
                "Gateway".to_string(),
                "Metric".to_string(),
            ],
            man_addr: String::default(),
            man_gateway: String::default(),
            man_netmask: String::default(),
            man_addui: text_input::State::new(),
            man_gatewayui: text_input::State::new(),
            man_netmaskui: text_input::State::new(),
            dns_input: text_input::State::new(),
            delete_manual: button::State::new(),
            route_addval: String::default(),
            route_netval: String::default(),
            route_gateval: String::default(),
            route_metricval: String::default(),
            delte_rotue: button::State::new(),
            route_addui: text_input::State::new(),
            route_gatewayui: text_input::State::new(),
            route_netmaskui: text_input::State::new(),
            route_metric: text_input::State::new(),
        }
    }
    pub fn update(&mut self, msg: IPv4Msg) {
        match msg {
            IPv4Msg::IPV4MethodChanged(value) => {
                self.selected = Some(value);
            }
            IPv4Msg::Add2Chagned(value) => {
                self.man_addr = value;
            }
            IPv4Msg::AddNetmastk(value) => {
                self.man_netmask = value;
            }
            IPv4Msg::AddGateway(value) => {
                self.man_gateway = value;
            }
            IPv4Msg::DeleteManual => {}
            IPv4Msg::DnsAutomatic(value) => {
                self.dns_auto = value;
            }
            IPv4Msg::DnsChanged(value) => {
                self.dns = value;
            }
            IPv4Msg::PrivateNetChagned(value) => {
                self.private_net = value;
            }
            IPv4Msg::RouteChagned(value) => {
                self.route_addval = value;
            }
            IPv4Msg::RouteGateway(value) => {
                self.route_netval = value;
            }
            IPv4Msg::RouteNetmastk(value) => {
                self.route_gateval = value;
            }
            IPv4Msg::RouteMetric(value) => {
                self.route_metricval = value;
            }
            IPv4Msg::DelteManualRoute => {}
        }
    }

    pub fn view(&mut self) -> Element<IPv4Msg> {
        let IPV4 {
            dns,
            dns_auto,
            private_net,
            selected,
            address,
            routes,
            man_addr,
            man_netmask,
            man_gateway,
            man_addui,
            man_netmaskui,
            man_gatewayui,
            dns_input,
            delete_manual,
            delte_rotue,
            route_addval,
            route_netval,
            route_gateval,
            route_metricval,
            route_addui,
            route_netmaskui,
            route_gatewayui,
            route_metric,
        } = self;

        let grid = IPV4Method::all().iter().cloned().fold(
            Grid::new().column_width(200),
            |layout, value| {
                layout.push(
                    Radio::new(value, value, *selected, IPv4Msg::IPV4MethodChanged)
                        .size(18)
                        .width(Length::Units(100)),
                )
            },
        );
        let data = match selected.unwrap() {
            IPV4Method::Manual => Column::new()
                .width(Length::Fill)
                .push(Column::new().push(Text::new("Address: ")))
                .push(
                    address
                        .iter()
                        .fold(Row::new().width(Length::Fill), |row, value| {
                            row.push(
                                Column::new()
                                    .width(Length::FillPortion(1))
                                    .align_items(Align::Center)
                                    .push(Text::new(value)),
                            )
                        }),
                )
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .width(Length::Fill)
                        .push(
                            TextInput::new(man_addui, "", man_addr, IPv4Msg::Add2Chagned)
                                .width(Length::FillPortion(1))
                                .padding(10),
                        )
                        .push(
                            TextInput::new(man_netmaskui, "", man_netmask, IPv4Msg::AddNetmastk)
                                .width(Length::FillPortion(1))
                                .padding(10),
                        )
                        .push(
                            TextInput::new(man_gatewayui, "", man_gateway, IPv4Msg::AddGateway)
                                .width(Length::FillPortion(1))
                                .padding(10),
                        )
                        .push(
                            Button::new(delete_manual, Text::new("Delete"))
                                .on_press(IPv4Msg::DeleteManual)
                                .padding(10),
                        ),
                ),
            _ => Column::new(),
        };
        let dns_content = Column::new()
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::Start)
                            .push(Text::new("DNS")),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::End)
                            .push(Checkbox::new(*dns_auto, "Automatic", IPv4Msg::DnsAutomatic)),
                    ),
            )
            .push(
                TextInput::new(dns_input, "", dns, IPv4Msg::DnsChanged)
                    .padding(10)
                    .width(Length::Fill),
            );

        let route_section = Column::new()
            .width(Length::Fill)
            .push(Column::new().push(Text::new("Routes: ")))
            .push(
                routes
                    .iter()
                    .fold(Row::new().width(Length::Fill), |row, value| {
                        row.push(
                            Column::new()
                                .width(Length::FillPortion(1))
                                .align_items(Align::Center)
                                .push(Text::new(value)),
                        )
                    }),
            )
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(
                        TextInput::new(route_addui, "", route_addval, IPv4Msg::RouteChagned)
                            .width(Length::FillPortion(2))
                            .padding(10),
                    )
                    .push(
                        TextInput::new(route_netmaskui, "", route_netval, IPv4Msg::RouteNetmastk)
                            .width(Length::FillPortion(2))
                            .padding(10),
                    )
                    .push(
                        TextInput::new(route_gatewayui, "", route_gateval, IPv4Msg::RouteGateway)
                            .width(Length::FillPortion(2))
                            .padding(10),
                    )
                    .push(
                        TextInput::new(route_metric, "", route_metricval, IPv4Msg::RouteMetric)
                            .width(Length::FillPortion(1))
                            .padding(10),
                    )
                    .push(
                        Button::new(delte_rotue, Text::new("Delete"))
                            .on_press(IPv4Msg::DelteManualRoute)
                            .padding(10),
                    ),
            );
        let first_section = Row::new()
            .push(Text::new("IPv4 Method").width(Length::Fill))
            .push(grid);
        let contetn: Element<_> = Column::new()
            .spacing(10)
            .push(first_section)
            .push(data)
            .push(dns_content)
            .push(route_section)
            .push(Checkbox::new(
                *private_net,
                "Use this connection only for resources on its network",
                IPv4Msg::PrivateNetChagned,
            ))
            .width(Length::Fill)
            .into();
        Container::new(contetn)
            .padding(10)
            .style(CustomContainer::ForegroundWhite)
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPV6Method {
    DHCP,
    Manual,
    Shared,
    LinkLocal,
    Disable,
    Automatic,
}
impl Default for IPV6Method {
    fn default() -> Self {
        IPV6Method::DHCP
    }
}
impl IPV6Method {
    fn all() -> [IPV6Method; 6] {
        [
            IPV6Method::DHCP,
            IPV6Method::Manual,
            IPV6Method::Shared,
            IPV6Method::LinkLocal,
            IPV6Method::Disable,
            IPV6Method::Automatic,
        ]
    }
}
impl From<IPV6Method> for String {
    fn from(language: IPV6Method) -> String {
        String::from(match language {
            IPV6Method::DHCP => "Automatic(DHCP)",
            IPV6Method::Manual => "Manual",
            IPV6Method::Shared => "Shared to other",
            IPV6Method::LinkLocal => "Link-Local Only",
            IPV6Method::Disable => "Disable",
            IPV6Method::Automatic => "Automatic, DHCP only",
        })
    }
}
#[derive(Debug, Clone)]
pub struct IPV6 {
    dns: String,
    dns_auto: bool,
    private_net: bool,
    selected: Option<IPV6Method>,
    address: Vec<String>,
    routes: Vec<String>,
    man_addr: String,
    man_netmask: String,
    man_gateway: String,
    man_addui: text_input::State,
    man_netmaskui: text_input::State,
    man_gatewayui: text_input::State,
    dns_input: text_input::State,
    delete_manual: button::State,
    route_addval: String,
    route_netval: String,
    route_gateval: String,
    route_metricval: String,
    delte_rotue: button::State,
    route_addui: text_input::State,
    route_netmaskui: text_input::State,
    route_gatewayui: text_input::State,
    route_metric: text_input::State,
}

#[derive(Debug, Clone)]
pub enum IPv6Msg {
    IPV6MethodChanged(IPV6Method),
    Add2Chagned(String),
    AddNetmastk(String),
    AddGateway(String),
    RouteChagned(String),
    RouteNetmastk(String),
    RouteGateway(String),
    RouteMetric(String),
    DeleteManual,
    DelteManualRoute,
    DnsChanged(String),
    DnsAutomatic(bool),
    PrivateNetChagned(bool),
}

impl IPV6 {
    pub fn new() -> Self {
        Self {
            dns: String::default(),
            dns_auto: true,
            private_net: false,
            selected: Some(IPV6Method::default()),
            address: vec![
                "Address".to_string(),
                "Prefix".to_string(),
                "Gateway".to_string(),
            ],
            routes: vec![
                "Address".to_string(),
                "Prefix".to_string(),
                "Gateway".to_string(),
                "Metric".to_string(),
            ],
            man_addr: String::default(),
            man_gateway: String::default(),
            man_netmask: String::default(),
            man_addui: text_input::State::new(),
            man_gatewayui: text_input::State::new(),
            man_netmaskui: text_input::State::new(),
            dns_input: text_input::State::new(),
            delete_manual: button::State::new(),
            route_addval: String::default(),
            route_netval: String::default(),
            route_gateval: String::default(),
            route_metricval: String::default(),
            delte_rotue: button::State::new(),
            route_addui: text_input::State::new(),
            route_gatewayui: text_input::State::new(),
            route_netmaskui: text_input::State::new(),
            route_metric: text_input::State::new(),
        }
    }
    pub fn update(&mut self, msg: IPv6Msg) {
        match msg {
            IPv6Msg::IPV6MethodChanged(value) => {
                self.selected = Some(value);
            }
            IPv6Msg::Add2Chagned(value) => {
                self.man_addr = value;
            }
            IPv6Msg::AddNetmastk(value) => {
                self.man_netmask = value;
            }
            IPv6Msg::AddGateway(value) => {
                self.man_gateway = value;
            }
            IPv6Msg::DeleteManual => {}
            IPv6Msg::DnsAutomatic(value) => {
                self.dns_auto = value;
            }
            IPv6Msg::DnsChanged(value) => {
                self.dns = value;
            }
            IPv6Msg::PrivateNetChagned(value) => {
                self.private_net = value;
            }
            IPv6Msg::RouteChagned(value) => {
                self.route_addval = value;
            }
            IPv6Msg::RouteGateway(value) => {
                self.route_netval = value;
            }
            IPv6Msg::RouteNetmastk(value) => {
                self.route_gateval = value;
            }
            IPv6Msg::RouteMetric(value) => {
                self.route_metricval = value;
            }
            IPv6Msg::DelteManualRoute => {}
        }
    }

    pub fn view(&mut self) -> Element<IPv6Msg> {
        let IPV6 {
            dns,
            dns_auto,
            private_net,
            selected,
            address,
            routes,
            man_addr,
            man_netmask,
            man_gateway,
            man_addui,
            man_netmaskui,
            man_gatewayui,
            dns_input,
            delete_manual,
            delte_rotue,
            route_addval,
            route_netval,
            route_gateval,
            route_metricval,
            route_addui,
            route_netmaskui,
            route_gatewayui,
            route_metric,
        } = self;

        let grid = IPV6Method::all().iter().cloned().fold(
            Grid::new().column_width(200),
            |layout, value| {
                layout.push(
                    Radio::new(value, value, *selected, IPv6Msg::IPV6MethodChanged)
                        .size(18)
                        .width(Length::Units(100)),
                )
            },
        );
        let data = match selected.unwrap() {
            IPV6Method::Manual => Column::new()
                .width(Length::Fill)
                .push(Column::new().push(Text::new("Address: ")))
                .push(
                    address
                        .iter()
                        .fold(Row::new().width(Length::Fill), |row, value| {
                            row.push(
                                Column::new()
                                    .width(Length::FillPortion(1))
                                    .align_items(Align::Center)
                                    .push(Text::new(value)),
                            )
                        }),
                )
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .width(Length::Fill)
                        .push(
                            TextInput::new(man_addui, "", man_addr, IPv6Msg::Add2Chagned)
                                .width(Length::FillPortion(1))
                                .padding(10),
                        )
                        .push(
                            TextInput::new(man_netmaskui, "", man_netmask, IPv6Msg::AddNetmastk)
                                .width(Length::FillPortion(1))
                                .padding(10),
                        )
                        .push(
                            TextInput::new(man_gatewayui, "", man_gateway, IPv6Msg::AddGateway)
                                .width(Length::FillPortion(1))
                                .padding(10),
                        )
                        .push(
                            Button::new(delete_manual, Text::new("Delete"))
                                .on_press(IPv6Msg::DeleteManual)
                                .padding(10),
                        ),
                ),
            _ => Column::new(),
        };
        let dns_content = Column::new()
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::Start)
                            .push(Text::new("DNS")),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::End)
                            .push(Checkbox::new(*dns_auto, "Automatic", IPv6Msg::DnsAutomatic)),
                    ),
            )
            .push(
                TextInput::new(dns_input, "", dns, IPv6Msg::DnsChanged)
                    .padding(10)
                    .width(Length::Fill),
            );

        let route_section = Column::new()
            .width(Length::Fill)
            .push(Column::new().push(Text::new("Routes: ")))
            .push(
                routes
                    .iter()
                    .fold(Row::new().width(Length::Fill), |row, value| {
                        row.push(
                            Column::new()
                                .width(Length::FillPortion(1))
                                .align_items(Align::Center)
                                .push(Text::new(value)),
                        )
                    }),
            )
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(
                        TextInput::new(route_addui, "", route_addval, IPv6Msg::RouteChagned)
                            .width(Length::FillPortion(2))
                            .padding(10),
                    )
                    .push(
                        TextInput::new(route_netmaskui, "", route_netval, IPv6Msg::RouteNetmastk)
                            .width(Length::FillPortion(2))
                            .padding(10),
                    )
                    .push(
                        TextInput::new(route_gatewayui, "", route_gateval, IPv6Msg::RouteGateway)
                            .width(Length::FillPortion(2))
                            .padding(10),
                    )
                    .push(
                        TextInput::new(route_metric, "", route_metricval, IPv6Msg::RouteMetric)
                            .width(Length::FillPortion(1))
                            .padding(10),
                    )
                    .push(
                        Button::new(delte_rotue, Text::new("Delete"))
                            .on_press(IPv6Msg::DelteManualRoute)
                            .padding(10),
                    ),
            );
        let first_section = Row::new()
            .push(Text::new("IPv6 Method").width(Length::Fill))
            .push(grid);
        let contetn: Element<_> = Column::new()
            .spacing(10)
            .push(first_section)
            .push(data)
            .push(dns_content)
            .push(route_section)
            .push(Checkbox::new(
                *private_net,
                "Use this connection only for resources on its network",
                IPv6Msg::PrivateNetChagned,
            ))
            .width(Length::Fill)
            .into();
        Container::new(contetn)
            .padding(10)
            .style(CustomContainer::ForegroundWhite)
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct ListCon {
    name: String,
    icon: String,
    status: String,
}
#[derive(Debug, Clone)]
pub enum ListMessage {}
impl ListCon {
    pub fn new(n: String, i: String, s: String) -> Self {
        Self {
            name: n,
            icon: i,
            status: s,
        }
    }
    pub fn view(&mut self) -> Element<ListMessage> {
        let ListCon { name, icon, status } = self;
        let data = Row::new()
            .spacing(5)
            .push(Svg::from_path(format!(
                "{}/assets/images/{}.svg",
                env!("CARGO_MANIFEST_DIR"),
                icon
            )))
            .push(
                Column::new()
                    .push(Text::new(name.as_str()).size(20))
                    .push(Text::new(status.as_str())),
            );
        data.into()
    }
}
