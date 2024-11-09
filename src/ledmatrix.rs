use std::io::Error;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::thread::sleep;
use std::time::Duration;

// Heart symbol pattern for display
const HEART: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x22, 0x42, 0x84, 0x42, 0x22, 0x1C, 0x00, 0x00, 0x00, 0x00];

struct Frame {
    frames: u32,
    data: [u32; 16],
}

fn set_line_value(line: &LineHandle, value: u8) {
    line.set_value(value).expect("Failed to set line value");
}

fn sda(sda_handle: &LineHandle, status: u8) {
    sda_handle.set_value(status).unwrap()
}

fn scl(scl_handle: &LineHandle, status: u8) {
    scl_handle.set_value(status).unwrap()
}

fn iic_start(sda_handle: &LineHandle, scl_handle: &LineHandle) {
    scl(&scl_handle, 0);
    sleep(Duration::from_micros(3));
    sda(&sda_handle, 1);
    sleep(Duration::from_micros(3));
    scl(&scl_handle, 1);
    sleep(Duration::from_micros(3));
    sda(&sda_handle, 0);
    sleep(Duration::from_micros(3));
}

fn iic_send(sda_handle: &LineHandle, scl_handle: &LineHandle, mut send_data: u32) {
    for _ in 0..8 {
        scl(&scl_handle, 0);
        sleep(Duration::from_micros(3));
        sda(&sda_handle, (send_data & 0x01) as u8);
        sleep(Duration::from_micros(3));
        scl(&scl_handle, 1);
        sleep(Duration::from_micros(3));
        send_data >>= 1;
    }
}

fn iic_end(sda_handle: &LineHandle, scl_handle: &LineHandle) {
    scl(&scl_handle, 0);
    sleep(Duration::from_micros(3));
    sda(&sda_handle, 0);
    sleep(Duration::from_micros(3));
    scl(&scl_handle, 1);
    sleep(Duration::from_micros(3));
    sda(&sda_handle, 1);
    sleep(Duration::from_micros(3));
}

fn setup_lines(sda_pin: u32, scl_pin: u32) -> Result<(LineHandle, LineHandle), Error> {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let sda_handle = chip.get_line(sda_pin).unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "sda").unwrap();
    let scl_handle = chip.get_line(scl_pin).unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "scl").unwrap();
    sda_handle.set_value(0).unwrap();
    scl_handle.set_value(1).unwrap();

    sda(&sda_handle, 0);
    scl(&scl_handle, 0);

    Ok((sda_handle, scl_handle))
}

fn show_frame(sda_handle: &LineHandle, scl_handle: &LineHandle, frame: &[u32; 16]) {
    iic_start(sda_handle, scl_handle);
    iic_send(sda_handle, scl_handle, 0x40);
    iic_end(sda_handle, scl_handle);

    iic_start(sda_handle, scl_handle);
    iic_send(sda_handle, scl_handle, 0xc0);
    for &data in frame.iter() {
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
        let data = frame[1..17].try_into().unwrap();
        frames.push(Frame {
            frames: frame[0],
            data,
        })
    }
    frames
}

pub fn test() -> Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let sda_handle = chip.get_line(17)?
        .request(LineRequestFlags::OUTPUT, 0, "sda")?;
    let scl_handle = chip.get_line(27)?
        .request(LineRequestFlags::OUTPUT, 0, "scl")?;
    sda_handle.set_value(0)?;
    scl_handle.set_value(1)?;
    Ok(())
}

pub fn init() -> Result<(LineHandle, LineHandle), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let sda_handle = chip.get_line(17)?
        .request(LineRequestFlags::OUTPUT, 0, "sda")?;
    let scl_handle = chip.get_line(27)?
        .request(LineRequestFlags::OUTPUT, 0, "scl")?;
    Ok((sda_handle, scl_handle))
}

pub fn show(sda_handle: &LineHandle, scl_handle: &LineHandle, args: Vec<[u32; 17]>) {
    let frames_data = get_frames(args);

    for frame in &frames_data {
        println!("Showing frame");
        show_frame(&sda_handle, &scl_handle, &frame.data);
        let delay_ms = (frame.frames as f64 / 60.0 * 1_000.0) as u64;
        sleep(Duration::from_millis(delay_ms));
    }

    let empty_frame = [0u32; 16];
    if let Some(delay) = frames_data.last().map(|frame| frame.frames) {
        println!("Showing empty frame");
        show_frame(&sda_handle, &scl_handle, &empty_frame);
        sleep(Duration::from_millis(delay as u64));
    }
}
