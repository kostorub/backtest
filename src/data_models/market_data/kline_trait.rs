pub trait KLineTrait {
    fn date(&self) -> i64;
    fn open(&self) -> f64;
    fn high(&self) -> f64;
    fn low(&self) -> f64;
    fn close(&self) -> f64;
    fn qty(&self) -> f64;
}
