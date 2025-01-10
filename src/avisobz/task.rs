use regex::Regex;
use std::{path::Path, process::exit, sync::atomic::Ordering, time::Duration};
use thirtyfour::prelude::*;
use tokio::{signal::ctrl_c, spawn, time::sleep};

use crate::{
    browser::{start_driver, Browser},
    config::{Log, UserData},
    GLOBAL_CONTROL,
};

use super::Print;

#[allow(dead_code)]
enum TaskResult {
    QUIT,
    CRITICAL,
    OK,
    PAUSE,
    CONTINUE,
}

#[derive(Clone)]
struct TaskDriverAvisobz {
    headless: bool,
}

impl TaskDriverAvisobz {
    async fn login(self) -> Result<WebDriver, WebDriverError> {
        let user = UserData::load().await;
        Log::debug("TaskDriverSeofast-LOGIN", &user.port).await;
        let browser = Browser {
            headless: self.headless,
            proxy: Some(user.proxy.clone()),
            port: user.port,
        };

        let driver = browser.new().await;

        sleep(Duration::from_secs(4)).await;

        let _ = driver.set_page_load_timeout(Duration::from_secs(20)).await;

        if let Err(e) = driver.get("https://aviso.bz/login").await {
            Log::error(
                "TaskDriverAvisobz->LOGIN",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
        }
        sleep(Duration::from_secs(4)).await;
        if user.cookies.is_empty() {
            let username = driver.find(By::Name("username")).await;
            if let Err(e) = username.as_ref() {
                Log::error(
                    "TaskDriverAvisobz->LOGIN",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
            }
            let _ = username.unwrap().send_keys(&user.email).await?;
            sleep(Duration::from_secs(2)).await;

            let password = driver.find(By::Name("password")).await;
            if let Err(e) = password.as_ref() {
                Log::error(
                    "TaskDriverAvisobz->LOGIN",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
            }

            let _ = password.unwrap().send_keys(&user.password).await?;
            sleep(Duration::from_secs(2)).await;

            let login = driver.find(By::Id("button-login")).await;
            if let Err(e) = login.as_ref() {
                Log::error(
                    "TaskDriverAvisobz->LOGIN",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
            }
            let _ = login.unwrap().click().await?;

            let user_block = driver
                .wait_element(By::Id("user-block-info-username"), 15)
                .await;
            if let Err(e) = user_block {
                Log::error(
                    "TaskDriverAvisobz->LOGIN",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
            }

            if let Ok(cookies) = driver.get_all_cookies().await {
                let mut cookie_format = String::new();
                for cookie in cookies {
                    if cookie.name.contains("googtrans") {
                        continue;
                    } else {
                        cookie_format
                            .push_str(format!("{}={}; ", cookie.name, cookie.value).as_str());
                    }
                }
                let _ = UserData::modify(&user.email, &user.password, &cookie_format, &user.proxy)
                    .await;
                return Ok(driver);
            }
        }
        for cookies in user.cookies.split("; ") {
            let cookie: Vec<&str> = cookies.split("=").collect();
            if cookie.len() <= 1 {
                continue;
            } else {
                let mut ck = Cookie::new(cookie[0], cookie[1]);
                ck.set_domain("aviso.bz");
                ck.set_same_site(SameSite::Lax);
                ck.set_path("/");
                if let Err(e) = driver.add_cookie(ck).await {
                    return Err(e);
                }
            }
        }
        if let Err(e) = driver.get("https://aviso.bz").await {
            let _ = driver.quit().await;
            return Err(e);
        };
        let user_block = driver
            .wait_element(By::Id("user-block-info-username"), 5)
            .await;
        if let Err(e) = user_block {
            let _ = driver.quit().await;
            let _ = UserData::modify(&user.email, &user.password, "", &user.proxy).await;
            return Err(e);
        }
        return Ok(driver);
    }

    async fn youtube(driver: WebDriver, task_number: &str) -> TaskResult {
        if let Err(e) = driver.goto("https://aviso.bz/work-youtube").await {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
        }

        let mut username = String::new();
        let mut classification = String::new();
        let mut money = String::new();

        let elem_username = driver.find(By::Id("user-block-info-username")).await;
        if let Err(e) = elem_username.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CRITICAL;
        }
        let _ = username.push_str(&elem_username.unwrap().text().await.unwrap());

        let elem_classification = driver.find(By::Id("reyt-user-block")).await;
        let _ = classification.push_str(&elem_classification.unwrap().text().await.unwrap());

        let elem_money = driver.find(By::Id("new-money-ballans")).await;
        let _ = money.push_str(
            &elem_money
                .unwrap()
                .text()
                .await
                .unwrap()
                .replace(" руб.", ""),
        );
        let print = Print {
            task: task_number.parse().unwrap(),
            username,
            classification,
            money: money.parse().unwrap(),
        };
        print.user().await;
        let _ = driver
            .screenshot(&Path::new(&format!(
                "config/avisobz/screenshot/youtube.png",
            )))
            .await;
        let work_serf = driver.find_all(By::ClassName("work-serf")).await;
        if let Err(e) = work_serf.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::PAUSE;
        }

        if work_serf.as_ref().unwrap().len() == 0 {
            return TaskResult::PAUSE;
        }

        let tds = work_serf.unwrap()[0].find_all(By::Tag("td")).await;
        if let Err(e) = tds.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }
        let tds = tds.unwrap();

        let ico_remove = tds[2].find(By::ClassName("ico-remove")).await;
        if let Err(e) = ico_remove.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        let divs = tds[1].find_all(By::Tag("div")).await;
        if let Err(e) = divs.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }
        let divs = divs.unwrap();

        let id_elem = divs[0].clone().attr("id").await;
        if let Err(e) = id_elem.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        let mut id = String::new();
        if let Some(txt) = id_elem.as_ref().unwrap() {
            if !txt.contains("start-ads") {
                return TaskResult::PAUSE;
            }
            let re = Regex::new(r"([0-9]+)").unwrap();
            let id_txt = re.captures(&txt).unwrap();
            id.push_str(&id_txt[1].to_string());
        }

        Log::debug(
            "TaskDriverAvisobz->YOUTUBE",
            &format!("line:{}\nid_element:{}", line!(), id),
        )
        .await;

        let spans = divs[0].find_all(By::Tag("span")).await;
        if let Err(e) = spans.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }
        let spans = spans.unwrap();

        let span = spans[0].clone();

        let window = driver.window().await;
        if let Err(e) = window.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        let tab_origin = window.unwrap();

        Log::debug(
            "TaskDriverAvisobz->YOUTUBE",
            &format!("line:{}\ntab_origin:{:?}", line!(), tab_origin),
        )
        .await;

        Log::debug(
            "TaskDriverAvisobz->YOUTUBE",
            &format!(
                "line:{}\nspan:{}",
                line!(),
                span.outer_html().await.unwrap()
            ),
        )
        .await;

        let onclick_span_elem = span.attr("onclick").await;
        if let Err(e) = onclick_span_elem.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        let mut onclick = String::new();
        if let Some(txt) = onclick_span_elem.unwrap() {
            onclick.push_str(&txt);
        }

        let re_fn_onclick =
            Regex::new(r"(funcjs\[\'start_youtube_new\'\]\([0-9]+, \'[0-9]+\'\);)").unwrap();
        let onclick_fn = re_fn_onclick.captures(&onclick).unwrap();

        Log::debug(
            "TaskDriverAvisobz->YOUTUBE",
            &format!("line:{}\nfn onclick:{}", line!(), &onclick_fn[1]),
        )
        .await;

        if let Err(e) = span.click().await {
            let _ = driver
                .screenshot(Path::new("config/avisobz/screenshot/span326.png"))
                .await;
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        //let _ = driver.execute(&onclick_fn[1], Vec::new()).await;

        let button_blue = driver
            .wait_element(By::ClassName("button_theme_blue"), 3)
            .await;
        if let Ok(e) = button_blue.as_ref() {
            let _ = e.click().await;
        }

        let windows = driver.windows().await;
        if let Err(e) = windows.as_ref() {
            let _ = driver
                .screenshot(Path::new(&format!(
                    "config/avisobz/screenshot/windows{}.png",
                    line!()
                )))
                .await;
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        let mut tabs = windows.unwrap();

        for i in 0..=10 {
            if tabs.len() > 1 {
                break;
            }

            let windows = driver.windows().await;
            if let Err(e) = windows.as_ref() {
                let _ = driver
                    .screenshot(Path::new(&format!(
                        "config/avisobz/screenshot/windows{}.png",
                        line!()
                    )))
                    .await;
                Log::error(
                    "TaskDriverAvisobz->YOUTUBE",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
                return TaskResult::CONTINUE;
            }
            tabs = windows.unwrap();

            Log::debug(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\ntabs_len:{}", line!(), tabs.len()),
            )
            .await;

            sleep(Duration::from_secs(1)).await;
            if i == 10 {
                return TaskResult::CONTINUE;
            }
        }
        let mut tab_video = WindowHandle::from("");
        for i in &tabs {
            if *i != tab_origin {
                tab_video = i.clone();
                let _ = driver.switch_to_window(tab_video.clone()).await;
            }
        }

        let mut newmode: bool = false;
        let yt_large = driver
            .wait_element(By::ClassName("ytp-large-play-button"), 5)
            .await;
        if let Err(e) = yt_large.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
        }

        if let Ok(elem) = yt_large.as_ref() {
            newmode = true;
            let _ = elem.click().await;
        }

        let tmr_id = match newmode {
            true => &format!("timer_ads_{}", id),
            false => "tmr",
        };

        Log::debug(
            "TaskDriverAvisobz->YOUTUBE",
            &format!("newmode:{:?}", newmode),
        )
        .await;

        //check old mode
        if !newmode {
            let iframe = driver.find(By::Tag("iframe")).await;
            if let Err(e) = iframe.as_ref() {
                Log::error(
                    "TaskDriverAvisobz->YOUTUBE",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
                let _ = driver.close_window().await;
                let _ = driver.switch_to_window(tab_origin.clone()).await;
                let _ = ico_remove.unwrap().click().await;
                return TaskResult::CONTINUE;
            }

            let _ = iframe.unwrap().enter_frame().await;
            let yt_large = driver
                .wait_element(By::ClassName("ytp-large-play-button"), 5)
                .await;
            if let Err(e) = yt_large.as_ref() {
                Log::error(
                    "TaskDriverAvisobz->YOUTUBE",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
            }
            if let Ok(res) = yt_large.as_ref().unwrap().is_displayed().await {
                if !res {
                    let _ = driver.close_window().await;
                    let _ = driver.switch_to_window(tab_origin.clone()).await;
                    let _ = ico_remove.unwrap().click().await;
                    return TaskResult::CONTINUE;
                }
                let _ = yt_large.unwrap().click().await;
                sleep(Duration::from_secs(3)).await;
            }
            let _ = driver.enter_default_frame().await;
        }

        if newmode {
            let _ = driver.switch_to_window(tab_origin.clone()).await;
        }

        let tmr_elem = driver.wait_element(By::Id(tmr_id), 5).await;
        if let Err(e) = tmr_elem.as_ref() {
            Log::error(
                "TaskDriverAvisobz->YOUTUBE",
                &format!("line:{}\n{}", line!(), e),
            )
            .await;
            return TaskResult::CONTINUE;
        }

        let mut tmr: i32 = 0;
        let tmr_txt = tmr_elem.unwrap().text().await;
        if let Ok(txt) = tmr_txt {
            if txt.is_empty() {
                tmr = "0".parse().unwrap();
            } else {
                tmr = txt.parse().unwrap();
            }
        }

        let mut tmr_old = 0;
        let mut tmr_error = 0;

        while tmr > 0 {
            print.tmr("YOUTUBE", &tmr.to_string()).await;

            if tmr != tmr_old {
                tmr_old = tmr;
                tmr_error = 0;
            }
            if tmr == tmr_old {
                tmr_error += 1;
                if tmr_error == 2000 {
                    let _ = driver.close_window().await;
                    let _ = driver.switch_to_window(tab_origin).await;
                    let _ = ico_remove.unwrap().click().await;
                    return TaskResult::CONTINUE;
                }
            }

            let tmr_txt = match driver.find(By::Id(tmr_id)).await {
                Ok(tmr_elem) => match tmr_elem.text().await {
                    Ok(txt) => {
                        if txt.is_empty() {
                            "0".to_string()
                        } else {
                            txt
                        }
                    }
                    Err(_) => "0".to_string(),
                },
                Err(_) => "0".to_string(),
            };
            tmr = tmr_txt.parse().unwrap();
            sleep(Duration::from_millis(50)).await;
        }

        if newmode {
            // switch to window video for close window
            let _ = driver.switch_to_window(tab_video.clone()).await;
            let _ = driver.close_window().await;

            // back window origin for confirm task
            let _ = driver.switch_to_window(tab_origin.clone()).await;

            let _ = match driver
                .wait_element(By::Id(format!("ads_btn_confirm_{}", id)), 5)
                .await
            {
                Ok(btn) => btn.click().await,
                Err(e) => {
                    Log::error(
                        "TaskDriverAvisobz->YOUTUBE",
                        &format!("line:{}\n{}", line!(), e),
                    )
                    .await;
                    return TaskResult::CONTINUE;
                }
            };
        }

        //old mode
        if !newmode {
            let _ = driver.switch_to_window(tab_origin.clone()).await;
        }

        let earn = match divs[0].wait_element(By::Tag("b"), 5).await {
            Ok(b) => {
                let src_b = match b.outer_html().await {
                    Ok(txt) => txt,
                    Err(e) => {
                        Log::error(
                            "TaskDriverAvisobz->YOUTUBE",
                            &format!("line:{}\n{}", line!(), e),
                        )
                        .await;
                        return TaskResult::CONTINUE;
                    }
                };

                Log::debug(
                    "TaskDriverAvisobz->YOUTUBE",
                    &format!("line:{}\nearn:{}", line!(), src_b),
                )
                .await;
                b.text().await
            }
            Err(e) => {
                Log::error(
                    "TaskDriverAvisobz->YOUTUBE",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
                return TaskResult::CONTINUE;
            }
        };

        if let Ok(_) = earn {
            if !newmode {
                let _ = driver.switch_to_window(tab_video.clone()).await;
                let _ = driver.close_window().await;
                let _ = driver.switch_to_window(tab_origin.clone()).await;
            }
        }

        let earn = earn.unwrap();
        print.earn(&earn).await;
        return TaskResult::OK;
    }
}

struct TaskControlAvisobz;

impl TaskControlAvisobz {
    async fn youtube(headless: bool) -> () {
        let _ = spawn(async move {
            let mut cmdriver = start_driver().await;
            loop {
                if GLOBAL_CONTROL.load(Ordering::Relaxed) {
                    let _ = cmdriver.kill().unwrap();
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }
        });

        sleep(Duration::from_millis(500)).await;

        let user_task = TaskDriverAvisobz { headless };
        let mut task_number = 1;
        loop {
            if GLOBAL_CONTROL.load(Ordering::Relaxed) {
                return ();
            }
            let result_user_task = user_task.clone().login().await;
            if let Err(e) = result_user_task {
                Log::error(
                    "TaskControlSeofast->YOUTUBE",
                    &format!("line:{}\n{}", line!(), e),
                )
                .await;
                continue;
            }

            let driver = result_user_task.unwrap();
            loop {
                if GLOBAL_CONTROL.load(Ordering::Relaxed) {
                    return ();
                }
                let youtube =
                    TaskDriverAvisobz::youtube(driver.clone(), &task_number.to_string()).await;
                match youtube {
                    TaskResult::CONTINUE => continue,
                    TaskResult::PAUSE => {
                        let _ = driver.clone().quit().await;
                        Print::pause().await;
                        break;
                    }
                    TaskResult::OK => {
                        task_number += 1;
                        continue;
                    }
                    TaskResult::CRITICAL => {
                        let _ = driver.clone().quit().await;
                        break;
                    }
                    TaskResult::QUIT => break,
                }
            }
        }
    }

    #[allow(dead_code)]
    async fn surfing() {
        todo!()
    }

    #[allow(dead_code)]
    async fn all() {
        todo!()
    }
}

pub struct ThreadTaskAvisobz {
    pub headless: bool,
}

impl ThreadTaskAvisobz {
    pub async fn youtube(self) {
        spawn(async move {
            if let Ok(_) = ctrl_c().await {
                GLOBAL_CONTROL.store(true, Ordering::SeqCst)
            }
        });
        let th = spawn(async move { TaskControlAvisobz::youtube(self.headless).await });

        loop {
            if th.is_finished() {
                exit(0);
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn surfing(self) {
        todo!()
    }

    pub async fn all(self) {
        todo!()
    }
}
