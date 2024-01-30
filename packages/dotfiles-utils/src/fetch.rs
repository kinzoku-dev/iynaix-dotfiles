use crate::{
    asset_path,
    cli::WaifuFetchArgs,
    cmd_output, full_path, json,
    nixinfo::NixInfo,
    wallpaper::{self, WallInfo},
    CmdOutput,
};
use chrono::{DateTime, Datelike, NaiveDate, Timelike};
use serde_json::{json, Value};
use std::process::Command;

#[cfg(feature = "wfetch-waifu")]
pub const fn arg_waifu(args: &WaifuFetchArgs) -> bool {
    args.waifu
}

#[cfg(not(feature = "wfetch-waifu"))]
pub const fn arg_waifu(_args: &WaifuFetchArgs) -> bool {
    false
}

#[cfg(feature = "wfetch-wallpaper")]
pub const fn arg_wallpaper(args: &WaifuFetchArgs) -> bool {
    args.wallpaper
}

#[cfg(not(feature = "wfetch-wallpaper"))]
pub const fn arg_wallpaper(_args: &WaifuFetchArgs) -> bool {
    false
}

#[cfg(any(feature = "wfetch-waifu", feature = "wfetch-wallpaper"))]
pub const fn arg_exit(args: &WaifuFetchArgs) -> bool {
    args.exit
}

#[cfg(not(any(feature = "wfetch-waifu", feature = "wfetch-wallpaper")))]
pub const fn arg_exit(_args: &WaifuFetchArgs) -> bool {
    false
}

#[cfg(any(feature = "wfetch-waifu", feature = "wfetch-wallpaper"))]
pub const fn arg_image_size(args: &WaifuFetchArgs) -> Option<i32> {
    args.size
}

#[cfg(not(any(feature = "wfetch-waifu", feature = "wfetch-wallpaper")))]
pub const fn arg_image_size(_args: &WaifuFetchArgs) -> Option<i32> {
    None
}

fn create_output_image(filename: String) -> String {
    let output_dir = full_path("~/.cache/wfetch");
    std::fs::create_dir_all(&output_dir).expect("failed to create output dir");

    output_dir
        .join(filename)
        .to_str()
        .expect("could not convert output dir to str")
        .to_string()
}

fn create_nixos_logo(nix_info: &NixInfo, args: &WaifuFetchArgs) -> String {
    let logo = asset_path("nixos.png");
    let logo = logo.as_str();
    let hexless = &nix_info.colors;
    let c1 = hexless.get("color4").expect("invalid color");
    let c2 = hexless.get("color6").expect("invalid color");

    let output = create_output_image(format!("{c1}-{c2}.png"));
    let image_size = arg_image_size(args).unwrap_or(if args.challenge { 420 } else { 340 });

    Command::new("convert")
        .args([
            logo, "-fuzz", "10%", "-fill", c1, "-opaque", "#5278c3", // replace color 1
            "-fuzz", "10%", "-fill", c2, "-opaque", "#7fbae4", // replace color 2
        ])
        .args(["-resize", format!("{image_size}x{image_size}").as_str()])
        .arg(&output)
        .status()
        .expect("failed to execute imagemagick");

    output
}

fn create_wallpaper_crop(args: &WaifuFetchArgs) -> String {
    // read current wallpaper
    let wall = std::fs::read_to_string(full_path("~/.cache/current_wallpaper"))
        .expect("could not read current wallpaper");
    let wallpaper_info = wallpaper::info(&wall);

    let crop_area = if let Some(WallInfo {
        r1x1: crop_area, ..
    }) = &wallpaper_info
    {
        crop_area.to_owned()
    } else {
        let (width, height) =
            image::image_dimensions(&wall).expect("could not get image dimensions");

        // get square crop for imagemagick
        if width > height {
            format!("{height}x{height}+{}+0", (width - height) / 2)
        } else {
            format!("{width}x{width}+0+{}", (height - width) / 2)
        }
    };

    let image_size = arg_image_size(args).unwrap_or(if args.challenge { 380 } else { 300 });
    let output = create_output_image("wallpaper.png".to_string());

    // use imagemagick to crop and resize the wallpaper
    Command::new("convert")
        .arg(wall)
        .args(["-crop", &crop_area])
        .args(["-resize", format!("{image_size}x{image_size}").as_str()])
        .arg(&output)
        .status()
        .expect("failed to execute imagemagick");

    output
}

pub fn shell_module() -> serde_json::Value {
    // HACK: fastfetch detects the process as wfetch, detect it via STARSHIP_SHELL
    if std::env::var("STARSHIP_SHELL").unwrap_or_default() == "fish" {
        let fish_version = cmd_output(["fish", "--version"], &CmdOutput::Stdout)
            .first()
            .expect("could not run fish")
            .split(' ')
            .last()
            .expect("could not parse fish version")
            .to_string();

        json!({
            "type": "command",
            "key": "󰈺 SH",
            "text": format!("echo \"fish {}\"", fish_version),
        })
    } else {
        json!({ "type": "shell", "key": " SH", "format": "{3}" })
    }
}

#[allow(clippy::similar_names)] // gpu and cpu trips this
pub fn create_fastfetch_config(args: &WaifuFetchArgs, nix_info: &NixInfo, config_jsonc: &str) {
    let os = json!({ "type": "os", "key": " OS", "format": "{3}" });
    let kernel = json!({ "type": "kernel", "key": " VER", });
    let uptime = json!({ "type": "uptime", "key": "󰅐 UP", });
    let packages = json!({ "type": "packages", "key": "󰏖 PKG", });
    let display = json!({ "type": "display", "key": "󰍹 RES", "compactType": "scaled" });
    let wm = json!({ "type": "wm", "key": " WM", "format": "{2}" });
    let terminal = json!({ "type": "terminal", "key": " TER", "format": "{3}" });
    let cpu = json!({ "type": "cpu", "key": " CPU", "format": "{1} ({5})", });
    let gpu = json!({ "type": "gpu", "key": " GPU", "driverSpecific": true, "format": "{2}", "forceVulkan": true, "hideType": "integrated" });
    let memory =
        json!({ "type": "memory", "key": "󰆼 RAM", "format": "{/1}{-}{/}{/2}{-}{/}{} / {}" });
    let color = json!({ "type": "colors", "symbol": "circle", });

    // handle logo
    let mut logo = json!({ "source": "nixos" });

    if args.hollow {
        let hollow = asset_path("nixos_hollow.txt");
        logo = json!({
            "source": hollow,
            "color": {
                "1": "blue",
                "2": "cyan",
            }
        });
    } else if arg_wallpaper(args) {
        logo = json!({
            // ghostty supports kitty image protocol
            "type": "kitty-direct",
            "source": create_wallpaper_crop(args),
            "preserveAspectRatio": true,
        });
    } else if arg_waifu(args) {
        logo = json!({
            // ghostty supports kitty image protocol
            "type": "kitty-direct",
            "source": create_nixos_logo(nix_info, args),
            "preserveAspectRatio": true,
        });
    }

    let mut modules = vec![
        os,
        kernel,
        uptime,
        packages,
        json!("break"),
        cpu,
        gpu,
        memory,
        json!("break"),
        display,
        wm,
        terminal,
        shell_module(),
    ];

    // set colors for modules
    if !args.no_color_keys {
        let colors = ["green", "yellow", "blue", "magenta", "cyan"];
        for (i, module) in modules.iter_mut().enumerate() {
            if let Value::Object(module) = module {
                module.insert("keyColor".into(), json!(colors[i % colors.len()]));
            }
        }
    }

    // optional challenge block
    if args.challenge {
        modules.extend_from_slice(&challenge_block(args));
    }

    modules.extend_from_slice(&[json!("break"), color]);

    let contents = json!( {
        "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
        "display": {
            "separator": "   ",
            // icon + space + 3 letters + separator
            "keyWidth": 1 + 1 + 3 + 3,
            "binaryPrefix": "si",
        },
        "logo": logo,
        "modules": modules,
    });

    // write json to file
    json::write(config_jsonc, contents);
}

fn term_color(color: i32, text: &String, bold: bool) -> String {
    let bold_format = if bold { "1;" } else { "" };
    format!("\u{1b}[{bold_format}{}m{text}\u{1b}[0m", 30 + color)
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    let month = if month == 12 { 1 } else { month };
    let year = if month == 12 { year + 1 } else { year };

    let first_day_of_next_month = NaiveDate::from_ymd_opt(year, month + 1, 1).expect("");
    (first_day_of_next_month - chrono::Duration::days(1)).day()
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_wrap)]
pub fn challenge_text(args: &WaifuFetchArgs) -> String {
    let start = DateTime::parse_from_str(args.challenge_timestamp.to_string().as_str(), "%s")
        .expect("could not parse start timestamp");

    let mths = args.challenge_months % 12;
    let yrs = args.challenge_years + args.challenge_months / 12;

    let final_mth = if start.month() + mths > 12 {
        start.month() + mths - 12
    } else {
        start.month() + mths
    };
    let final_yr = if start.month() + mths > 12 {
        start.year() + yrs as i32 + 1
    } else {
        start.year() + yrs as i32
    };
    let final_day = std::cmp::min(start.day(), last_day_of_month(final_yr, final_mth));

    let end = NaiveDate::from_ymd_opt(final_yr, final_mth, final_day)
        .expect("invalid end date")
        .and_time(
            chrono::NaiveTime::from_hms_opt(start.hour(), start.minute(), start.second())
                .expect("invalid end time"),
        );

    let now = chrono::offset::Local::now();

    let elapsed = now.timestamp() - start.timestamp();
    let total = end.timestamp() - start.timestamp();

    let percent = elapsed as f32 / total as f32 * 100.0;

    let elapsed_days = elapsed / 60 / 60 / 24;
    let total_days = total / 60 / 60 / 24;

    format!("{elapsed_days} Days / {total_days} Days ({percent:.2}%)")
}

pub fn challenge_block(args: &WaifuFetchArgs) -> Vec<serde_json::Value> {
    let body = challenge_text(args);
    let maxlen = body.len();

    let title = "  10 YEAR CHALLENGE  ";

    let title = json!({
        "type": "custom",
        "format": term_color(3, &format!("{title:^maxlen$}"), true),
    });
    let sep = json!({
        "type": "custom",
        // fill line with box drawing dash
        "format": term_color(3, &format!("{:─^maxlen$}", ""), false),
    });
    let body = json!({
        "type": "custom",
        "format": body,
    });

    vec![json!("break"), title, sep, body]
}
