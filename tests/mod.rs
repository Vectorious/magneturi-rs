extern crate magneturi;

use magneturi::MagnetUri;

#[test]
fn test_mediawiki() {
    let uri = "magnet:?xt=urn:ed2k:354B15E68FB8F36D7CD88FF94116CDC1
&xt=urn:tree:tiger:7N5OAMRNGMSSEUE3ORHOKWN4WWIQ5X4EBOOTLJY
&xt=urn:btih:QHQXPYWMACKDWKP47RRVIV7VOURXFE5Q
&xl=10826029&dn=mediawiki-1.15.1.tar.gz
&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce
&as=http%3A%2F%2Fdownload.wikimedia.org%2Fmediawiki%2F1.15%2Fmediawiki-1.15.1.tar.gz
&xs=http%3A%2F%2Fcache.example.org%2FXRX2PEFXOOEJFRVUCX6HMZMKS5TWG4K5
&xs=dchub://example.org";

    let magnet = MagnetUri::parse(uri).ok().unwrap();
    let res = magnet.resources().ok().unwrap()[0].clone();

    assert_eq!(res.xt[0], "urn:ed2k:354B15E68FB8F36D7CD88FF94116CDC1");
    assert_eq!(res.xt[1], "urn:tree:tiger:7N5OAMRNGMSSEUE3ORHOKWN4WWIQ5X4EBOOTLJY");
    assert_eq!(res.xt[2], "urn:btih:QHQXPYWMACKDWKP47RRVIV7VOURXFE5Q");
    assert_eq!(res.xl, Some(10826029));
    assert_eq!(res.dn, Some("mediawiki-1.15.1.tar.gz".to_owned()));
    assert_eq!(res.tr[0], "udp://tracker.openbittorrent.com:80/announce");
    assert_eq!(res.as_[0], "http://download.wikimedia.org/mediawiki/1.15/mediawiki-1.15.1.tar.gz");
    assert_eq!(res.xs[0], "http://cache.example.org/XRX2PEFXOOEJFRVUCX6HMZMKS5TWG4K5");
    assert_eq!(res.xs[1], "dchub://example.org");
}

#[test]
fn test_https() {
    let uri = "https://www.google.com/";

    let magnet = MagnetUri::parse(uri);

    match magnet {
        Err(e) => assert_eq!(e, magneturi::Error::InvalidScheme("https".to_owned())),
        _      => panic!("Did not invalidate HTTPS URI.")
    }
}
