#[derive(Debug)]
pub struct Multipart {
    pub header: Vec<u8>,
    pub footer: Vec<u8>,
    content_length: usize,
    boundary: &'static str,
}

impl Multipart {
    /// Creates a creates basic request outline for file streaming to a multipart/form endpoint.
    ///
    /// ### Arguments
    /// * `file_name` - The name of the file to be uploaded.
    /// * `file_size` - The size of the file to be uploaded.
    ///
    /// ### Returns
    /// A new `Multipart` instance.
    ///
    pub fn new(file_name: &str, file_size: usize) -> Self {
        let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
        let header = format!(
            "--{}\r\n\
             Content-Disposition: form-data; name=\"image\"; filename=\"{}\"\r\n\
             Content-Type: application/octet-stream\r\n\r\n",
            boundary, file_name
        )
        .into_bytes();
        let footer = format!("\r\n--{}--\r\n", boundary).into_bytes();
        let content_length = header.len() + file_size + footer.len();

        Multipart {
            header,
            footer,
            content_length,
            boundary,
        }
    }

    pub fn request_header(&self, host: &str, endpoint: &str) -> Vec<u8> {
        let request_header = format!(
            "POST {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: multipart/form-data; boundary={}\r\n\
             Content-Length: {}\r\n\
             \r\n",
            endpoint, host, self.boundary, self.content_length
        );

        request_header.into_bytes()
    }
}
