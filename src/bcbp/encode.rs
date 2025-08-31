use crate::bcbp::{Bcbp, BcbpResult, Error};
use std::fmt::Write as _;

// const MAX_LEGS: usize = 9;
const BLANK3: &str = "   ";
const BLANK4: &str = "    ";

pub fn encode_bcbp(bcbp: &Bcbp) -> BcbpResult<String> {
    let legs_count = bcbp.legs_count();

    if !(1..9).contains(&legs_count) {
        return Err(Error::InvalidLegsCount);
    }

    let mut mandatory = String::with_capacity(60);

    write!(
        mandatory,
        "M{}{:<20}{}",
        legs_count,
        bcbp.passenger_name,
        bcbp.eticket_indicator.unwrap_or(' ')
    )?;

    let mut legs = bcbp.legs.iter();

    let first_leg = legs.next().unwrap();

    mandatory.push_str(&encode_leg_mandatory_data(first_leg)?);

    let mut ext = String::with_capacity(4096);

    if bcbp.is_extednded() {
        let cond_data = encode_cond_data(bcbp)?;
        let leg_cond_data = encode_leg_cond_data(first_leg)?;

        write!(
            mandatory,
            "{:02X}",
            cond_data.len() + leg_cond_data.len() + 2 + 2
        )?;
        ext.push('>');
        ext.push(bcbp.version().map(char::from).unwrap_or(' '));
        write!(ext, "{:02X}", cond_data.len())?;
        ext.push_str(&cond_data);
        ext.push_str(&leg_cond_data);
    } else {
        mandatory.push_str("00");
    }

    for leg in legs {
        let leg_data = encode_leg_mandatory_data(leg)?;
        let leg_cond_data = encode_leg_cond_data(leg)?;

        ext.push_str(&leg_data);
        write!(ext, "{:02X}", leg_cond_data.len())?;
        ext.push_str(&leg_cond_data);
    }

    let mut data = String::with_capacity(4096);
    data.push_str(&mandatory);

    if !ext.is_empty() {
        data.push_str(&ext);
    }

    Ok(data)
}

fn encode_cond_data(bcbp: &Bcbp) -> BcbpResult<String> {
    let mut buf = String::with_capacity(512);

    buf.push(bcbp.pax_kind().map(char::from).unwrap_or(' '));
    buf.push(bcbp.checkin_src().unwrap_or(' '));
    buf.push(bcbp.boardingpass_src().unwrap_or(' '));
    write!(buf, "{:03}", bcbp.boardingpass_issued().unwrap_or(0))?;
    buf.push(bcbp.doc_type().unwrap_or(' '));
    buf.push_str(bcbp.boardingpass_airline().as_deref().unwrap_or(BLANK3));
    buf.push_str(&" ".repeat(14));

    Ok(buf)
}

fn encode_leg_mandatory_data(leg: &crate::bcbp::Leg) -> BcbpResult<String> {
    let mut buf = String::with_capacity(512);

    let pnr = leg.pnr().unwrap_or_default();
    let src = leg.src_airport().unwrap_or_default();
    let dst = leg.dst_airport().unwrap_or_default();
    let operating_carrier = leg.operating_carrier().unwrap_or_default();
    let flight = leg.flight_number().unwrap_or_default();

    // Write fixed fields in a batch
    write!(
        buf,
        "{:<7}{:<3}{:<3}{:<3}{:<5}",
        pnr, src, dst, operating_carrier, flight
    )?;

    // Flight day (DDD) or spaces
    match leg.flight_day() {
        Some(day) => write!(buf, "{:03}", day.ordinal())?,
        None => buf.push_str(BLANK3),
    }

    // Compartment (1 character)
    buf.push(leg.compartment().unwrap_or(' '));

    // Seat (4 characters): if starts with digit — zero-pad right to 4, otherwise left-pad with spaces
    match leg.seat().as_deref() {
        Some(s)
            if s.as_bytes()
                .first()
                .map(|b| b.is_ascii_digit())
                .unwrap_or(false) =>
        {
            write!(buf, "{:0>4}", s)?;
        }
        Some(s) => {
            write!(buf, "{:<4}", s)?;
        }
        None => buf.push_str(BLANK4),
    }

    // Sequence (4 characters, zero-padded) or spaces
    if let Some(seq) = leg.checkin_sequence() {
        write!(buf, "{:0>4}", seq).unwrap();
    } else {
        buf.push_str(BLANK4);
    }
    // Blank afger sequence
    buf.push(' ');

    // Pax status (1 character)
    buf.push(char::from(leg.pax_status()));

    Ok(buf)
}

fn encode_leg_cond_data(leg: &crate::bcbp::Leg) -> BcbpResult<String> {
    let mut buf = String::with_capacity(512);

    if !leg.has_conditional_data() {
        return Ok(buf);
    };

    match leg.airline_num() {
        Some(n) => write!(buf, "{n:03}")?,
        None => buf.push_str(BLANK3),
    }

    write!(buf, "{:<10}", leg.doc_number().unwrap_or_default())?;

    buf.push(leg.selectee_indicator().unwrap_or(' '));
    buf.push(leg.doc_int_verification().unwrap_or(' '));

    write!(buf, "{:<3}", leg.marketing_airline().unwrap_or_default())?;
    write!(buf, "{:<3}", leg.freq_flyer_airline().unwrap_or_default())?;
    write!(buf, "{:<16}", leg.freq_flyer_number().unwrap_or_default())?;

    buf.push(leg.id_ad_indicator().unwrap_or(' '));

    write!(buf, "{:<3}", leg.bag_allowance().unwrap_or_default())?;

    buf.push(leg.fast_track().unwrap_or(' '));
    buf.push_str(&leg.variable_data().as_deref().unwrap_or_default());

    let mut rec = String::with_capacity(buf.len() + 2);

    write!(rec, "{:02X}", buf.len() - 2)?;

    rec.push_str(&buf);

    Ok(rec)
}
