use zemen::Zemen;

use crate::parser::Rule;

use super::{Calender, DisplayPlain};

impl From<Zemen> for Calender {
    fn from(value: zemen::Zemen) -> Self {
        Calender::Zemen(value)
    }
}

impl DisplayPlain for Zemen {
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.month(), self.day(), self.year())
    }
}

fn werh_from_quex(as_str: &str) -> zemen::Werh {
    match as_str {
        "mes" | "መስከ" => zemen::Werh::Meskerem,
        "tik" | "ጥቅም" => zemen::Werh::Tikimit,
        "hed" | "ህዳር" => zemen::Werh::Hedar,
        "tah" | "ታኅሣ" => zemen::Werh::Tahasass,
        "tir" | "ጥር" => zemen::Werh::Tir,
        "yek" | "የካቲ" => zemen::Werh::Yekatit,
        "meg" | "መጋቢ" => zemen::Werh::Megabit,
        "miy" | "ሚያዝ" => zemen::Werh::Miyazia,
        "gin" | "ግንቦ" => zemen::Werh::Ginbot,
        "sen" | "ሴኒ" => zemen::Werh::Sene,
        "ham" | "ሐምሌ" => zemen::Werh::Hamle,
        "neh" | "ነሐሴ" => zemen::Werh::Nehase,
        "pua" | "ጳጉሜ" => zemen::Werh::Puagme,
        _ => unreachable!(),
    }
}

pub fn parse_eth_date(
    mut ethiopian_date: pest::iterators::Pairs<Rule>,
    mut description: String,
) -> (Result<Zemen, zemen::error::Error>, String) {
    let year = ethiopian_date.next().unwrap(); // won't fail
    let mut year_str = year.as_str();
    let is_named_year = year.as_rule() == Rule::named_yearly;
    let today = Zemen::today();

    if is_named_year {
        year_str = year.as_str().strip_suffix('*').unwrap(); // won't fail
        let years_past = today.year() - year_str.parse::<i32>().unwrap(); // won't fail

        description = description
            .replace("\\y", year_str)
            .replace("\\a", &years_past.to_string());
    }

    let month = ethiopian_date.next().unwrap(); // won't fail
    let day = ethiopian_date.next().unwrap(); // won't fail

    (
        Zemen::from_eth_cal(
            year_str
                .parse()
                .map(|year| {
                    // helpful when printing all the past schedules.
                    // It won't replace their years with the current year.
                    if year < today.year() && is_named_year {
                        return today.year();
                    }
                    year
                })
                .unwrap_or(today.year()),
            werh_from_quex(month.as_str()),
            day.as_str().parse().unwrap(),
        ),
        description,
    )



}
