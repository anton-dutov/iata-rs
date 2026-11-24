#![cfg(feature = "with-bcbp")]

use iata::bcbp::{format::Field, view::BcbpView, Bcbp, BcbpResult, Error, Leg, Version};

#[test]
fn fast_track_checks_every_leg_at_encoding_time() -> BcbpResult<()> {
    let mut pass = Bcbp::default();
    pass.set_version(Some(Version::Value(4)))?;
    pass.add_leg(Leg::default())?;
    let mut second = Leg::default();
    second.set_fast_track(Some('N'))?;
    pass.add_leg(second)?;

    assert_eq!(
        pass.encode(),
        Err(Error::FieldRequiresVersion {
            field: Field::FastTrack,
            version: 4,
            minimum: 5,
        })
    );
    pass.set_version(Some(Version::Value(5)))?;
    let encoded = pass.encode()?;
    let view = BcbpView::parse(&encoded)?;
    assert_eq!(view.version(), Some('5'));
    assert_eq!(view.legs()[1].fast_track(), Some('N'));

    let legacy = encoded.replacen(">5", ">2", 1);
    let view = BcbpView::parse(&legacy)?;
    assert_eq!(view.legs()[1].fast_track(), Some('N'));
    let mut decoded = Bcbp::try_from(view)?;
    assert_eq!(
        decoded.encode_bcbp(),
        Err(Error::FieldRequiresVersion {
            field: Field::FastTrack,
            version: 2,
            minimum: 5,
        })
    );
    decoded.set_version(None)?;
    assert_eq!(BcbpView::parse(&decoded.encode()?)?.version(), Some('6'));
    Ok(())
}

#[test]
fn nonconsecutive_tags_require_version_four() -> BcbpResult<()> {
    for field in [
        Field::FirstNonConsecutiveBaggageTagNumbers,
        Field::SecondNonConsecutiveBaggageTagNumbers,
    ] {
        let mut pass = Bcbp::default();
        pass.add_leg(Leg::default())?;
        let tag = Some("0123456789001");
        if field == Field::FirstNonConsecutiveBaggageTagNumbers {
            pass.set_nonconsecutive_bagtag1(tag)?;
        } else {
            pass.set_nonconsecutive_bagtag2(tag)?;
        }
        pass.set_version(Some(Version::Value(3)))?;
        assert_eq!(
            pass.encode(),
            Err(Error::FieldRequiresVersion {
                field,
                version: 3,
                minimum: 4
            })
        );
        pass.set_version(Some(Version::Value(4)))?;
        let decoded: Bcbp = pass.encode()?.parse()?;
        assert_eq!(decoded.version(), Some(Version::Value(4)));
        assert_eq!(
            decoded.nonconsecutive_bagtag1(),
            pass.nonconsecutive_bagtag1()
        );
        assert_eq!(
            decoded.nonconsecutive_bagtag2(),
            pass.nonconsecutive_bagtag2()
        );
    }
    Ok(())
}

#[test]
fn security_data_requires_version_three() -> BcbpResult<()> {
    let mut pass: Bcbp =
        "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 226F001A0025 100^100".parse()?;
    pass.set_version(Some(Version::Value(2)))?;
    assert_eq!(
        pass.encode(),
        Err(Error::FieldRequiresVersion {
            field: Field::SecurityData,
            version: 2,
            minimum: 3,
        })
    );
    pass.set_version(Some(Version::Value(3)))?;
    let encoded = pass.encode()?;
    assert!(encoded.ends_with("^100"));
    assert_eq!(BcbpView::parse(&encoded)?.version(), Some('3'));
    Ok(())
}

#[test]
fn clearing_the_only_version_field_restores_the_minimal_pass() -> BcbpResult<()> {
    let mut pass = Bcbp::default();
    pass.add_leg(Leg::default())?;
    let minimal = pass.encode()?;
    assert_eq!(minimal.len(), 60);
    assert_eq!(BcbpView::parse(&minimal)?.version(), None);
    pass.set_version(None)?;
    assert_eq!(pass.encode()?, minimal);
    pass.set_version(Some(Version::Value(4)))?;
    assert_eq!(BcbpView::parse(&pass.encode()?)?.version(), Some('4'));
    pass.set_version(None)?;
    assert_eq!(pass.encode()?, minimal);
    Ok(())
}

#[test]
fn unspecified_version_uses_latest_even_for_older_fields() -> BcbpResult<()> {
    let mut pass = Bcbp::default();
    pass.add_leg(Leg::default())?;
    pass.set_doc_type(Some('B'))?;
    assert_eq!(BcbpView::parse(&pass.encode()?)?.version(), Some('6'));
    pass.set_version(Some(Version::Value(2)))?;
    assert_eq!(BcbpView::parse(&pass.encode()?)?.version(), Some('2'));
    pass.set_version(None)?;
    let encoded = pass.encode()?;
    let view = BcbpView::parse(&encoded)?;
    assert_eq!(view.version(), Some('6'));
    assert_eq!(view.doc_type(), Some('B'));
    Ok(())
}

#[test]
fn version_char_conversion_checks_numeric_width() {
    for digit in 0..=9 {
        assert_eq!(
            char::try_from(Version::Value(digit)).unwrap(),
            char::from(b'0' + digit)
        );
    }
    for value in 10..=u8::MAX {
        assert_eq!(
            char::try_from(Version::Value(value)),
            Err(Error::InvalidFieldFormat {
                field: Field::VersionNumber
            })
        );
    }
    assert_eq!(char::try_from(Version::Other(b'X')).unwrap(), 'X');
}

#[test]
fn unknown_version_can_be_read_but_cannot_be_emitted() -> BcbpResult<()> {
    let mut pass: Bcbp =
        "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 226F001A0025 106>90000".parse()?;
    assert_eq!(pass.version(), Some(Version::Value(9)));
    assert_eq!(pass.encode(), Err(Error::UnsupportedEncodingVersion(9)));
    pass.set_version(Some(Version::Value(u8::MAX)))?;
    assert_eq!(
        pass.encode(),
        Err(Error::UnsupportedEncodingVersion(u8::MAX))
    );
    Ok(())
}
