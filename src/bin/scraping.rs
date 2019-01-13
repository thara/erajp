extern crate reqwest;
extern crate scraper;
use scraper::{Html, Selector};

extern crate encoding;
use encoding::all::EUC_JP;
use encoding::{DecoderTrap, Encoding};

extern crate csv;

static TARGET_URL: &str = "http://www.kumamotokokufu-h.ed.jp/kumamoto/bungaku/nengoui.html";

fn main() {
    let mut resp = reqwest::get(TARGET_URL).unwrap();
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf).unwrap();

    let html = EUC_JP.decode(&buf, DecoderTrap::Strict).unwrap();

    let document = Html::parse_document(&html[..]);
    let tr_selector = Selector::parse(r#"table[border="1"] tr"#).unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let (mut prevy, mut prevm, mut prevd): (String, String, String) =
        ("".to_string(), "".to_string(), "".to_string());
    for node in document.select(&tr_selector) {
        let cells = node.select(&td_selector).collect::<Vec<_>>();
        let n_cells = cells.len();

        if n_cells <= 0 {
            continue;
        }
        if cells[1].value().attr("rowspan").is_some() {
            continue;
        }
        let values = match cells.len() {
            6 => Some((
                cells[1].text().collect::<Vec<_>>().join(""),
                cells[2].text().collect::<Vec<_>>().join(""),
                cells[3].text().collect::<Vec<_>>().join(""),
                cells[4].text().collect::<Vec<_>>().join(""),
            )),
            7 => Some((
                cells[2].text().collect::<Vec<_>>().join(""),
                cells[3].text().collect::<Vec<_>>().join(""),
                cells[4].text().collect::<Vec<_>>().join(""),
                cells[5].text().collect::<Vec<_>>().join(""),
            )),
            _ => None,
        };
        if values.is_none() {
            continue;
        }
        let (name, ruby, term, changed_at) = values.unwrap();

        let term: Vec<&str> = term.split("～").collect();
        let changed_at = changed_at.replace("閏", "");
        let changed_at: Vec<&str> = changed_at
            .split("/")
            .map(|e| if e == "？" { "1" } else { e })
            .collect();

        if term[0] == prevy && changed_at[0] == prevm && changed_at[1] == prevd {
            continue;
        }

        let record = format!(
            "{},{},{},{},{}",
            name, ruby, term[0], changed_at[0], changed_at[1]
        );
        println!("{}", record);

        prevy = term[0].to_string();
        prevm = changed_at[0].to_string();
        prevd = changed_at[1].to_string();
    }
}
