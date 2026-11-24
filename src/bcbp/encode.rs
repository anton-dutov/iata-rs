use crate::bcbp::{format::Field, Bcbp, BcbpResult, Error, Leg, Version};
use std::fmt::Write as _;

const BLANK3: &str = "   ";
const BLANK4: &str = "    ";

pub fn encode_bcbp(bcbp: &Bcbp) -> BcbpResult<String> {
    let legs_count = bcbp.legs_count();

    if !(1..=9).contains(&legs_count) {
        return Err(Error::InvalidNumberOfLegs);
    }

    let version = bcbp
        .version()
        .unwrap_or(Version::LATEST)
        .encoding_number()?;
    validate_versioned_fields(bcbp, version)?;

    let mut data = String::with_capacity(60);
    write!(
        data,
        "M{}{:<20}{}",
        legs_count,
        bcbp.passenger_name,
        bcbp.eticket_indicator.unwrap_or(' ')
    )?;

    for (index, leg) in bcbp.legs().enumerate() {
        write_leg_mandatory_data(&mut data, leg)?;

        let mut section = String::new();
        let first_extended = index == 0 && bcbp.is_extednded();
        if first_extended {
            section.push('>');
            section.push(char::from(b'0' + version));
            let unique = encode_cond_data(bcbp)?;
            write_section(
                &mut section,
                Field::FieldSizeOfStructuredMessageUnique,
                &unique,
            )?;
        }
        if first_extended || leg.has_conditional_data() {
            let repeated = encode_leg_cond_data(leg)?;
            write_section(
                &mut section,
                Field::FieldSizeOfStructuredMessageRepeated,
                &repeated,
            )?;
            section.push_str(leg.variable_data().unwrap_or_default());
        }
        write_section(&mut data, Field::FieldSizeOfVariableSizeField, &section)?;
    }

    if let Some(sd) = bcbp.security_data() {
        data.push('^');
        data.push(sd.kind);
        write_section(&mut data, Field::LengthOfSecurityData, &sd.data)?;
    }
    Ok(data)
}

fn validate_versioned_fields(pass: &Bcbp, version: u8) -> BcbpResult<()> {
    use Field::*;

    // Resolution 792, Implementation Guide §2.2.3 (Version management).
    for (field, minimum, present) in [
        (SecurityData, 3, pass.security_data().is_some()),
        (
            FirstNonConsecutiveBaggageTagNumbers,
            4,
            pass.nonconsecutive_bagtag1().is_some(),
        ),
        (
            SecondNonConsecutiveBaggageTagNumbers,
            4,
            pass.nonconsecutive_bagtag2().is_some(),
        ),
        (
            FastTrack,
            5,
            pass.legs().any(|leg| leg.fast_track().is_some()),
        ),
    ] {
        if present && version < minimum {
            return Err(Error::FieldRequiresVersion {
                field,
                version,
                minimum,
            });
        }
    }
    Ok(())
}

fn write_section(dst: &mut String, field: Field, body: &str) -> BcbpResult<()> {
    let len = u8::try_from(body.len()).map_err(|_| Error::FieldLengthExceeded {
        field,
        max: u8::MAX.into(),
    })?;
    write!(dst, "{len:02X}{body}")?;
    Ok(())
}

fn encode_cond_data(bcbp: &Bcbp) -> BcbpResult<String> {
    if !bcbp.has_unique_fields() {
        return Ok(String::new());
    }
    let mut buf = String::with_capacity(50);

    buf.push(char::try_from(
        bcbp.passenger_description().unwrap_or_default(),
    )?);
    buf.push(bcbp.checkin_src().unwrap_or(' '));
    buf.push(bcbp.boardingpass_src().unwrap_or(' '));
    match bcbp.boardingpass_issued() {
        Some(val) => write!(buf, "{val:04}")?,
        None => buf.push_str(BLANK4),
    }
    buf.push(bcbp.doc_type().unwrap_or(' '));
    match bcbp.boardingpass_airline() {
        Some(val) => write!(buf, "{val:<3}")?,
        None => buf.push_str(BLANK3),
    }

    write!(buf, "{:<13}", bcbp.bagtags().unwrap_or_default())?;
    if bcbp.nonconsecutive_bagtag1().is_some() || bcbp.nonconsecutive_bagtag2().is_some() {
        write!(
            buf,
            "{:<width$}",
            bcbp.nonconsecutive_bagtag1().unwrap_or_default(),
            width = Field::FirstNonConsecutiveBaggageTagNumbers.len()
        )?;
    }
    if let Some(tag) = bcbp.nonconsecutive_bagtag2() {
        write!(
            buf,
            "{tag:<width$}",
            width = Field::SecondNonConsecutiveBaggageTagNumbers.len()
        )?;
    }

    Ok(buf)
}

fn write_leg_mandatory_data(buf: &mut String, leg: &Leg) -> BcbpResult<()> {
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
    match leg.seat() {
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

    match leg.checkin_sequence() {
        Some(seq) if seq <= 9999 => write!(buf, "{seq:04} ")?,
        Some(_) => return Err(Error::InvalidCheckInSequence),
        None => buf.push_str("     "),
    }

    // Passenger status (1 character)
    buf.push(char::from(leg.passenger_status()));

    Ok(())
}

fn encode_leg_cond_data(leg: &Leg) -> BcbpResult<String> {
    if !leg.has_repeated_fields() {
        return Ok(String::new());
    }
    let mut buf = String::with_capacity(42);

    match leg.airline_num() {
        Some(n) => write!(buf, "{n:03}")?,
        None => buf.push_str(BLANK3),
    }

    write!(buf, "{:<10}", leg.doc_number().unwrap_or_default())?;

    buf.push(leg.selectee_indicator().unwrap_or(' '));
    buf.push(leg.doc_intl_verification().unwrap_or(' '));

    write!(buf, "{:<3}", leg.marketing_carrier().unwrap_or_default())?;
    write!(buf, "{:<3}", leg.freq_flyer_airline().unwrap_or_default())?;
    write!(buf, "{:<16}", leg.freq_flyer_number().unwrap_or_default())?;

    buf.push(leg.id_ad_indicator().unwrap_or(' '));

    write!(buf, "{:<3}", leg.baggage_allowance().unwrap_or_default())?;

    if let Some(fast_track) = leg.fast_track() {
        buf.push(fast_track);
    }

    Ok(buf)
}
