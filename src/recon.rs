use crate::geo::{
    Coordinate, Latitude,
    LatitudeHemisphere::{NORTH, SOUTH},
    Longitude,
    LongitudeHemisphere::{EAST, WEST},
};
use crate::measure::{
    Altitude, Angle, DValue, Direction, Pressure, RainRate, Speed, Temperature, Wind,
};

use chrono::{Date, DateTime, TimeZone, Utc};
use lazy_static::lazy_static;
use regex::Regex;

const MISSING: &str = "///";

#[derive(Debug)]
pub struct HDOBMessage {
    pub header: String,
    pub mission_id: String,
    pub obs_number: u32,
    pub date: Date<Utc>,
    pub obs: Vec<HDHALog>,
}

impl HDOBMessage {
    pub fn parse(hdob: &str) -> Self {
        let mut lines = hdob.lines().skip(1);
        let header = lines.next().unwrap().trim().to_string();
        let mission_header = lines.next().expect("No header");
        let re =
            Regex::new(r"([A-Z0-9 ]*) HDOB ([0-9]{2}) ([0-9]{4})([0-9]{2})([0-9]{2})").unwrap();
        let captures = re.captures(mission_header).unwrap();
        let mission_id = captures.get(1).unwrap().as_str().trim().to_string();
        let obs_number = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .expect("Unable to parse obs number");
        let y = captures.get(3).unwrap().as_str().parse().unwrap();
        let m = captures.get(4).unwrap().as_str().parse().unwrap();
        let d = captures.get(5).unwrap().as_str().parse().unwrap();
        let date = Utc.ymd(y, m, d);
        let mut obs = vec![];
        for line in lines {
            if line == "$$" {
                break;
            }
            let log = HDHALog::parse(&date, line);
            obs.push(log);
        }

        Self {
            header,
            mission_id,
            obs_number,
            date,
            obs,
        }
    }
}

#[derive(Debug)]
pub struct HDHALog {
    pub time: DateTime<Utc>,
    pub location: Coordinate,
    pub aircraft_pressure: Pressure,
    pub height: Altitude,
    pub surface_pressure: Option<ExtrapolatedSurfacePressure>,
    pub temp: Option<Temperature>,
    pub dewpoint: Option<Temperature>,
    pub wind: Option<Wind>,
    pub peak_wind_speed: Option<Speed>,
    pub peak_sfmr_speed: Option<Speed>,
    pub rain_rate: Option<RainRate>,
    pub latlon_questionable: bool,
    pub altitude_or_pressure_questionable: bool,
    pub temp_or_dewpoint_questionable: bool,
    pub winds_questionable: bool,
    pub sfmr_questionable: bool,
}

impl HDHALog {
    pub fn parse(date: &Date<Utc>, line: &str) -> Self {
        let mut cols = line.split(" ");

        let time = parse_hhmmss(date, cols.next().expect("Missing time"));
        let location = parse_latlon(
            cols.next().expect("Missing lat"),
            cols.next().expect("Missing lon"),
        );
        let aircraft_pressure = parse_aircraft_pressure(cols.next().expect("Missing pressure"));
        let height = Altitude::with_meters(cols.next().expect("Missing altitude").parse().unwrap());
        let surface_pressure =
            parse_extrapolated_sfc_pressure(aircraft_pressure, cols.next().expect("Missing ESP"));
        let temp = parse_temperature(cols.next().expect("Missing temp"));
        let dewpoint = parse_temperature(cols.next().expect("Missing dewpoint"));
        let wind = parse_wind(cols.next().expect("Missing wind dir."));
        let peak_wind_speed = parse_speed(cols.next().expect("Missing gusts"));
        let peak_sfmr_speed = parse_speed(cols.next().expect("Missing sfmr"));
        let rain_rate = parse_rain_rate(cols.next().expect("Missing rain rate"));

        let quality = cols.next().expect("Missing quality").parse::<u8>().unwrap();
        let (latlon_questionable, altitude_or_pressure_questionable) = match quality / 10 {
            0 => (false, false),
            1 => (true, false),
            2 => (false, true),
            3 => (true, true),
            x => panic!("Unexpected pos quality: {}", x),
        };

        let (temp_or_dewpoint_questionable, winds_questionable, sfmr_questionable) =
            match quality % 10 {
                0 => (false, false, false),
                1 => (true, false, false),
                2 => (false, true, false),
                3 => (false, false, true),
                4 => (true, true, false),
                5 => (true, false, true),
                6 => (false, true, true),
                9 => (true, true, true),
                x => panic!("Unexpected met quality: {}", x),
            };

        HDHALog {
            time,
            location,
            aircraft_pressure,
            height,
            surface_pressure,
            temp,
            dewpoint,
            wind,
            peak_wind_speed,
            peak_sfmr_speed,
            rain_rate,
            latlon_questionable,
            altitude_or_pressure_questionable,
            temp_or_dewpoint_questionable,
            winds_questionable,
            sfmr_questionable,
        }
    }
}

#[test]
fn test_parse_hdob() {
    let earl1 = include_str!("../testdata/hdob/20220905-31-HDOB-EARL-0906A-NOAA2.txt");
    let _ = HDOBMessage::parse(earl1);
    //println!("{:#?}", attempt);

    let earl2 = include_str!("../testdata/hdob/20220905-09-HDOB-EARL-1006A-AF308.txt");
    let _ = HDOBMessage::parse(earl2);
    //println!("{:#?}", attempt);

    let earl3 = include_str!("../testdata/hdob/20220903-15-HDOB-EARL-0606A-AF307.txt");
    let _ = HDOBMessage::parse(earl3);
    //println!("{:#?}", attempt)

    let kay1 = include_str!("../testdata/hdob/20220905-12-HDOB-KAY-0112E-AF309.txt");
    let attempt = HDOBMessage::parse(kay1);
    println!("{:#?}", attempt);
}

#[test]
fn test_parse_hdha() {
    let date = Utc.ymd(2022, 09, 01);
    const LINE1: &str = "181830 2006N 06141W 9236 00794 0115 +201 +173 123041 041 021 002 00";

    let attempt = HDHALog::parse(&date, LINE1);
    println!("{:#?}", attempt);

    const LINES2: &str = "135600 1821N 06526W 7752 02317 0126 +145 +051 234022 023 /// /// 03
135630 1819N 06525W 7799 02267 0124 +152 +059 237021 021 /// /// 03
135700 1817N 06524W 7800 02264 0119 +155 +062 239021 022 /// /// 03
135730 1815N 06523W 7799 02266 0115 +160 +066 244021 022 /// /// 03
135800 1813N 06522W 7798 02268 0122 +152 +073 228020 021 /// /// 03
135830 1811N 06521W 7799 02264 0119 +153 +074 227021 023 /// /// 03
135900 1810N 06520W 7800 02263 0117 +156 +074 237019 020 /// /// 03
135930 1808N 06519W 7800 02262 0118 +152 +078 227021 022 /// /// 03
140000 1806N 06517W 7800 02262 0117 +153 +079 232022 023 /// /// 03
140030 1805N 06516W 7893 02165 0118 +163 +083 234021 022 /// /// 03
140100 1803N 06514W 8079 01968 0123 +170 +089 221022 023 /// /// 03
140130 1802N 06513W 8296 01742 0123 +184 +094 222023 023 /// /// 03
140200 1801N 06511W 8517 01511 0124 +196 +115 222024 025 /// /// 03
140230 1759N 06510W 8760 01275 0127 +207 +146 224023 024 /// /// 03
140300 1758N 06508W 9007 01035 0127 +222 +187 218025 027 /// /// 03
140330 1757N 06506W 9234 00815 0129 +233 +212 214024 026 /// /// 03
140400 1756N 06505W 9234 00816 0130 +235 +220 211025 026 /// /// 03
140430 1755N 06504W 9278 00779 0129 +238 +210 214027 027 /// /// 03
140500 1753N 06502W 9278 00779 0132 +238 +213 215027 028 /// /// 03
140530 1752N 06501W 9278 00779 0131 +239 +213 213027 028 /// /// 03";
    for hdha in LINES2.lines().map(|it| HDHALog::parse(&date, it)) {
        println!("{:?}", hdha)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExtrapolatedSurfacePressure {
    ExtrapolatedPressure(Pressure),
    DValue(DValue),
}

fn parse_hhmmss<TZ: TimeZone>(date: &Date<TZ>, hhmmss: &str) -> DateTime<TZ> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]{2})([0-9]{2})([0-9]{2})").unwrap();
    }

    let captures = RE.captures(hhmmss).unwrap();
    let hours = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let mins = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let secs = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    date.and_hms(hours, mins, secs)
}

#[test]
fn test_parse_hms() {
    let expected = Utc.ymd(2022, 09, 01).and_hms(18, 03, 09);
    let attempt = parse_hhmmss(&Utc.ymd(2022, 09, 01), "180309");
    assert_eq!(expected, attempt)
}

fn parse_latlon(llllh: &str, nnnnnh: &str) -> Coordinate {
    lazy_static! {
        static ref RELAT: Regex = Regex::new(r"([0-9]{2})([0-9]{2})([NS])").unwrap();
        static ref RELON: Regex = Regex::new(r"([0-9]{3})([0-9]{2})([EW])").unwrap();
    }

    let captures_lat = RELAT.captures(llllh).unwrap();
    let captures_lon = RELON.captures(nnnnnh).unwrap();

    let hemi_lat = match captures_lat.get(3).unwrap().as_str() {
        "N" => NORTH,
        "S" => SOUTH,
        _ => panic!(),
    };
    let hemi_lon = match captures_lon.get(3).unwrap().as_str() {
        "E" => EAST,
        "W" => WEST,
        _ => panic!(),
    };

    Coordinate {
        latitude: Latitude {
            angle: Angle::with_degrees_minutes_seconds(
                captures_lat.get(1).unwrap().as_str().parse().unwrap(),
                captures_lat.get(2).unwrap().as_str().parse().unwrap(),
                0,
            ),
            hemisphere: hemi_lat,
        },
        longitude: Longitude {
            angle: Angle::with_degrees_minutes_seconds(
                captures_lon.get(1).unwrap().as_str().parse().unwrap(),
                captures_lon.get(2).unwrap().as_str().parse().unwrap(),
                0,
            ),
            hemisphere: hemi_lon,
        },
    }
}

#[test]
fn test_parse_latlon() {
    // 2006N 06141W
    let expected = Coordinate {
        latitude: Latitude {
            angle: Angle::with_degrees_minutes_seconds(20, 06, 0),
            hemisphere: NORTH,
        },
        longitude: Longitude {
            angle: Angle::with_degrees_minutes_seconds(061, 41, 0),
            hemisphere: WEST,
        },
    };
    let attempt = parse_latlon("2006N", "06141W");
    assert_eq!(expected, attempt);
}

fn parse_aircraft_pressure(pppp: &str) -> Pressure {
    let raw: i32 = pppp.parse().unwrap();
    // Aircraft static air pressure, in tenths of mb with decimal omitted
    if raw > 2000 {
        // leading 1 not dropped
        Pressure::with_microbars(raw * 100)
    } else {
        // leading 1 dropped
        Pressure::with_microbars((raw + 10000) * 100)
    }
}

#[test]
fn test_parse_aircraft_pressure() {
    let expected1 = Pressure::with_microbars(923_600);
    let attempt1 = parse_aircraft_pressure("9236");
    assert_eq!(expected1, attempt1);

    let expected2 = Pressure::with_microbars(1_023_400);
    let attempt2 = parse_aircraft_pressure("0234");
    assert_eq!(expected2, attempt2);
}

fn parse_extrapolated_sfc_pressure(
    altitude: Pressure,
    xxxx: &str,
) -> Option<ExtrapolatedSurfacePressure> {
    if xxxx == MISSING {
        None
    } else {
        if altitude.millibars() < 550 {
            // D-Value
            let raw: i32 = xxxx.parse().unwrap();
            if raw > 5000 {
                // Negative D-value
                Some(ExtrapolatedSurfacePressure::DValue(DValue::with_meters(
                    -1 * (raw - 5000),
                )))
            } else {
                Some(ExtrapolatedSurfacePressure::DValue(DValue::with_meters(
                    raw,
                )))
            }
        } else {
            // Extrapolated surface pressure
            Some(ExtrapolatedSurfacePressure::ExtrapolatedPressure(
                parse_aircraft_pressure(xxxx),
            ))
        }
    }
}

#[test]
fn test_parse_extrapolated_sfc_pressure() {
    let alt = Pressure::with_microbars(923_000);
    let expected1 =
        ExtrapolatedSurfacePressure::ExtrapolatedPressure(Pressure::with_microbars(1_011_500));
    let attempt1 = parse_extrapolated_sfc_pressure(alt, "0115");
    assert_eq!(Some(expected1), attempt1)
}

fn parse_temperature(sttt: &str) -> Option<Temperature> {
    sttt.parse()
        .map(|mc: i32| Temperature::with_millicelsius(mc * 100))
        .ok()
}

fn parse_wind(www_sss: &str) -> Option<Wind> {
    www_sss
        .parse()
        .map(|raw: u32| {
            Wind::with_direction_and_speed(
                Direction::with_angle(Angle::with_degrees_minutes_seconds(raw / 1000, 0, 0)),
                Speed::with_knots(raw % 1000),
            )
        })
        .ok()
}

fn parse_speed(sss: &str) -> Option<Speed> {
    sss.parse().map(|knots| Speed::with_knots(knots)).ok()
}

fn parse_rain_rate(ppp: &str) -> Option<RainRate> {
    ppp.parse()
        .map(|mm_p_hr| RainRate::with_mm_per_hr(mm_p_hr))
        .ok()
}
