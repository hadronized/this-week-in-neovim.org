use chrono::prelude::*;

#[test]
fn test_building_dates() {
  let date = Utc.isoywd(2022, 26, Weekday::Mon);

  assert_eq!(date.year(), 2022);
  assert_eq!(date.month(), 6);
  assert_eq!(date.day(), 27);

  let w = "Mon".parse::<Weekday>();
  assert_eq!(w, Ok(Weekday::Mon));

  let m = "Mar".parse::<Month>();
  assert_eq!(m, Ok(Month::March));
}
