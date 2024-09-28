// use time::error::ComponentRange;

// pub use super::{time_span::TimeSpan, Rule};

// pub fn get_time_description(
//     schedule: &mut pest::iterators::Pairs<Rule>,
// ) -> Result<(Option<TimeSpan>, String), ComponentRange> {
//     let description;
//     let mut time: Option<TimeSpan> = None;

//     let pos = schedule.next().unwrap();

//     if pos.as_rule() == Rule::time {
//         let time_pair = pos.into_inner().next().unwrap(); // won't fail
//         match time_pair.as_rule() {
//             Rule::am_pm => {
//                 let am_pm: String = time_pair
//                     .as_str()
//                     .chars()
//                     .filter(|c| char::is_alphabetic(*c))
//                     .collect();
//                 let hm = time_pair
//                     .into_inner()
//                     .as_str()
//                     .split(':')
//                     .collect::<Vec<_>>();

//                 let mut hour = hm[0].parse().unwrap(); // won't fail
//                 if am_pm == "PM" {
//                     hour += 12;
//                 }
//                 let minute = hm[1].parse().unwrap(); // won't fail

//                 time = Some(TimeSpan::new_unit(time::Time::from_hms(hour, minute, 0)?));
//             }
//             Rule::clock => {
//                 let hm = time_pair.as_str().split(':').collect::<Vec<_>>();
//                 let hour = hm[0].parse().unwrap(); // won't fail
//                 let minute = hm[1].parse().unwrap(); // won't fail

//                 time = Some(TimeSpan::new_unit(time::Time::from_hms(hour, minute, 0)?));
//             }
//             _ => unreachable!(),
//         }
//         description = schedule.next().unwrap().as_str().to_string();

//     // A schedule planed to be done in a range of time
//     } else if pos.as_rule() == Rule::time_range {
//         let mut time_range = pos.into_inner(); // won't fail

//         let time_a = time_range.next().unwrap();
//         let time_b = time_range.next().unwrap();

//         let time_a = extract_time(time_a.as_str())?;
//         let time_b = extract_time(time_b.as_str())?;

//         time = Some(TimeSpan::new_range(time_a, time_b));

//         description = schedule.next().unwrap().as_str().to_string();
//     } else {
//         description = pos.as_str().to_string();
//     }

//     Ok((time, description))
// }

// fn extract_time(time: &str) -> Result<time::Time, time::error::ComponentRange> {
//     let am_pm: String = time.chars().filter(|c| char::is_alphabetic(*c)).collect();

//     let time = time.to_uppercase().replace("AM", "").replace("PM", "");
//     let time = time.trim();

//     let mut time = time.split(':');

//     let mut h = time.next().unwrap().parse().unwrap();
//     let m = time.next().unwrap().parse().unwrap();

//     if am_pm.to_uppercase() == "PM" {
//         h += 12
//     }

//     time::Time::from_hms(h, m, 0)
// }
