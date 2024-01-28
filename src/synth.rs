use crate::kinput::*;
use crate::kmath::*;
use crate::krenderer::*;

#[derive(Clone, Copy)]
pub struct Sound {
    pub freq: f32,
    pub A: f32,
    pub S: f32,
    pub R: f32,

    pub fmmod_carrier_ratio: f32,
    pub fmmod_amt: f32,

    pub fmod_carrier_ratio: f32,
    pub fmod_amt: f32,

    pub amplitude: f32,
    pub amp_lfo_freq: f32,
    pub amp_lfo_amount: f32,

    pub duration: f32,
}

// fm unit gets rate, amplitude, phase
// fm units connected however
// imagine if it was just array of floats, and then array of descriptions, wow so declarative
// to do sounds for gball, well kinda need to fix that mixer thing
// some windowing probably would be good too

// oh nb this fm is parallel actually not series
// how about put feedback back in

// what if frequency gets plus or minusd instead of timsed

// also allow negative frequencies

// maybe pling

// probably could make envelope exponential decay instead of linear
// and have envelope as a per thing basis

impl Sound {
    pub fn new() -> Sound {
        Sound {
            freq: 440.0,
            A: 1.0,
            S: 1.0,
            R: 1.0,
            fmmod_carrier_ratio: 0.25,
            fmmod_amt: 0.0,
            fmod_carrier_ratio: 0.25,
            fmod_amt: 0.0,
            amplitude: 0.1,
            amp_lfo_freq: 20.0,
            amp_lfo_amount: 0.0, 
            duration: 1.0,
        }
    }
}

pub struct Synth {
    pub sound: Sound,
    pub any_change: bool,
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            sound: Sound::new(),
            any_change: false,
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, kc: &mut KRCanvas) {
        kc.set_camera(inputs.screen_rect);
        kc.set_depth(1.0);
        kc.set_colour(Vec4::new(0.8, 0.4, 0.2, 1.0));
        kc.rect(inputs.screen_rect);
        kc.set_depth(1.1);
        
        let synth_area = inputs.screen_rect.dilate_pc(-0.03);
        
        let (top, bot) = synth_area.split_ud(0.5);
        let (tl, tr) = top.split_lr(0.5);
        kc.set_colour(Vec4::new(0.8, 0.2, 0.2, 1.0));
        kc.rect(tl);
        kc.set_colour(Vec4::new(0.2, 0.8, 0.2, 1.0));
        kc.rect(tr);
        
        // envelope
        let env = tl.dilate_pc(-0.05);

        self.any_change |= label_slider("A", env.grid_child(0, 0, 3, 1).dilate_pc(-0.02), 0.0, 3.0, &mut self.sound.A, false, inputs, kc);
        self.any_change |= label_slider("S", env.grid_child(1, 0, 3, 1).dilate_pc(-0.02), 0.0, 3.0, &mut self.sound.S, false, inputs, kc);
        self.any_change |= label_slider("R", env.grid_child(2, 0, 3, 1).dilate_pc(-0.02), 0.0, 3.0, &mut self.sound.R, false, inputs, kc);

        // lfo
        let lfo = tr.dilate_pc(-0.05);
        let (lfof, lfoa) = lfo.split_lr(0.5);
        self.any_change |= label_slider("lfo amp", lfoa, 0.0, 1.0, &mut self.sound.amp_lfo_amount, false, inputs, kc);
        self.any_change |= label_slider("lfo freq", lfof, 0.0, 100.0, &mut self.sound.amp_lfo_freq, true, inputs, kc);

        let (bl, br) = bot.split_lr(0.9);
        let fmods = bl;
        let volume = br;

        {   // fmods
            let (freq, rest) = fmods.split_lr(0.2);
            self.any_change |= label_slider("base freq", freq, 50.0, 10000.0, &mut self.sound.freq, true, inputs, kc);

            let (fmod, fmodmod) = rest.split_lr(0.5);

            let (fmr, fma) = fmod.split_lr(0.5);
            self.any_change |= label_slider("fmod ratio", fmr, 0.0, 3.0, &mut self.sound.fmod_carrier_ratio, true, inputs, kc);
            self.any_change |= label_slider("fmod amt", fma, 0.0, 2.0, &mut self.sound.fmod_amt, false, inputs, kc);

            let (fmmr, fmma) = fmodmod.split_lr(0.5);
            self.any_change |= label_slider("fmm ratio", fmmr, 0.0, 3.0, &mut self.sound.fmmod_carrier_ratio, true, inputs, kc);
            self.any_change |= label_slider("fmm amt", fmma, 0.0, 2.0, &mut self.sound.fmmod_amt, false, inputs, kc);
        }

        self.any_change |= label_slider("vol", volume, 0.0, 1.0, &mut self.sound.amplitude, false, inputs, kc);


    }
}



// so remapping the exponential
// 10, 1000, 0.5 => 100
// 10, 1000, 0.99 => 1000
// 10, 1000, 0.01 => 11

// min + (max - min) * t // linear
// min + 2^t(log2 max)
// min + f((max - min), t) s.t. if t = 0, = 0 t = 1, 1

// f(min, max, t) s.t. t = 0 min, t = 1 max, t 0.5 log_min max

fn label_slider(label: &str, r: Rect, min: f32, max: f32, val: &mut f32, log: bool, inputs: &FrameInputState, kc: &mut KRCanvas) -> bool {
    kc.set_depth(1.5);
    let r = r.dilate_pc(-0.02);
    let (text, slider_rect) = r.split_ud(0.05);
    kc.text_center(label.as_bytes(), text);
    slider(slider_rect, min, max, val, log, inputs, kc)
}

fn slider(r: Rect, min: f32, max: f32, val: &mut f32, log: bool, inputs: &FrameInputState, kc: &mut KRCanvas) -> bool {
    let r = r.fit_aspect_ratio(0.25);

    kc.set_depth(2.0);
    kc.set_colour(Vec4::new(0.2, 0.2, 0.2, 1.0));
    kc.rect(r);
    kc.set_depth(2.1);
    kc.set_colour(Vec4::new(0.9, 0.9, 0.9, 1.0));
    kc.rect(r.fit_aspect_ratio(0.01));
    kc.set_depth(2.2);
    kc.set_colour(Vec4::new(0.7, 0.7, 0.7, 1.0));

    
    let mut slider_t = 0.0f32;
    let change = r.contains(inputs.mouse_pos) && inputs.lmb == KeyStatus::Pressed;
    if change {
        slider_t = unlerp(inputs.mouse_pos.y, r.bot(), r.top());
        if slider_t < 0.01 {
            slider_t = 0.0;
        }
        if slider_t > 1.0 - 0.01 {
            slider_t = 1.0;
        }
        if log {
            *val = min + 2.0f32.powf(slider_t * (max - min).log2()) - 1.0;
        } else {
            *val = lerp(min, max, slider_t);
        }
    } else {
        if log {
            // slider t is inverse of that log formula
            slider_t = (*val + 1.0 - min).log2() / (max - min).log2();
        } else {
            // slider t is linear inverse thing
            slider_t = unlerp(*val, min, max);
        }
    }


    // linear sliders are wrong when not change, but right when change
    // i mean it looks so simple and correct, remap val, but actually it is wrong anyway / i dont understand why its to low top

    let slider_pos = lerp(r.bot(), r.top(), slider_t);
    let rect_ish = r.dilate_pc(-0.05).fit_aspect_ratio(2.0);
    let slider_rect = Rect::new_centered(r.centroid().x, slider_pos, rect_ish.w, rect_ish.h);
    kc.rect(slider_rect);
    kc.set_depth(2.3);
    kc.text_center(format!("{:.2}", *val).as_bytes(), slider_rect);
    change

    // also render text and name of contained value
}