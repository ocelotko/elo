pub fn probability(rating1: f64, rating2: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((rating2 - rating1) / 400.0))
}

pub fn elo_rating(ra: f64, rb: f64, k: i32, outcome: f64) -> (f64, f64) {
    let pa = probability(ra, rb);
    let pb = probability(rb, ra);

    let new_ra = ra + k as f64 * (outcome - pa);
    let new_rb = rb + k as f64 * ((1.0 - outcome) - pb);

    (new_ra, new_rb)
}