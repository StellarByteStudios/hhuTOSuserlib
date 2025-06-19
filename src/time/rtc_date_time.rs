use crate::kernel::syscall::user_api::{usr_get_systime, usr_get_datetime, usr_get_pid_interval};
use core::convert::TryFrom;
use alloc::string::String;
use alloc::format;

pub struct RtcDate {
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl RtcDate {
    pub fn format(&self) -> String {
        format!(
            "{:02}.{:02}.{:02}",
            self.day,
            self.month,
            self.year,
        )
    }
}

pub struct RtcTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
}

impl RtcTime {
    pub fn format(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.hours,
            self.minutes,
            self.seconds,
        )
    }
}

pub struct RtcDateTime {
    pub date: RtcDate,
    pub time: RtcTime,
}

impl RtcDateTime {
    pub fn format(&self) -> String {
        format!(
            "{:02}.{:02}.{:02}    {:02}:{:02}:{:02}",
            self.date.day,
            self.date.month,
            self.date.year,
            self.time.hours,
            self.time.minutes,
            self.time.seconds,
        )
    }
}

pub fn systime() -> RtcTime {
    let systime = usr_get_systime();
    let interval = usr_get_pid_interval();

    let total_millis = systime * interval;
    let total_secs = total_millis / 1000;
    RtcTime {
        seconds: u8::try_from(total_secs % 60).unwrap_or(0),
        minutes: u8::try_from((total_secs % 3600) / 60).unwrap_or(0),
        hours: u8::try_from(total_secs / 3600).unwrap_or(0),
    } 
}

pub fn datetime() -> RtcDateTime {
    let mut dt = RtcDateTime { 
        date: RtcDate { day: 0, month: 0, year: 0 },
        time: RtcTime { seconds: 0, minutes: 0, hours: 0 },
    };
    usr_get_datetime(&mut dt as *mut RtcDateTime);
    dt
}
