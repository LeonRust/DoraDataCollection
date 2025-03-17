use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

use common::{config, state::UsbType};

pub fn find_usb_driver(usb_type: UsbType) -> Vec<String> {
    let (_, key_word) = match usb_type {
        UsbType::U2D2 => config::USB_U2D2,
        UsbType::Orbbec => config::USB_ORBBEC,
    };

    Command::new("ls")
        .arg("/dev")
        .output()
        .ok()
        .map(|output| {
            let data = String::from_utf8_lossy(&output.stdout);
            data.trim()
                .split("\n")
                .filter(|s| s.starts_with(key_word))
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default()
}

pub fn find_usb_number(usb_type: UsbType, usb_serials: Vec<String>) -> HashMap<String, String> {
    let mut serials = HashMap::new();

    for usb in usb_serials.into_iter() {
        if let Ok(output) = Command::new("udevadm")
            .arg("info")
            .arg("--attribute-walk")
            .arg(format!("--name=/dev/{}", usb))
            .stdout(Stdio::piped())
            .spawn()
        {
            if let Some(output) = output.stdout {
                if let Ok(grep_output) = Command::new("grep")
                        .arg("-E")
                        .arg("looking at device|ATTRS{idVendor}|ATTRS{idProduct}|ATTR{index}|ATTR{name}|ATTRS{serial}")
                        .stdin(Stdio::from(output))
                        .output() {
                            let data = String::from_utf8_lossy(&grep_output.stdout);
                            let data = data.trim().split("\n")
                                .map(|s| s.trim().replace("\"", ""))
                                .collect::<Vec<_>>();

                            match usb_type {
                                UsbType::U2D2 => {
                                    if data.len() >= 4 {
                                        let serial = data[3].replace("ATTRS{serial}==", "");
                                        if data[0].starts_with("looking at device") {
                                            serials.insert(
                                                serial,
                                                data[0]
                                                    .split("/")
                                                    .last()
                                                    .unwrap()
                                                    .replace("':", "")
                                                    .to_string(),
                                            );
                                        }
                                    }
                                },
                                UsbType::Orbbec => {
                                    if data.len() >= 6 {
                                        let serial = data[5].replace("ATTRS{serial}==", "");
                                        if data[0].starts_with("looking at device")
                                            && data[0].contains(":1.4/video4linux")
                                            && data[1] == "ATTR{index}==0"
                                        {
                                            serials.insert(
                                                serial,
                                                data[0]
                                                    .split("/")
                                                    .last()
                                                    .unwrap()
                                                    .replace("':", "")
                                                    .to_string(),
                                            );
                                        }
                                    }
                                },
                            }
                        }
            }
        }
    }

    serials
}
