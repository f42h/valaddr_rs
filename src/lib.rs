use std::{net::{IpAddr, ToSocketAddrs}};

pub struct ValAddr;
impl ValAddr {
    /// Validate a domain by checking if it resolves to an IP address.
    ///
    /// This function attempts to resolve the domain specified in `address` 
    /// by appending a dummy port number and using the `to_socket_addrs` method. 
    /// If the domain resolves successfully, it indicates that the domain is valid.
    ///
    /// # Parameters
    /// - `value`: The input address (domain) to be resolved. This should be a valid 
    ///   domain name or IP address format without any protocol prefix (e.g. "http://").
    /// 
    /// # Returns
    /// - `true` if the domain validation succeeds; otherwise `false` 
    pub fn is_domain(value: &str) -> bool {
        if let Ok(mut addrs_iter) = format!("{}:0", value).to_socket_addrs() {
            if addrs_iter.next().is_some() {
                return true;
            }
        }

        false
    }

    /// Validate an IP address by attempting to parse it.
    ///
    /// This function checks if the `self.address` can be successfully parsed 
    /// into a valid `IpAddr`. It supports both IPv4 and IPv6 formats. 
    /// If the parsing is successful, it indicates that the address is valid.
    /// 
    /// # Parameters
    /// - `value`: The input IP address to be validated. This should be in a valid format 
    ///   for either IPv4 (e.g., "192.168.1.1") or IPv6 
    ///   (e.g., "2001:0db8:85a3:0000:0000:8a2e:0370:7334") addresses.
    ///
    /// # Returns
    /// - `true` if the IP address validation succeeds; otherwise `false`
    pub fn is_ip(value: &str) -> bool {
        value.parse::<IpAddr>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_domain() {
        assert_eq!(true, ValAddr::is_domain("example,com")); // This should fail
    }

    #[test]
    fn cmp_ip() {
        assert_eq!(true, ValAddr::is_ip("192.168.176.43"));
    }
}
