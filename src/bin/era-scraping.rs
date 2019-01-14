use std::fs;
use std::io::{BufWriter, Write};

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

    let parsed = document
        .select(&tr_selector)
        .map(|e| e.select(&td_selector).collect::<Vec<_>>())
        .map(|e| (e.len(), e))
        .filter(|(n_cells, _)| 0 < *n_cells)
        .filter(|(_, cells)| cells[1].value().attr("rowspan").is_none())
        .filter_map(|(n_cells, cells)| match n_cells {
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
        });

    let mut prev: Option<(String, String, String)> = None;
    let mut f = BufWriter::new(fs::File::create("src/era.csv").unwrap());

    for (name, ruby, term, changed_at) in parsed {
        let term: Vec<&str> = term.split("～").collect();
        let changed_at = changed_at.replace("閏", "");
        let changed_at: Vec<&str> = changed_at
            .split("/")
            .map(|e| if e == "？" { "1" } else { e })
            .collect();

        if let Some((prevy, prevm, prevd)) = &prev {
            if term[0] == prevy && changed_at[0] == prevm && changed_at[1] == prevd {
                continue;
            }
        }
        let record = format!(
            "{},{},{},{},{}\n",
            name, ruby, term[0], changed_at[0], changed_at[1]
        );
        f.write(record.as_bytes()).unwrap();

        prev = Some((
            term[0].to_string(),
            changed_at[0].to_string(),
            changed_at[1].to_string(),
        ))
    }
}
