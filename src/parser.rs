use crate::constants::*;
use crfs::Attribute;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug};

#[derive(Default, Serialize, Deserialize)]
pub struct ParsedAddress {
    /// original: 'AddressNumberPrefix'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number_prefix: Option<String>,

    /// original: 'AddressNumber'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number: Option<String>,

    /// original: 'AddressNumberSuffix'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number_suffix: Option<String>,

    /// original: 'StreetNamePreModifier'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name_pre_modifier: Option<String>,

    /// original: 'StreetNamePreDirectional'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name_pre_directional: Option<String>,

    /// original: 'StreetNamePreType'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name_pre_type: Option<String>,

    /// original: 'StreetName'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,

    /// original: 'StreetNamePostType'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name_post_type: Option<String>,

    /// original: 'StreetNamePostDirectional'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name_post_directional: Option<String>,

    /// original: 'SubaddressType'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddress_type: Option<String>,

    /// original: 'SubaddressIdentifier'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddress_identifier: Option<String>,

    /// original: 'BuildingName'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_name: Option<String>,

    /// original: 'OccupancyType'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupancy_type: Option<String>,

    /// original: 'OccupancyIdentifier'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupancy_identifier: Option<String>,

    /// original: 'CornerOf'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_of: Option<String>,

    /// original: 'LandmarkName'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark_name: Option<String>,

    /// original: 'PlaceName'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_name: Option<String>,

    /// original: 'StateName'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,

    /// original: 'ZipCode'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,

    /// original: 'USPSBoxType'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usps_box_type: Option<String>,

    /// original: 'USPSBoxID'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usps_box_id: Option<String>,

    /// original: 'USPSBoxGroupType'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usps_box_group_type: Option<String>,

    /// original: 'USPSBoxGroupID'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usps_box_group_id: Option<String>,

    /// original: 'IntersectionSeparator'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_separator: Option<String>,

    /// original: 'Recipient'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,

    /// original: 'NotAddress'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_address: Option<String>,
}

impl Debug for ParsedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut root = f.debug_struct("ParsedAddress");
        if let Some(x) = &self.address_number_prefix {
            root.field("address_number_prefix", x);
        }
        if let Some(x) = &self.address_number_prefix {
            root.field("address_number_prefix", x);
        }
        if let Some(x) = &self.address_number {
            root.field("address_number", x);
        }
        if let Some(x) = &self.address_number_suffix {
            root.field("address_number_suffix", x);
        }
        if let Some(x) = &self.street_name_pre_modifier {
            root.field("street_name_pre_modifier", x);
        }
        if let Some(x) = &self.street_name_pre_directional {
            root.field("street_name_pre_directional", x);
        }
        if let Some(x) = &self.street_name_pre_type {
            root.field("street_name_pre_type", x);
        }
        if let Some(x) = &self.street_name {
            root.field("street_name", x);
        }
        if let Some(x) = &self.street_name_post_type {
            root.field("street_name_post_type", x);
        }
        if let Some(x) = &self.street_name_post_directional {
            root.field("street_name_post_directional", x);
        }
        if let Some(x) = &self.subaddress_type {
            root.field("subaddress_type", x);
        }
        if let Some(x) = &self.subaddress_identifier {
            root.field("subaddress_identifier", x);
        }
        if let Some(x) = &self.building_name {
            root.field("building_name", x);
        }
        if let Some(x) = &self.occupancy_type {
            root.field("occupancy_type", x);
        }
        if let Some(x) = &self.occupancy_identifier {
            root.field("occupancy_identifier", x);
        }
        if let Some(x) = &self.corner_of {
            root.field("corner_of", x);
        }
        if let Some(x) = &self.landmark_name {
            root.field("landmark_name", x);
        }
        if let Some(x) = &self.place_name {
            root.field("place_name", x);
        }
        if let Some(x) = &self.state_name {
            root.field("state_name", x);
        }
        if let Some(x) = &self.zip_code {
            root.field("zip_code", x);
        }
        if let Some(x) = &self.usps_box_type {
            root.field("usps_box_type", x);
        }
        if let Some(x) = &self.usps_box_id {
            root.field("usps_box_id", x);
        }
        if let Some(x) = &self.usps_box_group_type {
            root.field("usps_box_group_type", x);
        }
        if let Some(x) = &self.usps_box_group_id {
            root.field("usps_box_group_id", x);
        }
        if let Some(x) = &self.intersection_separator {
            root.field("intersection_separator", x);
        }
        if let Some(x) = &self.recipient {
            root.field("recipient", x);
        }
        if let Some(x) = &self.not_address {
            root.field("not_address", x);
        }

        root.finish()
    }
}
pub struct Parser<'a> {
    model: crfs::Model<'a>,
}

impl Parser<'_> {
    pub fn parse(&mut self, input: &str) -> Result<ParsedAddress, ()> {
        let mut tagger = self.model.tagger().unwrap();
        let tokens = tokenize(input);
        let features = tokens_to_features(&tokens);
        let result = tagger.tag(&features).unwrap();
        let mut addr = ParsedAddress::default();
        for (token, label) in tokens.iter().zip(result) {
            match label {
                "AddressNumberPrefix" => addr.address_number_prefix = Some(token.to_owned()),
                "AddressNumber" => addr.address_number = Some(token.to_owned()),
                "AddressNumberSuffix" => addr.address_number_suffix = Some(token.to_owned()),
                "StreetNamePreModifier" => addr.street_name_pre_modifier = Some(token.to_owned()),
                "StreetNamePreDirectional" => {
                    addr.street_name_pre_directional = Some(token.to_owned())
                }
                "StreetNamePreType" => addr.street_name_pre_type = Some(token.to_owned()),
                "StreetName" => addr.street_name = Some(token.to_owned()),
                "StreetNamePostType" => addr.street_name_post_type = Some(token.to_owned()),
                "StreetNamePostDirectional" => {
                    addr.street_name_post_directional = Some(token.to_owned())
                }
                "SubaddressType" => addr.subaddress_type = Some(token.to_owned()),
                "SubaddressIdentifier" => addr.subaddress_identifier = Some(token.to_owned()),
                "BuildingName" => addr.building_name = Some(token.to_owned()),
                "OccupancyType" => addr.occupancy_type = Some(token.to_owned()),
                "OccupancyIdentifier" => addr.occupancy_identifier = Some(token.to_owned()),
                "CornerOf" => addr.corner_of = Some(token.to_owned()),
                "LandmarkName" => addr.landmark_name = Some(token.to_owned()),
                "PlaceName" => addr.place_name = Some(token.to_owned()),
                "StateName" => addr.state_name = Some(token.to_owned()),
                "ZipCode" => addr.zip_code = Some(token.to_owned()),
                "USPSBoxType" => addr.usps_box_type = Some(token.to_owned()),
                "USPSBoxID" => addr.usps_box_id = Some(token.to_owned()),
                "USPSBoxGroupType" => addr.usps_box_group_type = Some(token.to_owned()),
                "USPSBoxGroupID" => addr.usps_box_group_id = Some(token.to_owned()),
                "IntersectionSeparator" => addr.intersection_separator = Some(token.to_owned()),
                "Recipient" => addr.recipient = Some(token.to_owned()),
                "NotAddress" => addr.not_address = Some(token.to_owned()),
                unknown => todo!("Uknown label: {unknown}"),
            }
        }
        Ok(addr)
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        let model = crfs::Model::new(include_bytes!("../usaddress/usaddr.crfsuite")).unwrap();
        Self { model }
    }
}

lazy_static! {
    static ref STREET_NAMES_LOOKUP: HashSet<&'static str> =
        HashSet::from_iter(STREET_NAMES.iter().copied());
    static ref DIRECTIONS_LOOKUP: HashSet<&'static str> =
        HashSet::from_iter(DIRECTIONS.iter().copied());
    static ref VOWELS_LOOKUP: HashSet<char> = HashSet::from_iter(VOWELS.iter().copied());
    static ref ENDS_IN_PUNC_REGEX: Regex = Regex::new(".+[^.\\w]").unwrap();
}

fn token_features(token: &str) -> Vec<Attribute> {
    let mut features = vec![];
    let abbrev = token.to_lowercase().replace(['.', ','], "");
    let dig = digit(abbrev.as_str());
    features.push(Attribute::new(
        "abbrev",
        token
            .chars()
            .last()
            .map_or(0.0, |c| if c == '.' { 1.0 } else { 0.0 }),
    ));
    features.push(match dig {
        DigitResult::All => Attribute::new("digits:all_digits", 1.0),
        DigitResult::Some => Attribute::new("digits:some_digits", 1.0),
        DigitResult::None => Attribute::new("digits:no_digits", 1.0),
    });
    if !matches!(dig, DigitResult::None) {
        features.push(Attribute::new("word", 0.0));
    } else {
        features.push(Attribute::new(format!("word:{abbrev}"), 1.0));
    }

    features.push(Attribute::new("trailing.zeros", 1.0));
    if matches!(dig, DigitResult::None) {
        features.push(Attribute::new(
            format!("length:w:{}", token.chars().count()),
            1.0,
        ));
    } else {
        features.push(Attribute::new(
            format!("length:d:{}", token.chars().count()),
            1.0,
        ));
    }
    if ENDS_IN_PUNC_REGEX.is_match(token) {
        features.push(Attribute::new(
            format!("endsinpunc:{}", token.chars().last().unwrap()),
            1.0,
        ));
    } else {
        features.push(Attribute::new("endsinpunc", 0.0));
    }
    features.push(Attribute::new(
        "directional",
        if DIRECTIONS_LOOKUP.contains(abbrev.as_str()) {
            1.0
        } else {
            0.0
        },
    ));
    features.push(Attribute::new(
        "street_name",
        if STREET_NAMES_LOOKUP.contains(abbrev.as_str()) {
            1.0
        } else {
            0.0
        },
    ));
    features.push(Attribute::new(
        "has.vowels",
        abbrev
            .chars()
            .find(|c| VOWELS_LOOKUP.contains(c))
            .map_or(0.0, |v| 1.0),
    ));
    for f in features.as_slice() {
        print!("{}={} ", f.name, f.value)
    }
    println!();
    features
}

enum DigitResult {
    All,
    Some,
    None,
}
fn digit(token: &str) -> DigitResult {
    let num_digit = token
        .chars()
        .map(|c| c.is_ascii_digit())
        .filter(|&v| v)
        .count();
    if num_digit == 0 {
        DigitResult::None
    } else if num_digit == token.chars().count() {
        DigitResult::All
    } else {
        DigitResult::Some
    }
}

fn windows_mut_each<T>(v: &mut [T], n: usize, mut f: impl FnMut(&mut [T])) {
    let mut start = 0;
    let mut end = n;
    while end <= v.len() {
        f(&mut v[start..end]);
        start += 1;
        end += 1;
    }
}

fn tokens_to_features(address_tokens: &Vec<String>) -> Vec<Vec<Attribute>> {
    let mut base: Vec<Vec<Attribute>> = address_tokens
        .iter()
        .map(|token| token_features(token))
        .collect();
    if let Some(first) = base.first_mut() {
        first.push(Attribute::new("address.start", 1.0));
    }
    if let Some(last) = base.last_mut() {
        last.push(Attribute::new("address.end", 1.0));
    }
    windows_mut_each(&mut base, 2, |w| {
        let mut i = w.iter_mut();
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        for attr in a.clone() {
            b.push(Attribute::new(
                format!("previous:{}", attr.name),
                attr.value,
            ))
        }
        for attr in b.clone() {
            a.push(Attribute::new(format!("next:{}", attr.name), attr.value))
        }
    });
    base
}
fn tokenize(address_string: &str) -> Vec<String> {
    let address_string = Regex::new(r"(&#38;)|(&amp;)")
        .unwrap()
        .replace_all(address_string, "&")
        .to_string();

    let re_tokens = Regex::new(r"\(*\b[^\s,;#&()]+[.,;)\n]*|[#&]").unwrap();

    re_tokens
        .find_iter(&address_string)
        .map(|m| m.as_str().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! svec {
      ($($x:expr),*) => (vec![$($x.to_string()),*]);
  }

    #[test]
    fn tokenizer() {
        assert_eq!(
            tokenize("123 White Road, Whitter CA"),
            svec!["123", "White", "Road,", "Whitter", "CA"]
        );
    }
    #[test]
    fn tokenizer_py_tests() {
        // https://github.com/datamade/usaddress/blob/master/tests/test_tokenizing.py
        assert_eq!(tokenize("# 1 abc st"), svec!["#", "1", "abc", "st"]);
        assert_eq!(tokenize("#1 abc st"), svec!["#", "1", "abc", "st"]);
        assert_eq!(
            tokenize("box # 1 abc st"),
            svec!["box", "#", "1", "abc", "st"]
        );
        assert_eq!(
            tokenize("box #1 abc st"),
            svec!["box", "#", "1", "abc", "st"]
        );
        assert_eq!(
            tokenize("box# 1 abc st"),
            svec!["box", "#", "1", "abc", "st"],
        );
        assert_eq!(
            tokenize("box#1 abc st"),
            svec!["box", "#", "1", "abc", "st"]
        );
        assert_eq!(
            tokenize("1 abc st,suite 1"),
            svec!["1", "abc", "st,", "suite", "1"]
        );
        assert_eq!(
            tokenize("1 abc st;suite 1"),
            svec!["1", "abc", "st;", "suite", "1"]
        );
        assert_eq!(tokenize("1-5 abc road"), svec!["1-5", "abc", "road"],);
        assert_eq!(tokenize("1 abc st"), svec!["1", "abc", "st"]);
        assert_eq!(tokenize("1  abc st"), svec!["1", "abc", "st"],);
        assert_eq!(tokenize("1 abc st "), svec!["1", "abc", "st"]);
        assert_eq!(tokenize(" 1 abc st"), svec!["1", "abc", "st"],);
        assert_eq!(
            tokenize("222 W. Merchandise Mart Plaza"),
            svec!["222", "W.", "Merchandise", "Mart", "Plaza"],
        );
        assert_eq!(
            tokenize("222 W Merchandise Mart Plaza, Chicago, IL"),
            svec![
                "222",
                "W",
                "Merchandise",
                "Mart",
                "Plaza,",
                "Chicago,",
                "IL"
            ],
        );
        assert_eq!(tokenize("123 Monroe- St"), svec!["123", "Monroe-", "St"]);
        assert_eq!(
            tokenize("222 W Merchandise Mart Plaza Chicago IL 60654"),
            svec![
                "222",
                "W",
                "Merchandise",
                "Mart",
                "Plaza",
                "Chicago",
                "IL",
                "60654"
            ]
        );
        assert_eq!(tokenize("123 & 456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123&456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123& 456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123 &456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123 &#38; 456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123&#38;456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123&#38; 456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123 &#38;456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123 &amp; 456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123&amp;456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123&amp; 456"), svec!["123", "&", "456"]);
        assert_eq!(tokenize("123 &amp;456"), svec!["123", "&", "456"]);
        assert_eq!(
            tokenize("222 W Merchandise Mart Plaza (1871) Chicago IL 60654"),
            svec![
                "222",
                "W",
                "Merchandise",
                "Mart",
                "Plaza",
                "(1871)",
                "Chicago",
                "IL",
                "60654"
            ],
        );
        assert_eq!(
            tokenize("222 W Merchandise Mart Plaza (1871), Chicago IL 60654"),
            svec![
                "222",
                "W",
                "Merchandise",
                "Mart",
                "Plaza",
                "(1871),",
                "Chicago",
                "IL",
                "60654"
            ]
        );
        assert_eq!(
            tokenize("222 W Merchandise Mart Plaza(1871) Chicago IL 60654"),
            svec![
                "222",
                "W",
                "Merchandise",
                "Mart",
                "Plaza",
                "(1871)",
                "Chicago",
                "IL",
                "60654"
            ]
        );
    }
}
