extern crate url;

use url::Url;

pub struct MagnetUri {
    pairs: Vec<(String, String)>
}

impl MagnetUri {
    pub fn parse(magnet: &str) -> Result<MagnetUri, Error> {
        let uri = match Url::parse(magnet) {
            Ok(p)  => p,
            Err(e) => return Err(Error::ParseError(e))
        };

        if uri.scheme != "magnet" {
            return Err(Error::InvalidScheme(uri.scheme));
        }
        
        match uri.query_pairs() {
            Some(p) => Ok(MagnetUri { pairs: p }),
            None    => Err(Error::EmptyQuery)
        }
    }

    pub fn resources(&self) -> Result<Vec<Resource>, Error> {
        enum Bool3 { True, False, Unknown }

        let mut res_vec: Vec<Resource> = Vec::new();

        let mut is_multi = Bool3::Unknown;

        for &(ref param, ref value) in self.pairs.iter() {
            let parameter = try!(Parameter::parse(param));

            let group = match parameter.group {
                Some(n) => {
                    match is_multi {
                        Bool3::Unknown => is_multi = Bool3::True,
                        Bool3::True    => (),
                        Bool3::False   => return Err(Error::InconsistentGrouping)
                    }

                    n
                }
                None    => {
                    match is_multi {
                        Bool3::Unknown => is_multi = Bool3::False,
                        Bool3::True    => return Err(Error::InconsistentGrouping),
                        Bool3::False   => ()
                    }

                    0
                }
            };

            // Resize resource vec to group number so we can add current resource.
            while res_vec.len() <= group {
                res_vec.push(Resource::default());
            }

            let res = res_vec.get_mut(group).unwrap();

            let val = value.to_owned();

            match parameter.param_type {
                ParameterType::DN   => res.name = Some(val),
                ParameterType::XL   => res.size = Some(val.parse().unwrap()),
                ParameterType::XT   => res.hashes.push(val),
                ParameterType::AS   => res.web_sources.push(val),
                ParameterType::XS   => res.p2p_sources.push(val),
                ParameterType::KT   => res.keywords.push(val),
                ParameterType::MT   => res.manifests.push(val),
                ParameterType::TR   => res.trackers.push(val),
                ParameterType::X(s) => res.supplement.push((s, val))
            }
        }

        Ok(res_vec)
    }
}

struct Parameter {
    pub param_type: ParameterType,
    pub group: Option<usize>
}

impl Parameter {
    pub fn parse(s: &str) -> Result<Parameter, Error> {
        let mut sections = s.split('.');

        let param_type = match sections.next().unwrap() {
            "dn" => ParameterType::DN,
            "xl" => ParameterType::XL,
            "xt" => ParameterType::XT,
            "as" => ParameterType::AS,
            "xs" => ParameterType::XS,
            "kt" => ParameterType::KT,
            "mt" => ParameterType::MT,
            "tr" => ParameterType::TR,
            "x"  => ParameterType::X(sections.next().unwrap().to_owned()),
            _    => return Err(Error::InvalidParameter(s.to_owned()))
        };

        let group: Option<usize> = match sections.next() {
            Some(n) => Some(n.parse().unwrap()),
            None    => None
        };

        Ok(Parameter { param_type: param_type, group: group })
    }
}

enum ParameterType {
    DN,
    XL,
    XT,
    AS,
    XS,
    KT,
    MT,
    TR,
    X(String),
}

#[derive(Clone, Default, Debug)]
pub struct Resource {
    pub name: Option<String>,               // Filename
    pub size: Option<usize>,                // Size in bytes
    pub hashes: Vec<String>,                // URN containing hash
    pub web_sources: Vec<String>,           // Acceptable source: Web link
    pub p2p_sources: Vec<String>,           // Exact source: P2P link
    pub keywords: Vec<String>,              // Keywords
    pub manifests: Vec<String>,             // Link to metafile: http://rakjar.de/gnuticles/MAGMA-Specsv22.txt
    pub trackers: Vec<String>,              // Tracker URL for BitTorrent
    pub supplement: Vec<(String, String)>   // Alternative parameters
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(url::ParseError),
    InvalidScheme(String),
    InvalidParameter(String),
    InvalidStructure,
    InconsistentGrouping,
    EmptyQuery,
}
