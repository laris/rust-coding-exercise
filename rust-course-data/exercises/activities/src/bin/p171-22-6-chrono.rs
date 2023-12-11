//# chrono = "*"
use chrono::prelude::*;

fn main() {
    let utc = Utc::now();
    println!("Utc::now(): {utc:?}");
    let local = Local::now();
    println!("Local::now(): {local:?}");

    //EST is UTC-5
    let est = FixedOffset::west(5 * 3600)
        .ymd(2021, 05, 16)
        .and_hms(13, 45, 0);
    println!("{est}");

    let utc = est.with_timezone(&Utc);
    let utc: DateTime<Utc> = est.into();
    println!("utc: {utc:?}");

    let est_tz = FixedOffset::west(5 * 3600);
    let est = utc.with_timezone(&est_tz);
    println!("est: {est:?}");

    use chrono::Duration;
    let now = Utc::now();
    let an_hour = Duration::hours(1);
    let when = now + an_hour;
    let duration = when - now;
    println!("       now: {now:?}");
    println!("in an hour: {when:?}");
    println!("duration: {}", duration.num_hours());

    // parsing
    let date_str = "2020-08-23T10:00:00Z";
    let dt = date_str.parse::<DateTime<Utc>>();
    println!("{dt:?}");

    let date_str = "2020-08-23T10:00:00 +03:00";
    let fmt_str = "%Y-%m-%dT%H:%M:%S %z";
    let dt = DateTime::parse_from_str(date_str, fmt_str);
    println!("{dt:?}");

    let date_str = "2020-08-23 10:00:00";
    let fmt_str = "%Y-%m-%d %H:%M:%S";
    let dt = Utc.datetime_from_str(date_str, fmt_str);
    println!("{dt:?}");

    let date = Local::now();
    let fmt_str = "%Y-%m-%d %H:%M:%S";
    let fmt_str = "%Y.%m.%d-%H.%M.%S";
    println!("{}", date.format(fmt_str));
}
