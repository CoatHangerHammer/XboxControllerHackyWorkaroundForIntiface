use evdev::{
    Device, FFEffect, FFEffectData, FFEffectKind, FFReplay, FFTrigger
};

use crate::utils::exp_map;

// Expands low rumble through an exponential graph
// Returns 0-255 from the fifo and most of that is rumble that doesn't really change
// This means linear values it goes low - high very quickly
// The value of 3 works in my case
//
// Convert u8 (0-255) to u16 (0-65335)
// This will only ever go up to 51000, that is because I found my controller stops rumbling at 52122
// Idk why, it is 512 * 100 + extra, which seems it might be that thats the spec?
pub fn fifo_to_magnitude(fifo_int: u16) -> u16 {
    let fifo_int = exp_map(fifo_int, 3.0);
    fifo_int * 50 * 4
}

pub fn build_effect(magnitude: u16) -> FFEffectData {
    FFEffectData {
        direction: 0,
        trigger: FFTrigger::default(),
        replay: FFReplay {
            length: 100, // 100 ms, will become infinite due to play_effect, maxes at 65s 
            delay: 0
        },
        kind: FFEffectKind::Rumble {
             strong_magnitude: magnitude, 
             weak_magnitude: magnitude 
        },
    }
}


pub fn create_effect(device: &mut Device, fifo_data: u16) -> Result<FFEffect, std::io::Error>{
    let magnitude = fifo_to_magnitude(fifo_data);
    let effect_data = build_effect(magnitude);
    device.upload_ff_effect(effect_data)
}

pub fn play_effect(effect: &mut FFEffect) -> std::io::Result<()> {
    effect.play(i32::MAX)?; // This is set to essentially infinite because we rely on updates to override. Not a safe value tho
    Ok(())
}

// This is here because it can be used but overwriting the effect has the same function
pub fn _stop_effect(effect: &mut FFEffect) -> std::io::Result<()> {
    effect.stop()?; 
    Ok(())
}