extern crate html5ever;
extern crate hyper;
extern crate hyper_native_tls;
extern crate url;

use std::default::Default;
use std::io;

use html5ever::{parse_document, serialize};
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

use hyper::client::Client;
use hyper::header::ContentType;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use url::form_urlencoded;

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let http_client = Client::with_connector(HttpsConnector::new(ssl));

    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("contractCode", "NRSO")
        .append_pair("parkId", "70928")
        .append_pair("siteTypeFilter", "ALL")
        .append_pair("availStatus", "ONLINE")
        .append_pair("submitSiteForm", "true")
        .append_pair("search", "site")
        .append_pair("arrivalDate", "Tue Sep 12 2017")
        .append_pair("departureDate", "Wed Sep 13 2017")
        .append_pair("flexDates", "")
        .append_pair("loop", "")
        .append_pair("siteCode", "")
        .append_pair("lookingFor", "")
        .append_pair("camping_common_218", "")
        .append_pair("camping_common_3012", "")
        .append_pair("camping_common_3013", "")
        .finish();

    let mut response = http_client.post("https://www.recreation.gov/campsiteSearch.do")
        .header(ContentType::form_url_encoded())
        .body(&body)
        .send()
        .unwrap();

    let dom: RcDom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut response)
        .unwrap();

    // TODO(John Sirois): index into the dom to check for available sites.
    serialize(&mut io::stdout(), &dom.document, Default::default())
        .ok()
        .expect("Failed to serialize post body to stdout!")
}
