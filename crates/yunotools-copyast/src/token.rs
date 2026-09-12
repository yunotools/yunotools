use crate::config::TokenModel;
use crate::domain::TextFile;

const COPYAST_BANNER: &str = "******************Yunotools-Copyast******************";

// Kết quả ước lượng token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenEstimate {
    // Model được sử dụng để ước lượng.
    pub model: TokenModel,

    // Tổng token ước lượng.
    pub tokens: u64,

    // Tổng số ký tự, bao gồm header.
    pub characters: u64,

    // Tổng số byte UTF-8, bao gồm header.
    pub bytes: u64,
}

impl TokenEstimate {
    // Kiểm tra nội dung có nằm trong giới hạn token hay không.
    pub fn fits_within(&self, token_limit: u64) -> bool {
        self.tokens <= token_limit
    }

    // Số token còn lại trước khi đạt giới hạn.
    pub fn remaining_tokens(&self, token_limit: u64) -> u64 {
        token_limit.saturating_sub(self.tokens)
    }
}

pub struct TokenEstimator;

// characters khác bytes thế nào?
// Với ASCII:
//      let text = "abc";
// Ta có:
//      characters = 3
//      bytes      = 3
// Nhưng với tiếng Việt:
//      let text = "ế";
// Ta thường có:
//      characters = 1
//      bytes      = 3
// Bởi vì Rust lưu String dưới dạng UTF-8. Một ký tự Unicode có thể cần nhiều byte.
impl TokenEstimator {
    // Ước lượng tổng token của tất cả file,
    // bao gồm cả header do Copyast tạo ra.
    pub fn estimate(files: &[TextFile], model: TokenModel) -> TokenEstimate {
        let mut total_tokens = 0_u64;
        let mut total_characters = 0_u64;
        let mut total_bytes = 0_u64;

        for file in files {
            let header = create_header(file);

            total_tokens = total_tokens
                .saturating_add(Self::estimate_text(&header, model))
                .saturating_add(Self::estimate_text(&file.content, model));

            total_characters = total_characters
                .saturating_add(count_characters(&header))
                .saturating_add(count_characters(&file.content));

            total_bytes = total_bytes
                .saturating_add(count_bytes(&header))
                .saturating_add(count_bytes(&file.content));
        }

        TokenEstimate {
            model,
            tokens: total_tokens,
            characters: total_characters,
            bytes: total_bytes,
        }
    }

    pub fn estimate_text(text: &str, model: TokenModel) -> u64 {
        let mut ascii_characters = 0_u64;
        let mut non_ascii_characters = 0_u64;

        for character in text.chars() {
            if character.is_ascii() {
                ascii_characters += 1;
            } else {
                non_ascii_characters += 1;
            }
        }

        let ascii_tokens = match model {
            TokenModel::Claude => {
                // Khoảng 3.8 ký tự ASCII cho một token
                divide_rounding_up(ascii_characters.saturating_mul(10), 38)
            }

            _ => {
                // Khoảng 4 ký tự ASCII cho một token.
                divide_rounding_up(ascii_characters, 4)
            }
        };

        let non_ascii_tokens = match model {
            TokenModel::Cl100k => {
                // Tokenizer cũ thường xử lý Unicode
                // kém hiệu quả hơn
                non_ascii_characters
            }

            TokenModel::O200k => {
                // Ước lượng khoảng 2 token
                // cho mỗi 3 ký tự Unicode
                divide_rounding_up(non_ascii_characters.saturating_mul(2), 3)
            }

            TokenModel::Claude => non_ascii_characters,

            TokenModel::Gemini => {
                // Ước lượng khoảng 3 token
                // cho mỗi 4 ký tự Unicode
                divide_rounding_up(non_ascii_characters.saturating_mul(3), 4)
            }
        };

        ascii_tokens.saturating_add(non_ascii_tokens)
    }
}

// Tạo đúng hai dòng header mà Writer sẽ ghi.
fn create_header(file: &TextFile) -> String {
    format!(
        "{}\n******************{}******************\n",
        COPYAST_BANNER,
        file.path.display(),
    )
}

// Chia số nguyên nhưng làm tròn lên.
// Ví dụ:
// 5 / 4 bình thường bằng 1.
// Hàm này trả về 2.
fn divide_rounding_up(value: u64, divisor: u64) -> u64 {
    if value == 0 {
        return 0;
    }

    value.saturating_add(divisor - 1) / divisor
}

fn count_characters(text: &str) -> u64 {
    let count = text.chars().count();
    u64::try_from(count).unwrap_or(u64::MAX)
}

fn count_bytes(text: &str) -> u64 {
    let count = text.len();
    u64::try_from(count).unwrap_or(u64::MAX)
}
