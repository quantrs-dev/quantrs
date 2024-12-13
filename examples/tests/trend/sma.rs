#[test]
fn it_calculates_sma() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let sma = quantrs::averages::sma(&data, 3);
    let expected = vec![0.0, 0.0, 0.0, 2.0, 3.0];

    assert_eq!(sma, expected);
}
