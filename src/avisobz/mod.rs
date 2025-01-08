mod task;
use std::{
    io::{stdout, Write},
    sync::atomic::Ordering,
    time::Duration,
};

use chrono::{TimeZone, Utc};
use chrono_tz::{America::Sao_Paulo, Tz};
use regex::Regex;
use task::ThreadTaskAvisobz;
use tokio::time::sleep;

use crate::{colors::Colors, config::UserData, GLOBAL_CONTROL};

#[allow(dead_code)]
pub enum Mode {
    YOUTUBE,
    SURFING,
    ALL,
}

pub struct Avisobz;

impl Avisobz {
    pub async fn start(mode: Mode, headless: bool) {
        let c = Colors::new().await;
        print!("\x1bc");
        print!("{}[{}AVISOBZ{}]\n", c.CIAN, c.WHITE, c.CIAN);
        stdout().flush().unwrap();
        let thread = ThreadTaskAvisobz { headless };
        match mode {
            Mode::YOUTUBE => thread.youtube().await,
            Mode::SURFING => thread.surfing().await,
            Mode::ALL => thread.all().await,
        }
    }

    pub async fn add(email: &str, passw: &str, cookies: &str, proxy: &str) {
        UserData::create(email, passw, cookies, proxy).await;
    }

    pub async fn modify(email: &str, passw: &str, cookies: &str, proxy: &str) {
        UserData::modify(email, passw, cookies, proxy).await;
    }

    pub async fn delete() {
        UserData::delete().await;
    }
}

pub struct Print {
    task: i32,
    username: String,
    classification: String,
    money: f32,
}

impl Print {
    pub async fn user(&self) {
        let c = Colors::new().await;
        print!(
            "\r\x1b[K{}[{}{}{}][{}{}{}|{}{}{}|{}{:.2}{}][{}{}{}]",
            c.CIAN,
            c.WHITE,
            self.task,
            c.CIAN,
            c.WHITE,
            self.username.to_uppercase(),
            c.CIAN,
            c.WHITE,
            self.classification,
            c.CIAN,
            c.WHITE,
            self.money,
            c.CIAN,
            c.WHITE,
            time_now(Sao_Paulo).await,
            c.CIAN
        );
        stdout().flush().unwrap();
    }

    pub async fn tmr(&self, mode: &str, tmr: &str) {
        let c = Colors::new().await;
        print!(
            "\r\x1b[K{}[{}{}{}][{}{}{}|{}{}{}|{}{}{}|{}{}{}][{}{}{}]",
            c.CIAN,
            c.WHITE,
            self.task,
            c.CIAN,
            c.WHITE,
            self.username.to_uppercase(),
            c.CIAN,
            c.WHITE,
            self.classification,
            c.CIAN,
            c.WHITE,
            mode,
            c.CIAN,
            c.WHITE,
            tmr,
            c.CIAN,
            c.WHITE,
            time_now(Sao_Paulo).await,
            c.CIAN
        );
        stdout().flush().unwrap();
    }

    pub async fn earn(&self, earn: &str) {
        let c = Colors::new().await;
        let re = Regex::new(r"(\d.\d\d\d)").unwrap();
        let earn = re.captures(earn).unwrap();
        print!(
            "\r\x1b[K{}[{}{}{}][{}{}{}|{}{}{}|{}{:.2}{}|{}{}{}][{}{}{}]\n",
            c.CIAN,
            c.WHITE,
            self.task,
            c.CIAN,
            c.WHITE,
            self.username.to_uppercase(),
            c.CIAN,
            c.WHITE,
            self.classification,
            c.CIAN,
            c.WHITE,
            self.money,
            c.CIAN,
            c.WHITE,
            &earn[1],
            c.CIAN,
            c.WHITE,
            time_now(Sao_Paulo).await,
            c.CIAN
        );
        stdout().flush().unwrap();
    }

    pub async fn pause() {
        let colors = Colors::new().await;
        for i in (1..=600).rev() {
            if GLOBAL_CONTROL.load(Ordering::Relaxed) {
                break;
            }
            print!(
                "\r\x1b[K{}[{}PAUSED{}]({}{}{})",
                colors.CIAN, colors.YELLOW, colors.CIAN, colors.YELLOW, i, colors.CIAN
            );
            stdout().flush().unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    }
}

pub async fn time_now(state: Tz) -> String {
    let time_now = Utc::now();

    //convert time_now to NaiveDateTime
    let naive_date_time = time_now.naive_local();
    let hour_sp = state.from_utc_datetime(&naive_date_time).format("%H:%M");
    hour_sp.to_string()
}
