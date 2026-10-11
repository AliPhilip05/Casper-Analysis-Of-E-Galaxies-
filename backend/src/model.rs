/// One galaxy row from the input catalog.
#[derive(Debug, Clone)]
pub struct Galaxy {
    pub object_id: String,
    pub g_flux: f64,
    pub r_flux: f64,
    pub i_flux: f64,
    pub z_flux: f64,
    pub y_flux: f64,
    pub u_flux: f64,
    pub shape_xx: f64,
    pub shape_yy: f64,
}

/// CAS measurements for one galaxy in one photometric band.
#[derive(Debug, Clone)]
pub struct CasResult {
    pub index: usize,
    pub object_id: String,
    pub band: String,
    pub concentration: f64,
    pub asymmetry: f64,
    pub smoothness: f64,
}
