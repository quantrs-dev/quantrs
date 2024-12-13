pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut sma = vec![0.0; data.len()];
    for i in period..data.len() {
        sma[i] = data[i - period..i].iter().sum::<f64>() / period as f64;
    }
    sma
}
