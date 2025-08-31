#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
/// BCBP Item 117 — Passenger Status (1 byte).
///
/// From the table (image):
/// 0 — ticket issuance / passenger **not checked in**
/// 1 — ticket issuance / passenger **checked in**
/// 2 — **baggage checked**, passenger not checked in
/// 3 — **baggage checked**, passenger checked in
/// 4 — passenger **passed security check**
/// 5 — passenger **passed gate exit** (coupon used)
/// 6 — **transit**
/// 7 — **standby** (seat number not printed at check-in; will be printed at seat assignment)
/// 8 — **boarding data revalidation done** (gate/boarding time/seat already used on revalidation field)
/// 9 — **original boarding data used** at time of ticket issuance
/// A — **up-/down-grading required** at close-out (e.g. waitlisted in C, OK in Y)
/// B–Z — **reserved for future industry use**
pub enum PaxStatus {
    /// Space (rare/unspecified).
    None, // ' '
    #[default]
    NotCheckedIn, // '0'
    CheckedIn,                // '1'
    BaggageCheckedNotCI,      // '2'
    BaggageAndPaxCheckedIn,   // '3'
    PaxPassedSecurity,        // '4'
    PassedGateExitCouponUsed, // '5'
    Transit,                  // '6'
    Standby,                  // '7' (seat not printed at check-in)
    RevalidationDone,         // '8'
    OriginalBoardingDataUsed, // '9'
    UpDownGradingRequired,    // 'A'
    /// Reserved letters 'B'..='Z'.
    Reserved(char),
    /// Any other ASCII character.
    Other(char),
}

impl From<char> for PaxStatus {
    #[inline]
    fn from(value: char) -> Self {
        use PaxStatus::*;
        let v = if value.is_ascii_lowercase() {
            value.to_ascii_uppercase()
        } else {
            value
        };
        match v {
            ' ' => None,
            '0' => NotCheckedIn,
            '1' => CheckedIn,
            '2' => BaggageCheckedNotCI,
            '3' => BaggageAndPaxCheckedIn,
            '4' => PaxPassedSecurity,
            '5' => PassedGateExitCouponUsed,
            '6' => Transit,
            '7' => Standby,
            '8' => RevalidationDone,
            '9' => OriginalBoardingDataUsed,
            'A' => UpDownGradingRequired,
            'B'..='Z' => Reserved(v),
            other => Other(other),
        }
    }
}

impl From<PaxStatus> for char {
    #[inline]
    fn from(s: PaxStatus) -> Self {
        use PaxStatus::*;
        match s {
            None => ' ',
            NotCheckedIn => '0',
            CheckedIn => '1',
            BaggageCheckedNotCI => '2',
            BaggageAndPaxCheckedIn => '3',
            PassedSecurity => '4',
            PassedGateExitCouponUsed => '5',
            Transit => '6',
            Standby => '7',
            RevalidationDone => '8',
            OriginalBoardingDataUsed => '9',
            UpDownGradingRequired => 'A',
            Reserved(ch) | Other(ch) => ch,
        }
    }
}
