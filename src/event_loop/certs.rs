use rustls::Certificate;
use std::fs::File;
use std::io::BufReader;
use std::{env, io};

const ENV_CERT_FILE: &str = "SSL_CERT_FILE";

pub fn load_certs_from_env() -> io::Result<Vec<Certificate>> {
    let file_name = match env::var(ENV_CERT_FILE) {
        Ok(val) => val,
        Err(_) => {
            return io::Result::Err(io::Error::from(io::ErrorKind::NotFound));
        }
    };
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader)?;
    Ok(certs.into_iter().map(Certificate).collect())
}
