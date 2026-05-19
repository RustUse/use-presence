use use_presence::prelude::{ReferrerKind, classify_referrer_host, classify_source_medium};

fn main() {
    let email = classify_source_medium(Some("newsletter"), Some("email"));
    let organic = classify_referrer_host("www.google.com");

    assert_eq!(email, ReferrerKind::Email);
    assert_eq!(organic, ReferrerKind::Organic);
}
