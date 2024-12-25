use std::io::Write;

use sequoia_openpgp::{policy::StandardPolicy, serialize::stream::{Armorer, Encryptor2, LiteralWriter, Message}, Cert};
use wasm_bindgen::prelude::*;
use sequoia_openpgp::parse::Parse;

#[wasm_bindgen]
pub fn encrypt_message(public_key: &str, msg_str: &str) -> Result<String, JsValue> {

  let cert = Cert::from_bytes(public_key).expect("Couldn't parse cert");
  let p = &StandardPolicy::new();
  let recipients =
      cert.keys().with_policy(p, None).supported().alive().revoked(false)
      // Or `for_storage_encryption()`, for data at rest.
      .for_transport_encryption();
  let mut sink = vec![];
  let message = Message::new(&mut sink);
  let message = Armorer::new(message).build().expect("wups");
  let message = Encryptor2::for_recipients(message, recipients).build().expect("encryptor 2");
  let mut w = LiteralWriter::new(message).build().expect("didn't build");
  let _ = w.write_all(msg_str.as_bytes());
  let _ = w.finalize();
  let result = String::from_utf8(sink).expect("error converting result to string");
  return Ok(result);
}
