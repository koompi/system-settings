use iced::{
    button, pick_list, scrollable, text_input, Align, Button, Column, Container, Element,
    HorizontalAlignment, Length, PickList, Row, Rule, Scrollable, Space, Text, TextInput,
    VerticalAlignment,
};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Tab;
use icw::components::Toggler;
use icw::styles::{
    buttons::ButtonStyle, containers::ContainerStyle, pick_list::PickListStyle, rules::RuleStyle,
    text_input::InputStyle,
};
use std::fmt;
/// # use iced_native::{renderer::Null, Element, Grid as NativeGrid, Text};
// fn main() {
//     init();
// }
#[derive(Default, Debug, Clone)]
pub struct NetworkPage {
    choice: Choice,
    wireless: Wireless,
    wire: Wire,
    network: NetSettings,
    is_active: bool,
    scroll_content: scrollable::State,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}

#[derive(Debug, Clone)]
pub enum NetMessage {
    TabSelect(Choice),
    WirelessMsg(WirelessMsg),
    WireMsg(WireMsg),
    ToggleChange(bool),
    NetSettingsMsg(NetSettingsMsg),
}

impl NetworkPage {
    pub fn new() -> Self {
        Self {
            network: NetSettings::new(),
            wireless: Wireless::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: NetMessage) {
        match message {
            NetMessage::TabSelect(select) => {
                self.choice = select;
            }
            NetMessage::WirelessMsg(msg) => {
                self.wireless.update(msg);
            }
            NetMessage::ToggleChange(is_active) => {
                self.is_active = is_active;
            }
            NetMessage::WireMsg(msg) => {
                self.wire.update(msg);
            }
            NetMessage::NetSettingsMsg(msg) => {
                self.network.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<NetMessage> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Tab::new(
                    Choice::A,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f796}', "Ethernet"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::B,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f1eb}', "Wireless"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::C,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f6ff}', "DSL"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::D,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f3ed}', "VPN"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::E,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f7ba}', "System Proxy"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::F,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f7b9}', "Application Proxy"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::G,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f0c1}', "Personal Hotspot"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::H,
                    Some(self.choice),
                    NetMessage::TabSelect,
                    tab_content('\u{f05a}', "Network Details"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            );
        let contnet = Column::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .padding(20)
            .push(match self.choice {
                Choice::A => self.wire.view().map(move |msg| NetMessage::WireMsg(msg)),
                Choice::B => self
                    .wireless
                    .view()
                    .map(move |msg| NetMessage::WirelessMsg(msg)),
                Choice::C => self
                    .network
                    .view()
                    .map(move |msg| NetMessage::NetSettingsMsg(msg)),
                Choice::D => Text::new("Content D").into(),
                Choice::F => Text::new("Content F").into(),
                Choice::G => Text::new("Content G").into(),
                Choice::E => Text::new("Content E").into(),
                Choice::H => Text::new("Content H").into(),
            });
        let netsidebar_scroll = Scrollable::new(&mut self.scroll_content)
            .push(row)
            .padding(10)
            .scrollbar_width(4)
            .scroller_width(4);
        let whole_content: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(netsidebar_scroll.height(Length::Fill))
                    .style(ContainerStyle::White)
                    .width(Length::FillPortion(4))
                    .height(Length::Fill),
            )
            .push(Rule::vertical(10))
            .push(
                Container::new(contnet.height(Length::Fill))
                    .width(Length::FillPortion(9))
                    .height(Length::Fill)
                    .style(ContainerStyle::White), // .padding(10),
            )
            .into();
        let container = Container::new(whole_content)
            .width(Length::Fill)
            .center_x()
            .center_y();
        Container::new(container)
            .style(ContainerStyle::LightGray)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
fn tab_content<'a>(unicode: char, name: &str) -> Row<'a, NetMessage> {
    Row::new()
        .push(Icon::new(unicode).size(24))
        .push(Text::new(name).size(16))
        .align_items(Align::Center)
        .spacing(8)
}
// pub fn init() {
//     match NetworkPage::run(Settings::default()) {
//         Ok(val) => println!("run success with exit code: {:?}", val),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }

#[derive(Default, Debug, Clone)]
pub struct Wireless {
    is_active: bool,
    is_shown: bool,
    status: String,
    security: Option<String>,
    ssid: String,
    network_settings: NetSettings,
    ssid_vector: Vec<(button::State, bool, char, String)>,
}

impl Wireless {
    pub fn new() -> Self {
        let ssid_gen = |btn: button::State, secure: bool, icon: char, ssid: &str| {
            (btn, secure, icon, ssid.to_string())
        };
        let mut empty: Vec<(button::State, bool, char, String)> = Vec::new();
        for _ in 1..=10 {
            empty.push(ssid_gen(
                button::State::new(),
                true,
                '\u{f1eb}',
                "Koompi OS",
            ));
        }
        for i in &empty {
            println!("{:?}", i);
        }
        Self {
            ssid_vector: empty,
            is_shown: false,
            network_settings: NetSettings::new(),
            ..Self::default()
        }
    }
    pub fn update(&mut self, msg: WirelessMsg) {
        match msg {
            WirelessMsg::EnableWireless(value) => {
                self.is_active = value;
            }
            WirelessMsg::NothingButton => {}
            WirelessMsg::ShowSettings => {
                self.is_shown = !self.is_shown;
            }
            WirelessMsg::NetSettingsMsg(msg) => {
                self.network_settings.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<WirelessMsg> {
        println!("size of vector ssid: {}", self.ssid_vector.len());
        // .push(Toggler::new(
        //     self.is_active,
        //     String::from("Wire Network Adapter"),
        //     WirelessMsg::EnableWireless,
        // ))
        let wireless_layout = Column::new()
            .push(
                self.ssid_vector.iter_mut().fold(
                    Column::new()
                        .width(Length::Fill)
                        .spacing(4)
                        .height(Length::Fill),
                    |column, (state, status, icon, ssid)| {
                        column.push(
                            Row::new()
                                .align_items(Align::Center)
                                .padding(10)
                                .spacing(8)
                                .push(if *status {
                                    Icon::new('\u{f3ed}').size(16)
                                } else {
                                    Icon::new('\u{f09c}').size(16)
                                })
                                .push(Icon::new('\u{f1eb}').size(24))
                                .push(Text::new(ssid.as_str()).size(16))
                                .push(Space::with_width(Length::Fill))
                                .push(
                                    Button::new(state, Icon::new('\u{f105}'))
                                        .on_press(WirelessMsg::ShowSettings)
                                        .style(ButtonStyle::Transparent),
                                ),
                        )
                    },
                ),
            )
            .width(Length::Fill)
            .height(Length::Fill);
        let wifi_layout = Row::new()
            .push(
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(Toggler::new(
                        self.is_active,
                        String::from("Wireless Network Adapter"),
                        WirelessMsg::EnableWireless,
                    ))
                    .spacing(10)
                    .push(wireless_layout),
            )
            .push(Rule::vertical(10).style(RuleStyle {}))
            .push(if self.is_shown {
                self.network_settings
                    .view()
                    .map(move |msg| WirelessMsg::NetSettingsMsg(msg))
            } else {
                Space::with_width(Length::Shrink).into()
            });
        Container::new(wifi_layout)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
#[derive(Debug, Clone)]
pub enum WirelessMsg {
    EnableWireless(bool),
    NothingButton,
    ShowSettings,
    NetSettingsMsg(NetSettingsMsg),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Wire {
    is_enable: bool,
    add_net_con: button::State,
}
#[derive(Debug, Copy, Clone)]
pub enum WireMsg {
    EnableWired(bool),
    NetworkAdded,
}
impl Wire {
    fn new() -> Self {
        Self { ..Self::default() }
    }
    fn update(&mut self, msg: WireMsg) {
        match msg {
            WireMsg::EnableWired(is_enable) => {
                self.is_enable = is_enable;
            }
            WireMsg::NetworkAdded => {}
        }
    }
    fn view(&mut self) -> Element<WireMsg> {
        Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(Toggler::new(
                self.is_enable,
                String::from("Wire Network Adapter"),
                WireMsg::EnableWired,
            ))
            .push(
                Container::new(Text::new("Plug in the network cable first"))
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Units(100))
                    .style(ContainerStyle::LightGrayCircle),
            )
            .push(Space::with_height(Length::Fill))
            .push(
                Button::new(&mut self.add_net_con, Icon::new('\u{f067}').size(24))
                    .style(ButtonStyle::BigCircular(86, 101, 115, 1.0))
                    .padding(10)
                    .width(Length::Units(50))
                    .height(Length::Units(50))
                    .on_press(WireMsg::NetworkAdded),
            )
            .into()
    }
}

#[derive(Default, Debug, Clone)]
pub struct NetSettings {
    // Applicaiton State
    general: (String, bool),
    commonip: CommonIp,
    security: Security,
    wlan: Wlan,
    // Application Ui State
    // Security Part
    host_name: text_input::State,
    host_value: String,
    is_auto_conn: bool,
    is_custom_mtu: bool,
    is_shown_passwd: bool,
    is_shown_private_key: bool,
    passwd: String,
    passwd_name: text_input::State,
    pick_list: pick_list::State<SecurityType>,
    pick_list1: pick_list::State<PwdOption>,
    pick_list3: pick_list::State<Authentication>,
    pick_list4: pick_list::State<EAPAuth>,
    pick_listip4: pick_list::State<Ip4Method>,
    pick_listip6: pick_list::State<Ipv6Method>,
    pick_listwlan: pick_list::State<DeviceMacAddr>,
    selected_eapauth: EAPAuth,
    selected_auth: Authentication,
    selected_security: SecurityType,
    selected_pwdoption: PwdOption,
    selected_ip4: Ip4Method,
    selected_ip6: Ipv6Method,
    selected_wlan: DeviceMacAddr,
    toggle_show_passwd: button::State,
    identity: text_input::State,
    identity_val: String,
    private_pwd: text_input::State,
    private_pwd_val: String,
    private_key: text_input::State,
    private_key_val: String,
    ca_cert: text_input::State,
    ca_cert_val: String,
    user_cert: text_input::State,
    user_cert_val: String,
    private_pwd_file: button::State,
    private_key_file: button::State,
    ca_cert_file: button::State,
    user_cert_file: button::State,
    ip4_primary_dns_val: String,
    ip4_primary_dns: text_input::State,
    ip4_secondary_dns_val: String,
    ip4_secondary_dns: text_input::State,
    ip6_primary_dns_val: String,
    ip6_primary_dns: text_input::State,
    ip6_secondary_dns_val: String,
    ip6_secondary_dns: text_input::State,
    net_settings_scrolls: scrollable::State,

    // Wlan
    wlan_ssid: text_input::State,
    wlan_ssid_val: String,
    wlan_mtu_input: text_input::State,
    wlan_mtu_input_val: String,
    wlan_mtu_plus: button::State,
    wlan_mtu_minus: button::State,
    wlan_mtu_refresh: button::State,

    save_btn: button::State,
    cancel_btn: button::State,
    btn_state: ButtonState,
}

enum IP {
    IPV4,
    IPV6,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ip4Method {
    Auto,
    Manual,
}
impl Default for Ip4Method {
    fn default() -> Self {
        Ip4Method::Auto
    }
}
impl Ip4Method {
    const ALL: [Ip4Method; 2] = [Ip4Method::Auto, Ip4Method::Manual];
}

impl fmt::Display for Ip4Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ip4Method::Auto => "Auto",
                Ip4Method::Manual => "Manual",
            }
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv6Method {
    Auto,
    Manual,
    Ignore,
}
impl Default for Ipv6Method {
    fn default() -> Self {
        Ipv6Method::Auto
    }
}
impl Ipv6Method {
    const ALL: [Ipv6Method; 3] = [Ipv6Method::Auto, Ipv6Method::Manual, Ipv6Method::Ignore];
}
impl fmt::Display for Ipv6Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ipv6Method::Auto => "Auto",
                Ipv6Method::Manual => "Manual",
                Ipv6Method::Ignore => "Ignore",
            }
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceMacAddr {
    SOME,
    NONE,
}
impl Default for DeviceMacAddr {
    fn default() -> Self {
        DeviceMacAddr::NONE
    }
}
impl DeviceMacAddr {
    const ALL: [DeviceMacAddr; 2] = [DeviceMacAddr::NONE, DeviceMacAddr::SOME];
}
impl fmt::Display for DeviceMacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DeviceMacAddr::NONE => "Not Bind",
                DeviceMacAddr::SOME => "30:3A:64:AD:2B:77 (wlan0)",
            }
        )
    }
}

#[derive(Default, Debug, Clone)]
struct CommonIp {
    Method: Vec<String>,
    PrimaryDns: String,
    SecondaryDns: String,
}
#[derive(Default, Debug, Clone)]
struct Security {
    Type: Vec<String>,
    PwdOption: Vec<u8>,
    Pwd: String,
}

#[derive(Default, Debug, Clone)]
struct Wlan {
    Ssid: String,
    DevMacAddr: Option<String>,
    CustomMtu: bool,
}

#[derive(Debug, Clone)]
pub enum NetSettingsMsg {
    HostChanged(String),
    HostSubmit,
    AutoConnMutated(bool),
    LanguageSelected(SecurityType),
    PwdOptionSelected(PwdOption),
    PasswordInput(String),
    ToggleShownPasswd,
    AuthChanged(Authentication),
    EAPAuthChanged(EAPAuth),
    Ip4MethodChanged(Ip4Method),
    Ip6MethodChanged(Ipv6Method),
    IdentityChanged(String),
    PrivatePwdChanged(String),
    PrivateKeyChanged(String),
    CaCertChanged(String),
    UserCertChanged(String),
    PrimaryDnsIp4(String),
    SecondaryDnsIp4(String),
    PrimaryDnsIp6(String),
    SecondaryDnsIp6(String),
    ToggleKey,
    OpenFile1,
    OpenFile2,
    OpenFile3,
    // WLAN
    WlanSsidChanged(String),
    DevMacAddrChanged(DeviceMacAddr),
    CustomMtuChanged(bool),
    WlanMtuInput(String),
    WlanMtuPlus,
    WlanMtuMinus,
    WlanMtuRefresh,
    OnSave,
    OnCancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    NONE,
    WEP,
    WPA_WPA2_PERSONAL,
    WPA_WPA2_ENTERPRISE,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authentication {
    SharedKey,
    OpenSystem,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PwdOption {
    OneUser,
    AllUser,
    AskFirst,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EAPAuth {
    TLS,
    LEAP,
    FAST,
    TUNNELEDTLS,
    PROTECTEDEAP,
}
#[derive(Debug, Clone)]
pub enum ButtonState {
    Disable,
    Enable,
}
impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Disable
    }
}
impl EAPAuth {
    const ALL: [EAPAuth; 5] = [
        EAPAuth::TLS,
        EAPAuth::LEAP,
        EAPAuth::FAST,
        EAPAuth::TUNNELEDTLS,
        EAPAuth::PROTECTEDEAP,
    ];
}
impl Default for EAPAuth {
    fn default() -> Self {
        EAPAuth::TLS
    }
}

impl fmt::Display for EAPAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EAPAuth::TLS => "TLS",
                EAPAuth::LEAP => "LEAP",
                EAPAuth::FAST => "FAST",
                EAPAuth::TUNNELEDTLS => "Tunneled TLS",
                EAPAuth::PROTECTEDEAP => "Protected EAP",
            }
        )
    }
}
impl Authentication {
    const ALL: [Authentication; 2] = [Authentication::SharedKey, Authentication::OpenSystem];
}
impl Default for Authentication {
    fn default() -> Self {
        Authentication::SharedKey
    }
}
impl fmt::Display for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Authentication::SharedKey => "ShareKey",
                Authentication::OpenSystem => "OpenSystem",
            }
        )
    }
}
impl PwdOption {
    const ALL: [PwdOption; 3] = [PwdOption::OneUser, PwdOption::AllUser, PwdOption::OneUser];
}
impl Default for PwdOption {
    fn default() -> Self {
        PwdOption::OneUser
    }
}
impl fmt::Display for PwdOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PwdOption::OneUser => "Save password for this user",
                PwdOption::AllUser => "Save password for all users",
                PwdOption::AskFirst => "Ask me always",
            }
        )
    }
}
impl SecurityType {
    const ALL: [SecurityType; 4] = [
        SecurityType::NONE,
        SecurityType::WEP,
        SecurityType::WPA_WPA2_PERSONAL,
        SecurityType::WPA_WPA2_ENTERPRISE,
    ];
}

impl Default for SecurityType {
    fn default() -> Self {
        SecurityType::WPA_WPA2_PERSONAL
    }
}
impl std::fmt::Display for SecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SecurityType::NONE => "None",
                SecurityType::WEP => "WEP",
                SecurityType::WPA_WPA2_PERSONAL => "WPA/WPA2 Personal",
                SecurityType::WPA_WPA2_ENTERPRISE => "WPA/WPA2 Enterprise",
            }
        )
    }
}

fn default_text(text: &str) -> Text {
    Text::new(text).size(16)
}
impl NetSettings {
    fn new() -> Self {
        Self {
            is_shown_passwd: true,
            is_custom_mtu: false,
            wlan_mtu_input_val: "0".to_string(),
            ..Self::default()
        }
    }

    fn update(&mut self, msg: NetSettingsMsg) {
        match msg {
            NetSettingsMsg::HostChanged(name) => {
                self.host_value = name;
            }
            NetSettingsMsg::HostSubmit => {}
            NetSettingsMsg::AutoConnMutated(is_auto_conn) => {
                self.is_auto_conn = is_auto_conn;
            }
            NetSettingsMsg::LanguageSelected(lang) => {
                self.selected_security = lang;
            }
            NetSettingsMsg::PwdOptionSelected(opts) => {
                self.selected_pwdoption = opts;
            }
            NetSettingsMsg::PasswordInput(password) => {
                self.passwd = password;
            }
            NetSettingsMsg::ToggleShownPasswd => {
                self.is_shown_passwd = !self.is_shown_passwd;
            }
            NetSettingsMsg::AuthChanged(auth) => {
                self.selected_auth = auth;
            }
            NetSettingsMsg::EAPAuthChanged(eapauth) => {
                self.selected_eapauth = eapauth;
            }
            NetSettingsMsg::IdentityChanged(identity) => {
                self.identity_val = identity;
            }
            NetSettingsMsg::PrivatePwdChanged(ppwd) => {
                self.private_pwd_val = ppwd;
            }
            NetSettingsMsg::PrivateKeyChanged(pkey) => {
                self.private_key_val = pkey;
            }
            NetSettingsMsg::CaCertChanged(cacert) => {
                self.ca_cert_val = cacert;
            }
            NetSettingsMsg::UserCertChanged(usercert) => {
                self.user_cert_val = usercert;
            }
            NetSettingsMsg::Ip4MethodChanged(ip4) => {
                self.selected_ip4 = ip4;
            }
            NetSettingsMsg::Ip6MethodChanged(ip6) => {
                self.selected_ip6 = ip6;
            }
            NetSettingsMsg::PrimaryDnsIp4(ip4_dns_val) => {
                self.ip4_primary_dns_val = ip4_dns_val;
            }
            NetSettingsMsg::SecondaryDnsIp4(ip4_second_val) => {
                self.ip4_secondary_dns_val = ip4_second_val;
            }
            NetSettingsMsg::PrimaryDnsIp6(ip6_dns_val) => {
                self.ip6_primary_dns_val = ip6_dns_val;
            }
            NetSettingsMsg::SecondaryDnsIp6(ip6_second_val) => {
                self.ip6_secondary_dns_val = ip6_second_val;
            }
            NetSettingsMsg::ToggleKey => {
                self.is_shown_private_key = !self.is_shown_private_key;
            }
            NetSettingsMsg::WlanSsidChanged(ssid) => {
                self.wlan_ssid_val = ssid;
            }
            NetSettingsMsg::DevMacAddrChanged(dev_mac_addr) => {
                self.selected_wlan = dev_mac_addr;
            }
            NetSettingsMsg::CustomMtuChanged(is_shown) => {
                self.is_custom_mtu = is_shown;
            }
            NetSettingsMsg::OnCancel => {}
            NetSettingsMsg::OnSave => {}
            NetSettingsMsg::WlanMtuInput(mtu_val) => {
                if mtu_val.len() <= 4 {
                    match mtu_val.parse::<u32>() {
                        Ok(_) => self.wlan_mtu_input_val = mtu_val,
                        Err(e) => eprintln!("Error: {:?}", e),
                    }
                } else {
                    {}
                }
            }
            NetSettingsMsg::WlanMtuPlus => match self.wlan_mtu_input_val.parse::<u32>() {
                Ok(data) => {
                    if data <= 9999 {
                        self.wlan_mtu_input_val = (data + 1).to_string()
                    } else {
                        {}
                    }
                }
                Err(e) => eprintln!("Error: {:?}", e),
            },

            NetSettingsMsg::WlanMtuMinus => match self.wlan_mtu_input_val.parse::<u32>() {
                Ok(data) => {
                    if data > 0 {
                        self.wlan_mtu_input_val = (data - 1).to_string()
                    } else {
                        {}
                    }
                }
                Err(e) => eprintln!("Error: {:?}", e),
            },
            NetSettingsMsg::WlanMtuRefresh => {
                self.wlan_mtu_input_val = "0".to_string();
            }
            NetSettingsMsg::OpenFile1 => {}
            NetSettingsMsg::OpenFile2 => {}
            NetSettingsMsg::OpenFile3 => {}
        }
    }
    fn view(&mut self) -> Element<NetSettingsMsg> {
        let NetSettings {
            host_name,
            host_value,
            is_auto_conn,
            passwd,
            passwd_name,
            identity,
            identity_val,
            private_pwd,
            private_pwd_val,
            private_key,
            private_key_val,
            ca_cert,
            ca_cert_val,
            user_cert,
            user_cert_val,
            private_pwd_file,
            private_key_file,
            ca_cert_file,
            user_cert_file,
            net_settings_scrolls,
            ..
        } = self;
        let net_layout = Column::new();
        let general = Column::new()
            .spacing(10)
            .push(Text::new("General").size(24))
            .push(
                Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .padding(10)
                        .push(
                            Row::new()
                                .push(default_text("Name").width(Length::FillPortion(1)))
                                .align_items(Align::Center)
                                .push(
                                    TextInput::new(
                                        host_name,
                                        "Koompi Attic",
                                        host_value,
                                        NetSettingsMsg::HostChanged,
                                    )
                                    .padding(6)
                                    .style(InputStyle::InkBorder)
                                    .width(Length::FillPortion(2))
                                    .on_submit(NetSettingsMsg::HostSubmit),
                                ),
                        )
                        .push(Rule::horizontal(4).style(RuleStyle {}))
                        .push(
                            Row::new()
                                .push(default_text("Auto Connect"))
                                .push(Space::with_width(Length::Fill))
                                .push(Toggler::new(
                                    *is_auto_conn,
                                    "".to_string(),
                                    NetSettingsMsg::AutoConnMutated,
                                )),
                        ),
                )
                .width(Length::Fill)
                .style(ContainerStyle::LightGrayCircle),
            );
        let security = Column::new()
            .spacing(10)
            .push(Text::new("Security").size(24))
            .push(
                Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .padding(10)
                        .push(
                            Row::new()
                                .push(default_text("Security").width(Length::FillPortion(1)))
                                .align_items(Align::Center)
                                .push(
                                    PickList::new(
                                        &mut self.pick_list,
                                        &SecurityType::ALL[..],
                                        Some(self.selected_security),
                                        NetSettingsMsg::LanguageSelected,
                                    )
                                    .text_size(16)
                                    .style(PickListStyle {})
                                    .padding(6)
                                    .width(Length::FillPortion(2)),
                                )
                                .width(Length::FillPortion(3)),
                        )
                        .push(match self.selected_security {
                            SecurityType::NONE => {
                                Container::new(Space::with_height(Length::Units(0)))
                            }
                            SecurityType::WEP => Container::new(
                                Column::new()
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Pwd Options")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .align_items(Align::Center)
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list1,
                                                    &PwdOption::ALL[..],
                                                    Some(self.selected_pwdoption),
                                                    NetSettingsMsg::PwdOptionSelected,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(Rule::horizontal(10).style(RuleStyle {}))
                                    .push(
                                        Row::new()
                                            .align_items(Align::Center)
                                            .push(default_text("Key").width(Length::FillPortion(1)))
                                            .spacing(4)
                                            .push(if self.is_shown_passwd {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            } else {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .password()
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            })
                                            .push(
                                                Button::new(
                                                    &mut self.toggle_show_passwd,
                                                    Icon::new(if self.is_shown_passwd {
                                                        '\u{f06e}'
                                                    } else {
                                                        '\u{f070}'
                                                    }),
                                                )
                                                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                                                .on_press(NetSettingsMsg::ToggleShownPasswd),
                                            ),
                                    )
                                    .push(Rule::horizontal(10).style(RuleStyle {}))
                                    .push(
                                        Row::new()
                                            .align_items(Align::Center)
                                            .push(
                                                default_text("Authentication")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list3,
                                                    &Authentication::ALL[..],
                                                    Some(self.selected_auth),
                                                    NetSettingsMsg::AuthChanged,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    ),
                            ),
                            SecurityType::WPA_WPA2_PERSONAL => Container::new(
                                Column::new()
                                    .spacing(10)
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Pwd Options")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .align_items(Align::Center)
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list1,
                                                    &PwdOption::ALL[..],
                                                    Some(self.selected_pwdoption),
                                                    NetSettingsMsg::PwdOptionSelected,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(Rule::horizontal(10).style(RuleStyle {}))
                                    .push(
                                        Row::new()
                                            .align_items(Align::Center)
                                            .push(
                                                default_text("Password")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .spacing(4)
                                            .push(if self.is_shown_passwd {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            } else {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .password()
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            })
                                            .push(
                                                Button::new(
                                                    &mut self.toggle_show_passwd,
                                                    Icon::new(if self.is_shown_passwd {
                                                        '\u{f06e}'
                                                    } else {
                                                        '\u{f070}'
                                                    }),
                                                )
                                                .style(ButtonStyle::Circular(65, 203, 126, 1.0))
                                                .on_press(NetSettingsMsg::ToggleShownPasswd),
                                            ),
                                    ),
                            ),
                            SecurityType::WPA_WPA2_ENTERPRISE => Container::new(
                                Column::new()
                                    .padding(10)
                                    .spacing(10)
                                    .push(
                                        Row::new().push(
                                            PickList::new(
                                                &mut self.pick_list4,
                                                &EAPAuth::ALL[..],
                                                Some(self.selected_eapauth),
                                                NetSettingsMsg::EAPAuthChanged,
                                            )
                                            .text_size(16)
                                            .style(PickListStyle {})
                                            .padding(6)
                                            .width(Length::FillPortion(2)),
                                        ),
                                    )
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Pwd Options")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .align_items(Align::Center)
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list1,
                                                    &PwdOption::ALL[..],
                                                    Some(self.selected_pwdoption),
                                                    NetSettingsMsg::PwdOptionSelected,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Identity")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    identity,
                                                    "Required",
                                                    identity_val,
                                                    NetSettingsMsg::IdentityChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("Private Pwd")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(if self.is_shown_private_key {
                                                TextInput::new(
                                                    private_pwd,
                                                    "Required",
                                                    private_pwd_val,
                                                    NetSettingsMsg::PrivatePwdChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            } else {
                                                TextInput::new(
                                                    private_pwd,
                                                    "Required",
                                                    private_pwd_val,
                                                    NetSettingsMsg::PrivatePwdChanged,
                                                )
                                                .password()
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            })
                                            .push(
                                                Button::new(
                                                    private_pwd_file,
                                                    Icon::new(if self.is_shown_private_key {
                                                        '\u{f06e}'
                                                    } else {
                                                        '\u{f070}'
                                                    }),
                                                )
                                                .on_press(NetSettingsMsg::ToggleKey),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("Private Key")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    private_key,
                                                    "",
                                                    private_key_val,
                                                    NetSettingsMsg::PrivateKeyChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            )
                                            .push(
                                                Button::new(
                                                    private_key_file,
                                                    Icon::new('\u{f0c6}'),
                                                )
                                                .on_press(NetSettingsMsg::OpenFile1)
                                                .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("CA Cert")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    ca_cert,
                                                    "",
                                                    ca_cert_val,
                                                    NetSettingsMsg::CaCertChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            )
                                            .push(
                                                Button::new(ca_cert_file, Icon::new('\u{f0c6}'))
                                                    .on_press(NetSettingsMsg::OpenFile2)
                                                    .style(ButtonStyle::Circular(
                                                        86, 101, 115, 1.0,
                                                    )),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("User Cert")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    user_cert,
                                                    "",
                                                    user_cert_val,
                                                    NetSettingsMsg::UserCertChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            )
                                            .push(
                                                Button::new(user_cert_file, Icon::new('\u{f0c6}'))
                                                    .on_press(NetSettingsMsg::OpenFile3)
                                                    .style(ButtonStyle::Circular(
                                                        86, 101, 115, 1.0,
                                                    )),
                                            ),
                                    ),
                            )
                            .width(Length::Fill),
                        }),
                )
                .style(ContainerStyle::LightGrayCircle)
                .width(Length::Fill),
            );
        let ipv4 = Container::new(
            Column::new()
                .spacing(10)
                .push(Text::new("IPv4").size(24))
                .push(
                    Container::new(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(default_text("Method"))
                                    .push(
                                        PickList::new(
                                            &mut self.pick_listip4,
                                            &Ip4Method::ALL[..],
                                            Some(self.selected_ip4),
                                            NetSettingsMsg::Ip4MethodChanged,
                                        )
                                        .text_size(16)
                                        .style(PickListStyle {})
                                        .padding(6)
                                        .width(Length::FillPortion(2)),
                                    ),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(default_text("Primary DNS").width(Length::FillPortion(1)))
                                    .push(
                                        TextInput::new(
                                            &mut self.ip4_primary_dns,
                                            "",
                                            &self.ip4_primary_dns_val,
                                            NetSettingsMsg::PrimaryDnsIp4,
                                        )
                                        .padding(6)
                                        .style(InputStyle::InkBorder)
                                        .width(Length::FillPortion(2)),
                                    ),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(
                                        default_text("Secondary DNS").width(Length::FillPortion(1)),
                                    )
                                    .push(
                                        TextInput::new(
                                            &mut self.ip4_secondary_dns,
                                            "",
                                            &self.ip4_secondary_dns_val,
                                            NetSettingsMsg::SecondaryDnsIp4,
                                        )
                                        .padding(6)
                                        .style(InputStyle::InkBorder)
                                        .width(Length::FillPortion(2)),
                                    ),
                            ),
                    )
                    .style(ContainerStyle::LightGrayCircle),
                ),
        );
        let ipv6 = Container::new(
            Column::new()
                .spacing(10)
                .push(Text::new("IPv4").size(24))
                .push(
                    Container::new(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(default_text("Method"))
                                    .push(
                                        PickList::new(
                                            &mut self.pick_listip6,
                                            &Ipv6Method::ALL[..],
                                            Some(self.selected_ip6),
                                            NetSettingsMsg::Ip6MethodChanged,
                                        )
                                        .text_size(16)
                                        .style(PickListStyle {})
                                        .padding(6)
                                        .width(Length::FillPortion(2)),
                                    ),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(default_text("Primary DNS").width(Length::FillPortion(1)))
                                    .push(
                                        TextInput::new(
                                            &mut self.ip6_primary_dns,
                                            "",
                                            &self.ip6_primary_dns_val,
                                            NetSettingsMsg::PrimaryDnsIp6,
                                        )
                                        .padding(6)
                                        .style(InputStyle::InkBorder)
                                        .width(Length::FillPortion(2)),
                                    ),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(
                                        default_text("Secondary DNS").width(Length::FillPortion(1)),
                                    )
                                    .push(
                                        TextInput::new(
                                            &mut self.ip6_secondary_dns,
                                            "",
                                            &self.ip6_secondary_dns_val,
                                            NetSettingsMsg::SecondaryDnsIp6,
                                        )
                                        .padding(6)
                                        .style(InputStyle::InkBorder)
                                        .width(Length::FillPortion(2)),
                                    ),
                            ),
                    )
                    .style(ContainerStyle::LightGrayCircle),
                ),
        );
        let wlan = Column::new().push(
            Column::new()
                .spacing(10)
                .push(Text::new("WLAN").size(24))
                .push(
                    Container::new(
                        Column::new()
                            .spacing(10)
                            .padding(10)
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(default_text("SSID").width(Length::FillPortion(1)))
                                    .push(
                                        TextInput::new(
                                            &mut self.wlan_ssid,
                                            "",
                                            &self.wlan_ssid_val,
                                            NetSettingsMsg::WlanSsidChanged,
                                        )
                                        .padding(6)
                                        .style(InputStyle::InkBorder)
                                        .width(Length::FillPortion(2)),
                                    ),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(
                                        default_text("Device Mac Address")
                                            .width(Length::FillPortion(1)),
                                    )
                                    .push(
                                        PickList::new(
                                            &mut self.pick_listwlan,
                                            &DeviceMacAddr::ALL[..],
                                            Some(self.selected_wlan),
                                            NetSettingsMsg::DevMacAddrChanged,
                                        )
                                        .text_size(16)
                                        .style(PickListStyle {})
                                        .padding(6)
                                        .width(Length::FillPortion(2)),
                                    ),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(default_text("Customize MTU"))
                                    .push(Space::with_width(Length::Fill))
                                    .push(Toggler::new(
                                        self.is_custom_mtu,
                                        String::from(""),
                                        NetSettingsMsg::CustomMtuChanged,
                                    )),
                            )
                            .push(Rule::horizontal(10).style(RuleStyle {}))
                            .push(if self.is_custom_mtu {
                                Container::new(
                                    Row::new()
                                        .padding(10)
                                        .spacing(4)
                                        .align_items(Align::Center)
                                        .push(default_text("MTU").width(Length::FillPortion(1)))
                                        .push(
                                            Row::new()
                                                .spacing(10)
                                                .width(Length::FillPortion(3))
                                                .push(
                                                    TextInput::new(
                                                        &mut self.wlan_mtu_input,
                                                        "",
                                                        &self.wlan_mtu_input_val,
                                                        NetSettingsMsg::WlanMtuInput,
                                                    )
                                                    .padding(6)
                                                    .style(InputStyle::InkBorder)
                                                    .width(Length::FillPortion(2)),
                                                )
                                                .push(
                                                    Button::new(
                                                        &mut self.wlan_mtu_plus,
                                                        Icon::new('\u{f067}'),
                                                    )
                                                    .on_press(NetSettingsMsg::WlanMtuPlus)
                                                    .style(ButtonStyle::Circular(
                                                        215, 219, 221, 0.5,
                                                    )),
                                                )
                                                .push(
                                                    Button::new(
                                                        &mut self.wlan_mtu_minus,
                                                        Icon::new('\u{f068}'),
                                                    )
                                                    .on_press(NetSettingsMsg::WlanMtuMinus)
                                                    .style(ButtonStyle::Circular(
                                                        215, 219, 221, 0.5,
                                                    )),
                                                )
                                                .push(
                                                    Button::new(
                                                        &mut self.wlan_mtu_refresh,
                                                        Icon::new('\u{f2f9}'),
                                                    )
                                                    .on_press(NetSettingsMsg::WlanMtuRefresh)
                                                    .style(ButtonStyle::Circular(
                                                        215, 219, 221, 0.5,
                                                    )),
                                                ),
                                        ),
                                )
                            } else {
                                Container::new(Space::with_height(Length::Shrink))
                            }),
                    )
                    .style(ContainerStyle::LightGrayCircle),
                ),
        );
        let network_scroll = Scrollable::new(net_settings_scrolls).push(
            net_layout
                .spacing(20)
                .push(general)
                .push(security)
                .push(ipv4)
                .push(ipv6)
                .push(wlan),
        );
        let whole_settings = Column::new()
            .push(
                network_scroll
                    .padding(20)
                    .scroller_width(4)
                    .scrollbar_width(4),
            )
            .push(
                Row::new()
                    .push(Text::new("Hello wrold"))
                    .width(Length::Fill)
                    .height(Length::Units(100))
                    .spacing(4)
                    .align_items(Align::Center)
                    .push(match self.btn_state {
                        ButtonState::Enable => Button::new(
                            &mut self.cancel_btn,
                            Text::new("Cancel")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::Fill)
                        .padding(6)
                        .style(ButtonStyle::BigCircular(86, 101, 115, 1.0))
                        .on_press(NetSettingsMsg::OnCancel),
                        ButtonState::Disable => Button::new(
                            &mut self.cancel_btn,
                            Text::new("Cancel")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::Fill)
                        .padding(6)
                        .style(ButtonStyle::BigCircular(86, 101, 115, 1.0)),
                    })
                    .push(match self.btn_state {
                        ButtonState::Enable => Button::new(
                            &mut self.save_btn,
                            Text::new("Save")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::Fill)
                        .padding(6)
                        .style(ButtonStyle::BigCircular(86, 101, 115, 1.0))
                        .on_press(NetSettingsMsg::OnSave),
                        ButtonState::Disable => Button::new(
                            &mut self.save_btn,
                            Text::new("Save")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::Fill)
                        .padding(6)
                        .style(ButtonStyle::BigCircular(86, 101, 115, 1.0)),
                    }),
            );
        Container::new(whole_settings)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_connection() {}
}

// helper functions
// fn apply_row<'a, F>(
//     name: &str,
//     input: &'a mut text_input::State,
//     value: &'a str,
//     f: F,
// ) -> Element<'a, UserInfomsg>
// where
//     F: Fn(String) -> UserInfomsg + 'static,
// {
//     if name.eq_ignore_ascii_case("password") || name.eq_ignore_ascii_case("confirm") {
//         Row::new()
//             .align_items(Align::Center)
//             .push(Text::new(name).size(FONT_SIZE).width(Length::Units(100)))
//             .push(
//                 TextInput::new(input, "", &value, f)
//                     .password()
//                     .size(FONT_SIZE)
//                     .padding(10),
//             )
//             .into()
//     } else {
//         Row::new()
//             .align_items(Align::Center)
//             .push(Text::new(name).size(FONT_SIZE).width(Length::Units(100)))
//             .push(
//                 TextInput::new(input, "", &value, f)
//                     .size(FONT_SIZE)
//                     .padding(10),
//             )
//             .into()
//     }
// }
