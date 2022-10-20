use self::twelve_tone::TwelveTone;
pub mod twelve_tone;

pub fn create_notes_keyboard() -> TwelveTone{
    let mut a : Vec<f32> = Vec::with_capacity(6);
    let mut a_sharp : Vec<f32> = Vec::with_capacity(6);
    let mut b : Vec<f32> = Vec::with_capacity(6);
    let mut c : Vec<f32> = Vec::with_capacity(6);
    let mut c_sharp : Vec<f32> = Vec::with_capacity(6);
    let mut d : Vec<f32> = Vec::with_capacity(6);
    let mut d_sharp : Vec<f32> = Vec::with_capacity(6);
    let mut e : Vec<f32> = Vec::with_capacity(6);
    let mut f : Vec<f32> = Vec::with_capacity(6);
    let mut f_sharp : Vec<f32> = Vec::with_capacity(6);
    let mut g : Vec<f32> = Vec::with_capacity(6);
    let mut g_sharp : Vec<f32> = Vec::with_capacity(6);

    let mut keyboard = TwelveTone::new(a,a_sharp,b,c,c_sharp,d,d_sharp,e,f,f_sharp,g,g_sharp);
    return keyboard;    
}
