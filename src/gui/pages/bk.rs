
// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum Weekdays {
//    #[default]
//    Sun,
//    Mon,
//    Tue,
//    Wed,
//    Thu,
//    Fri,
//    Sat
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum Temperature {
//    #[default]
//    Celsius,
//    Fahrenheit,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum GroupSeperator {
//    #[default]
//    Dot,
//    Comma,
//    Space,
//    None,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum DecimalSeperator {
//    Dot,
//    #[default]
//    Comma,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum DateFormat {
//    #[default]
//    _dmy,
//    _dMy,
//    _dMMMMy,
//    _dMMMMyyyy,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum TimeFormat {
//    #[default]
//    _hma,
//    _Hm,
//    _hmsa,
//    _Hms,
//    _hmsaz,
//    _Hmsz,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum AppLang {
//    #[default]
//    Sys,
//    En,
//    Kh,
//    Ch,
//    Fr,
//    Sp,
//    Arabic
// }


// impl Weekdays {
//    const ALL:[Weekdays; 7] = [
//       Weekdays::Sun,
//       Weekdays::Mon,
//       Weekdays::Tue,
//       Weekdays::Wed,
//       Weekdays::Thu,
//       Weekdays::Fri,
//       Weekdays::Sat,
//    ];
// }

// impl Display for Weekdays {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       write!(f, "{}", match self {
//          Weekdays::Sun => "Sunday",
//          Weekdays::Mon => "Monday",
//          Weekdays::Tue => "Tuesday",
//          Weekdays::Wed => "Wednesday",
//          Weekdays::Thu => "Thursday",
//          Weekdays::Fri => "Friday",
//          Weekdays::Sat => "Saturday",
//       })
//    }
// }

// impl Temperature {
//    const ALL: [Temperature; 2] = [
//       Temperature::Celsius,
//       Temperature::Fahrenheit
//    ];
// }

// impl Display for Temperature {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       write!(f, "{}", match self {
//          Temperature::Celsius => "°C - Celsius",
//          Temperature::Fahrenheit => "°F - Fahrenheit"
//       })
//    }
// }

// impl GroupSeperator {
//    const ALL: [GroupSeperator; 4] = [
//       GroupSeperator::Dot,
//       GroupSeperator::Comma,
//       GroupSeperator::Space,
//       GroupSeperator::None
//    ];

//    pub fn as_str(&self) -> &'static str {
//       use GroupSeperator::*;
//       match self {
//          Dot => ".",
//          Comma => ",",
//          Space => " ",
//          None => ""
//       }
//    }
// }

// impl Display for GroupSeperator {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       use GroupSeperator::*;
//       write!(f, "{}", match self {
//          Dot => ".",
//          Comma => ",",
//          Space => "Space",
//          None => "None"
//       })
//    }
// }

// impl DecimalSeperator {
//    const ALL:[DecimalSeperator; 2] = [
//       DecimalSeperator::Dot,
//       DecimalSeperator::Comma,
//    ];

//    pub fn as_str(&self) -> &'static str {
//       use DecimalSeperator::*;
//       match self {
//          Dot => ".",
//          Comma => ",",
//       }
//    }
// }

// impl Display for DecimalSeperator {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       use DecimalSeperator::*;
//       write!(f, "{}", match self {
//          Dot => ".",
//          Comma => ","
//       })
//    }
// }

// impl DateFormat {
//    const ALL:[DateFormat; 4] = [
//       DateFormat::_dmy,
//       DateFormat::_dMy,
//       DateFormat::_dMMMMy,
//       DateFormat::_dMMMMyyyy,
//    ];

//    pub fn as_str(&self) -> &'static str {
//       use DateFormat::*;
//       match self {
//          _dmy => "%e/%m/%y",
//          _dMy => "%e/%b/%y",
//          _dMMMMy => "%e/%B/%y",
//          _dMMMMyyyy => "%e/%B/%Y"
//       }
//    }
// }

// impl Display for DateFormat {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       use DateFormat::*;
//       write!(f, "{}", match self {
//          _dmy => "d/m/y",
//          _dMy => "d/M/y",
//          _dMMMMy => "d/MMMM/y",
//          _dMMMMyyyy => "d/MMMM/yyyy"
//       })
//    }
// }

// impl TimeFormat {
//    const ALL:[TimeFormat; 6] = [
//       TimeFormat::_hma,
//       TimeFormat::_Hm,
//       TimeFormat::_hmsa,
//       TimeFormat::_Hms,
//       TimeFormat::_hmsaz,
//       TimeFormat::_Hmsz,
//    ];

//    pub fn as_str(&self) -> &'static str {
//       use TimeFormat::*;
//       match self {
//          _hma => "%I:%M %p",
//          _Hm => "%R",
//          _hmsa => "%r",
//          _Hms => "%X",
//          _hmsaz => "%I:%M:%S %p %Z",
//          _Hmsz => "%H:%M:%S %Z",
//       }
//    }
// }

// impl Display for TimeFormat {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       use TimeFormat::*;
//       write!(f, "{}", match self {
//          _hma => "h:m AM/PM",
//          _Hm => "H:m",
//          _hmsa => "h:m:s AM/PM",
//          _Hms => "H:m:s",
//          _hmsaz => "h:m:s AM/PM #",
//          _Hmsz => "H:m:s #",
//       })
//    }
// }

// impl AppLang {
//    const ALL:[AppLang; 7] = [
//       AppLang::Sys,
//       AppLang::En,
//       AppLang::Kh,
//       AppLang::Ch,
//       AppLang::Fr,
//       AppLang::Sp,
//       AppLang::Arabic
//    ];
// }

// impl Display for AppLang {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//       use AppLang::*;
//       write!(f, "{}", match self {
//          Sys => "System Language",
//          En => "English",
//          Kh => "Khmer",
//          Ch => "Chinese",
//          Fr => "French",
//          Sp => "Spanish",
//          Arabic => "Arabic"
//       })
//    }
// }