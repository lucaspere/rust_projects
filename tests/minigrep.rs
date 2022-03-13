use minigrep::*;

#[test]
fn should_run_correctly() {
    let config = Config {
        case_sensitive: true,
        filename: "cronograma.txt".to_string(),
        query: "feriado".to_string(),
    };

    let result = minigrep::run(config).is_ok();

    assert!(result);
}

#[test]
fn should_occur_error() {
    let config = Config {
        case_sensitive: true,
        filename: "cronograma.csv".to_string(),
        query: "feriado".to_string(),
    };

    let result = minigrep::run(config).is_err();

    assert!(result);
}
