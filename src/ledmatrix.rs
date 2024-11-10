use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::thread::sleep;
use std::time::Duration;
use alfred_rs::log::debug;

struct Frame {
    frames: u32,
    data: [u32; 16],
}

fn sda(sda_handle: &LineHandle, status: u8) {
    sda_handle.set_value(status).expect("sda set_value failed");
}

fn scl(scl_handle: &LineHandle, status: u8) {
    scl_handle.set_value(status).expect("scl set_value failed");
}

fn iic_start(sda_handle: &LineHandle, scl_handle: &LineHandle) {
    scl(scl_handle, 0);
    sleep(Duration::from_micros(3));
    sda(sda_handle, 1);
    sleep(Duration::from_micros(3));
    scl(scl_handle, 1);
    sleep(Duration::from_micros(3));
    sda(sda_handle, 0);
    sleep(Duration::from_micros(3));
}

fn iic_send(sda_handle: &LineHandle, scl_handle: &LineHandle, mut send_data: u32) {
    for _ in 0..8 {
        scl(scl_handle, 0);
        sleep(Duration::from_micros(3));
        sda(sda_handle, (send_data & 0x01) as u8);
        sleep(Duration::from_micros(3));
        scl(scl_handle, 1);
        sleep(Duration::from_micros(3));
        send_data >>= 1;
    }
}

fn iic_end(sda_handle: &LineHandle, scl_handle: &LineHandle) {
    scl(scl_handle, 0);
    sleep(Duration::from_micros(3));
    sda(sda_handle, 0);
    sleep(Duration::from_micros(3));
    scl(scl_handle, 1);
    sleep(Duration::from_micros(3));
    sda(sda_handle, 1);
    sleep(Duration::from_micros(3));
}

fn show_frame(sda_handle: &LineHandle, scl_handle: &LineHandle, frame: &[u32; 16]) {
    iic_start(sda_handle, scl_handle);
    iic_send(sda_handle, scl_handle, 0x40);
    iic_end(sda_handle, scl_handle);

    iic_start(sda_handle, scl_handle);
    iic_send(sda_handle, scl_handle, 0xc0);
    for &data in frame {
        iic_send(sda_handle, scl_handle, data);
    }
    iic_end(sda_handle, scl_handle);

    iic_start(sda_handle, scl_handle);
    iic_send(sda_handle, scl_handle, 0x8A);
    iic_end(sda_handle, scl_handle);
}

fn get_frames(args: Vec<[u32; 17]>) -> Vec<Frame> {
    let mut frames = Vec::new();
    for frame in args {
        let data = frame[1..17].try_into().expect("frame too big");
        frames.push(Frame {
            frames: frame[0],
            data,
        });
    }
    frames
}

pub fn init(sda_pin: u32, scl_pin: u32) -> Result<(LineHandle, LineHandle), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let sda_handle = chip.get_line(sda_pin)?
        .request(LineRequestFlags::OUTPUT, 0, "sda")?;
    let scl_handle = chip.get_line(scl_pin)?
        .request(LineRequestFlags::OUTPUT, 0, "scl")?;
    Ok((sda_handle, scl_handle))
}

#[allow(clippy::integer_division, clippy::cast_lossless)]
pub fn show(sda_handle: &LineHandle, scl_handle: &LineHandle, args: Vec<[u32; 17]>) {
    let frames_data = get_frames(args);

    for frame in &frames_data {
        show_frame(sda_handle, scl_handle, &frame.data);
        sleep(Duration::from_millis((frame.frames / 60 * 1_000) as u64));
    }

    let empty_frame = [0u32; 16];
    if let Some(delay) = frames_data.last().map(|frame| frame.frames) {
        if delay == 0 { return }
        debug!("showing empty frame");
        show_frame(sda_handle, scl_handle, &empty_frame);
        sleep(Duration::from_millis(u64::from(delay)));
    }
}
