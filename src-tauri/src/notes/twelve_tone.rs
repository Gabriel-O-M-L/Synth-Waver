#[derive(Clone,Debug)]
pub struct TwelveTone{
    pub a : Vec<f32>,
    pub a_sharp : Vec<f32>,
    pub b : Vec<f32>,
    pub c : Vec<f32>,
    pub c_sharp : Vec<f32>,
    pub d : Vec<f32>,
    pub d_sharp : Vec<f32>,
    pub e : Vec<f32>,
    pub f : Vec<f32>,
    pub f_sharp : Vec<f32>,
    pub g : Vec<f32>,
    pub g_sharp : Vec<f32>
}
impl TwelveTone{
    pub fn new(
        a_vec:Vec<f32>,
        a_sharp_vec:Vec<f32>,
        b_vec:Vec<f32>,
        c_vec:Vec<f32>,
        c_sharp_vec:Vec<f32>,
        d_vec:Vec<f32>,
        d_sharp_vec:Vec<f32>,
        e_vec:Vec<f32>,
        f_vec:Vec<f32>,
        f_sharp_vec:Vec<f32>,
        g_vec:Vec<f32>,
        g_sharp_vec:Vec<f32>) -> TwelveTone {
            return TwelveTone{
            a : a_vec,
            a_sharp : a_sharp_vec, 
            b : b_vec,
            c : c_vec,
            c_sharp : c_sharp_vec,
            d : d_vec,
            d_sharp : d_sharp_vec,
            e : e_vec,
            f : f_vec,
            f_sharp : f_sharp_vec, 
            g : g_vec,
            g_sharp : g_sharp_vec 
        }
    } 
    pub fn create_notes_frequency(&mut self){
        self.a.push(55 as f32);
        self.b.push(61.173);
        self.c.push(32.703);
        self.d.push(36.708);
        self.e.push(41.203);
        self.f.push(43.654);
        self.g.push(48.999);
        self.a_sharp.push(58.27);
        self.c_sharp.push(34.648);
        self.d_sharp.push(38.891);
        self.f_sharp.push(46.249);
        self.g_sharp.push(51.913);

        for n in 1..self.a.capacity() {
            self.a.push((self.a[n-1]*2 as f32) as f32);
            self.b.push((self.b[n-1]*2 as f32) as f32);
            self.c.push((self.c[n-1]*2 as f32) as f32);
            self.d.push((self.d[n-1]*2 as f32) as f32);
            self.e.push((self.e[n-1]*2 as f32) as f32);
            self.f.push((self.f[n-1]*2 as f32) as f32);
            self.g.push((self.g[n-1]*2 as f32) as f32);
            self.a_sharp.push((self.a_sharp[n-1]*2 as f32) as f32);
            self.c_sharp.push((self.c_sharp[n-1]*2 as f32) as f32);
            self.d_sharp.push((self.d_sharp[n-1]*2 as f32) as f32);
            self.f_sharp.push((self.f_sharp[n-1]*2 as f32) as f32);
            self.g_sharp.push((self.g_sharp[n-1]*2 as f32) as f32);
        }
    }
}