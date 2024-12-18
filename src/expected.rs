use std::collections::HashMap;
use std::sync::LazyLock;


pub const SOLUTIONS: LazyLock<HashMap<u32, (&str, &str)>> = LazyLock::new(|| HashMap::from([
    (1, ("2066446", "24931009")),
    (2, ("421", "476")),
    (3, ("180233229", "95411583")),
    (4, ("2427", "1900")),
    (5, ("5955", "4030")),
    (6, ("5208", "1972")),
    (7, ("66343330034722", "637696070419031")),
    (8, ("273", "1017")),
    (9, ("6337921897505", "6362722604045")),
    (10, ("510", "1058")),
    (11, ("190865", "225404711855335")),
    (12, ("1477762", "923480")),
    (13, ("35255", "87582154060429")),
    (14, ("215987200", "0")),
    (15, ("1371036", "1392847")),
    (16, ("102488", "559")),
    // (17, ("102488", "559")),// add
    (18, ("270", "51,40")),
]));
