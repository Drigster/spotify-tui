use super::super::app::{ActiveBlock, App};
use super::common_key_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.active_block = ActiveBlock::MyPlaylists;
        }
        k if common_key_events::down_event(k) => {
            match &app.devices {
                Some(p) => {
                    if let Some(selected_device_index) = app.selected_device_index {
                        let next_index = common_key_events::on_down_press_handler(
                            &p.devices,
                            Some(selected_device_index),
                        );
                        app.selected_device_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.devices {
                Some(p) => {
                    if let Some(selected_device_index) = app.selected_device_index {
                        let next_index = common_key_events::on_up_press_handler(
                            &p.devices,
                            Some(selected_device_index),
                        );
                        app.selected_device_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        Key::Char('\n') => {
            if let (Some(devices), Some(index)) = (&app.devices, app.selected_device_index) {
                if let Some(device) = &devices.devices.get(index) {
                    app.device_id = Some(device.id.clone());
                    app.active_block = ActiveBlock::MyPlaylists;
                    match app.set_cached_device_token(device.id.clone()) {
                        Ok(()) => {}
                        Err(e) => {
                            app.active_block = ActiveBlock::Error;
                            app.api_error = e.to_string();
                        }
                    };
                }
            };
        }
        _ => {}
    }
}