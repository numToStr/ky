use qrcodegen::QrCode;

use super::KyError;

pub struct Qr {
    qrcode: QrCode,
}

impl Qr {
    pub fn new(data: &str) -> Result<Self, KyError> {
        let qrcode =
            QrCode::encode_text(data, qrcodegen::QrCodeEcc::Medium).map_err(|_| KyError::Qr)?;

        Ok(Self { qrcode })
    }

    pub fn render(self) -> String {
        let qr = self.qrcode;
        let mut code_str = String::new();

        let border: i32 = 2;
        for y in -border..qr.size() + border {
            for x in -border..qr.size() + border {
                let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
                code_str.push_str(&format!("{0}{0}", c));
            }
            code_str.push('\n');
        }

        code_str
    }
}
