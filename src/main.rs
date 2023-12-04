
use std::process;
use core::time;
use sysinfo::{CpuExt, System, SystemExt};
use tray_icon::{
    menu::Menu,
    TrayIconBuilder, TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png");
    let icon = load_tray_icon(std::path::Path::new(path)); 
    let event_loop = EventLoopBuilder::new().build().unwrap();

    let mut sys = System::new();

    #[cfg(not(target_os = "linux"))]
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(Menu::new()))
        .with_tooltip("CPU Usage")
        .with_icon(icon)
        .build()
        .unwrap();
    let tray_channel = TrayIconEvent::receiver();

    match event_loop.run(move |_event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Poll);

        sys.refresh_cpu(); // Refreshing CPU information.

        let cpu_usages: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        let cpu_usages_string = cpu_usages
            .iter()
            .enumerate()
            .map(|(cpu_idx, cpu_usage)| format!("CPU {}: {:.2}%", cpu_idx, cpu_usage))
            .collect::<Vec<String>>()
            .join("\n");
    
        let average_cpu_usage: f32 = cpu_usages.iter().sum::<f32>() / cpu_usages.len() as f32;
        
        let final_tooltip = format!("Average: {:.2}%", average_cpu_usage);

        println!("{}\n\nAverage: {}", cpu_usages_string, average_cpu_usage);

        match tray_icon.set_tooltip(Some(final_tooltip.as_str())) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error setting tooltip: {}", err);
            }
        }
    
        match tray_channel.try_recv() {
            // ToDo: Display all metrics in a small window when the user clicks the icon
            Ok(_) => {
                println!("App exit by user");
                process::exit(0);
            }
            Err(err) => {
                eprintln!("Error receiving tray channel message: {}", err);
            }
        }

        std::thread::sleep(time::Duration::from_secs(1));
    }) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error running eventloop: {}", err);
        }
    };
}

fn load_tray_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (rgba, width, height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (icon_width, icon_height) = image.dimensions();
        let icon_rgba = image.into_raw();
        (icon_rgba, icon_width, icon_height)
    };
    tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to open icon")
}



