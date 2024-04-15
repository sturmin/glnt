use std::process::ExitCode;
use std::time::{Duration, SystemTime};
use nokhwa::{Camera, native_api_backend, query};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{RequestedFormat, RequestedFormatType};


fn main() -> ExitCode {
    let granted = request_access();

    if !granted {
        ExitCode::FAILURE
    } else {
        println!("Access: {granted}");

        let backend = native_api_backend().unwrap();
        let devices = query(backend).unwrap();
        println!("There are {} available cameras.", devices.len());
        for device in &devices {
            println!("{device}");
        }

        let h = devices.first().unwrap();

        let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::with_backend(h.index().clone(), format, backend).unwrap();

        let now  = SystemTime::now();

        println!("{}", now.elapsed().unwrap().as_millis());
        let _ = camera.open_stream().and_then (|_| {
            println!("{}", now.elapsed().unwrap().as_millis());
           std::thread::sleep(Duration::from_millis(10000));
           let r = camera.stop_stream();
           println!("{}", now.elapsed().unwrap().as_millis());
           r
        });

        ExitCode::SUCCESS
    }
}

// fn devices() {
//     let backend = native_api_backend().expect("");
//     //query(backend).ma
//
//
//
//
// }

fn request_access() -> bool {
    use std::sync::mpsc::channel;
    use nokhwa::nokhwa_initialize;
    let (tx, rx) = channel();

    nokhwa_initialize(move |granted| {
        tx.send(granted).unwrap()
    });

    rx.recv().unwrap()
}