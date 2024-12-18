#![windows_subsystem = "windows"]
use freya::prelude::*;
use std::{
    fs,
    io::ErrorKind,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn read_file() -> i8 {
    match fs::read_to_string("time_zone.txt") {
        Ok(file) => file.parse().unwrap(),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match fs::write("time_zone.txt", "0") {
                Ok(()) => 0,
                Err(_error) => panic!("Failed to create file {:?}", _error),
            },
            other_error => panic!("{}", other_error),
        },
    }
}

fn add_zero(time: i64) -> String {
    let time = time.to_string();
    if time.len() == 1 {
        let mut zero = "0".to_owned();
        zero.push_str(&time);
        zero
    } else {
        time
    }
}

fn negative_add_zero(time: i64) -> String {
    if time < 0 {
        let number = add_zero(-time);
        let mut minus = "-".to_owned();
        minus.push_str(&number);
        minus
    } else {
        add_zero(time)
    }
}

fn format_time(time: &SystemTime, time_zone: i8) -> String {
    let current_time =
        time_zone as i64 * 3600 + time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    add_zero(current_time.rem_euclid(86400) / 3600)
        + ":"
        + &add_zero(current_time.rem_euclid(3600) / 60)
        + ":"
        + &add_zero(current_time.rem_euclid(60))
}

fn app() -> Element {
    let mut system_time = use_signal(SystemTime::now);
    use_effect(move || {
        spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(500));
            loop {
                interval.tick().await;
                system_time.set(SystemTime::now());
            }
        });
    });

    let mut time_zone = use_signal(read_file);

    use_effect(move || {
        fs::write("time_zone.txt", time_zone.read().to_string()).expect("failed to write file")
    });

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            rect {
                position: "absolute",
                position_left: "0",
                position_top: "0",
                Dropdown {
                    value: time_zone(),
                    ScrollView {
                        theme: theme_with!(ScrollViewTheme {
                            width: "200".into(),
                            height: "300".into(),
                        }),
                        for i in -12..=14 {
                            DropdownItem {
                                value: i,
                                onclick: move |_| time_zone.set(i),
                                label {"UTC {negative_add_zero(i as i64)}:00"}
                            }
                        }
                    }
                }
            }
            rect {
                corner_radius: "10",
                main_align: "center",
                background: "blue",
                min_width: "500",
                width: "90%",
                height: "200",
                label {
                    font_family: "Consolas",
                    text_align: "center",
                    font_size: "100",
                    "{format_time(&system_time.read(), *time_zone.read())}"
                }
            }
        }
    )
}

const WIDTH: f64 = 500.0;
const HEIGHT: f64 = 400.0;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(WIDTH)
            .with_height(HEIGHT)
            .with_min_width(WIDTH)
            .with_min_height(HEIGHT)
            .with_title("Clock")
            .build(),
    );
}
