extern crate chrono;
extern crate mbedtls;

use chrono::prelude::*;

use mbedtls::hash::Type::Sha256;
use mbedtls::pk::Pk;
use mbedtls::rng::{Rdrand};
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context};
use mbedtls::x509::certificate::{Builder, Certificate};
use mbedtls::x509::Time;
use mbedtls::Result as TlsResult;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread};
use std::sync::{Arc, Mutex};

const RSA_KEY_SIZE: u32 = 3072;
const RSA_KEY_EXP: u32 = 0x10001;
const DAYS_TO_SES: u64 = 86400;
const CERT_VAL_SECS: u64 = 365 * DAYS_TO_SES;
const THREADS: usize = 5;

trait ToTime {
    fn to_time(&self) -> Time;
}

impl ToTime for chrono::DateTime<Utc> {
    fn to_time(&self) -> Time {
        Time::new(
            self.year() as _,
            self.month() as _,
            self.day() as _,
            self.hour() as _,
            self.minute() as _,
            self.second() as _,
        )
        .unwrap()
    }
}

fn get_validity() -> (Time, Time) {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let end = start + CERT_VAL_SECS;
    let not_before = Utc.timestamp(start as _, 0);
    let not_after = Utc.timestamp(end as _, 0);
    (not_before.to_time(), not_after.to_time())
}

/// The below code generates a key and a self signed certificate
/// to configure the TLS context.
/// SGX applications should not rely on untrusted sources for their key.
/// Ideally, enclaves communicating via TLS should, ideally,
/// also verify attestation information, 
/// along with traditional certificate verification.
/// But this example doesn't show that.
fn get_key_and_cert() -> (Pk, Certificate) {
    let mut rng = Rdrand;
    let mut key = Pk::generate_rsa(&mut rng, RSA_KEY_SIZE, RSA_KEY_EXP).unwrap();
    let mut key_i = Pk::generate_rsa(&mut rng, RSA_KEY_SIZE, RSA_KEY_EXP).unwrap();
    let (not_before, not_after) = get_validity();

    let cert = Certificate::from_der(
        &Builder::new()
            .subject_key(&mut key)
            .subject_with_nul("CN=mbedtls-server.example\0")
            .unwrap()
            .issuer_key(&mut key_i)
            .issuer_with_nul("CN=mbedtls-server.example\0")
            .unwrap()
            .validity(not_before, not_after)
            .unwrap()
            .serial(&[5])
            .unwrap()
            .signature_hash(Sha256)
            .write_der_vec(&mut rng)
            .unwrap(),
    )
    .unwrap();
    (key, cert)
}

/// Establish a TLS connection with a randomly generated key and
/// a self signed certificate.
/// After a session is established, echo the incoming stream to the client 
/// till EOF is detected.
fn serve(mut conn: TcpStream, key: &mut Pk, cert: &mut Certificate) -> TlsResult<()> {
    let mut rng = Rdrand;

    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    config.set_rng(Some(&mut rng));
    config.push_cert(&mut **cert, key)?;
    let mut ctx = Context::new(&config)?;

    let mut buf = String::new();
    let session = ctx.establish(&mut conn, None)?;
    println!("Connection established!");
    let mut reader = BufReader::new(session);
    while let Ok(1..=std::usize::MAX) = reader.read_line(&mut buf) {
        let session = reader.get_mut();
        session.write_all(buf.as_bytes()).unwrap();
        buf.clear();
    }
    Ok(())
}

/// main() starts a TLS echo server on `localhost:7878`.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let arc_connection_amount: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let mut connection_amount = arc_connection_amount.lock().unwrap();

        if *connection_amount < (THREADS - 1) {
            *connection_amount += 1;

            thread::spawn(move || {
                //TO DO: Use the same certificate with each call
                let (mut key, mut cert) = get_key_and_cert();
                serve(stream, &mut key, &mut cert).unwrap();
                println!("Connection closed!");
            });
        }else{
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
    }
}
