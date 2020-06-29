// π (std::f64::consts::PI is not used for compatibility with the reference implementation)
pub const PI: f64 = 3.14159265358979323846;

// θ̇ = 4.37526908801129966 × 10⁻³ rad.min⁻¹
pub const SIDEREAL_SPEED: f64 = 4.37526908801129966e-3;

pub struct Geopotential {
    pub ae: f64, // equatorial radius of the earth in km, aₑ
    pub ke: f64, // square root of earth's gravitational parameter in earth radii³ min⁻², kₑ
    pub j2: f64, // un-normalised second zonal harmonic, J₂
    pub j3: f64, // un-normalised third zonal harmonic, J₃
    pub j4: f64, // un-normalised fourth zonal harmonic, J₄
}

pub const WGS72OLD: Geopotential = Geopotential {
    ae: 6378.135,
    ke: 0.0743669161,
    j2: 0.001082616,
    j3: -0.00000253881,
    j4: -0.00000165597,
};

pub const WGS72: Geopotential = Geopotential {
    ae: 6378.135,
    ke: 0.07436691613317342,
    j2: 0.001082616,
    j3: -0.00000253881,
    j4: -0.00000165597,
};

pub const WGS84: Geopotential = Geopotential {
    ae: 6378.137,
    ke: 0.07436685316871385,
    j2: 0.00108262998905,
    j3: -0.00000253215306,
    j4: -0.00000161098761,
};

// t0: years since UTC 1 January 2000 12h00 t₀
pub fn afspc_epoch_to_sidereal_time(t0: f64) -> f64 {
    // t₁₉₇₀ = 365.25 (t₀ + 30)
    let t1970 = (t0 + 30.0) * 365.25 + 1.0;

    // θ₀ = 1.7321343856509374 + 1.72027916940703639 × 10⁻² ⌊t₁₉₇₀ + 10⁻⁸⌋
    //      + (1.72027916940703639 × 10⁻² + 2π) (t₁₉₇₀ - ⌊t₁₉₇₀ + 10⁻⁸⌋)
    //      + 5.07551419432269442 × 10⁻¹⁵ t₁₉₇₀² mod 2π
    (1.7321343856509374
        + 1.72027916940703639e-2 * (t1970 + 1.0e-8).floor()
        + (1.72027916940703639e-2 + 2.0 * PI) * (t1970 - (t1970 + 1.0e-8).floor())
        + t1970.powi(2) * 5.07551419432269442e-15)
        .rem_euclid(2.0 * PI)
}

// t0: years since UTC 1 January 2000 12h00 t₀
pub fn iau_epoch_to_sidereal_time(t0: f64) -> f64 {
    // t₂₀₀₀ = t₀ / 100
    let t2000 = t0 / 100.0;

    // θ₀ = ¹/₂₄₀ (π / 180) (- 6.2 × 10⁻⁶ t₂₀₀₀³ + 0.093104 t₂₀₀₀²
    //      + (876600 × 3600 + 8640184.812866) t₂₀₀₀ + 67310.54841) mod 2π
    ((-6.2e-6 * t2000.powi(3)
        + 0.093104 * t2000.powi(2)
        + (876600.0 * 3600.0 + 8640184.812866) * t2000
        + 67310.54841)
        * (PI / 180.0)
        / 240.0)
        .rem_euclid(2.0 * PI)
}