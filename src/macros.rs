/// Generate getter/setter for an `Option<char>` field.
///
/// Forms:
/// - `gen_get_set_char!(get_set set_method for field_name);`
/// - `gen_get_set_char!(get_set set_method(preprocess_fn) for field_name);`
///   where `preprocess_fn: fn(char) -> char` (e.g. `char::to_ascii_uppercase`)
/// - `gen_get_set_char!(get_set set_method for field_name where verify_expr);`
/// - `gen_get_set_char!(get_set set_method(preprocess_fn) for field_name where verify_expr);`
///   where `verify_expr` is an expression callable as `verify_expr(c: char) -> BcbpResult<()>`.
///
/// Поведение:
/// - `None` очищает поле (`take()`).
/// - Проверяем ASCII (`InvalidCharacters` для не-ASCII).
/// - Применяем препроцессор, затем валидацию.
/// - Геттер возвращает `Option<char>` по значению.
macro_rules! gen_get_set_char {
    (get_set $method_name:ident for $field_name:ident) => {
        gen_get_set_char!(get_set $method_name(|c: char| c) for $field_name where |_c: char| -> BcbpResult<()> { Ok(()) });
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident) => {
        gen_get_set_char!(get_set $method_name($preprocess) for $field_name where |_c: char| -> BcbpResult<()> { Ok(()) });
    };
    (get_set $method_name:ident for $field_name:ident where $verify:expr) => {
        gen_get_set_char!(get_set $method_name(|c: char| c) for $field_name where $verify);
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident where $verify:expr) => {
        /// Setter for a single ASCII character; `None` clears the field.
        #[inline]
        pub fn $method_name(&mut self, ch: Option<char>) -> BcbpResult<()> {
            match ch {
                None => {
                    self.$field_name.take();
                    Ok(())
                }
                Some(mut c) => {
                    if !c.is_ascii() {
                        return Err(Error::InvalidCharacters);
                    }
                    c = $preprocess(&c);
                    $verify(c)?;
                    self.$field_name = Some(c);
                    Ok(())
                }
            }
        }

        /// Getter: returns the stored character (if any).
        #[inline]
        pub fn $field_name(&self) -> Option<char> {
            self.$field_name
        }
    };
}

macro_rules! gen_get_set {
    (get_set $method_name:ident for $field_name:ident with len $to:literal) => {
        gen_get_set!(get_set $method_name for $field_name with len 1..=$to);
    };
    (get_set $method_name:ident for $field_name:ident with len $from:literal..=$to:literal) => {
        gen_get_set!(get_set $method_name(str::trim) for $field_name with len $from..=$to);
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident with len $to:literal) => {
        gen_get_set!(get_set $method_name($preprocess) for $field_name with len 1..=$to);
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident with len $from:literal..=$to:literal) => {
        gen_get_set!(
            get_set $method_name($preprocess) for $field_name
            with |s: &str| {
                if !($from..=$to).contains(&s.len()) {
                    Err(Error::FieldSizeExceeded)
                } else {
                    Ok(())
                }
            }
        );
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident with $verify:expr) => {
        pub fn $method_name(&mut self, s: Option<&str>) -> BcbpResult<()> {

            let s = s.unwrap_or_default().trim();

            let s = $preprocess(s);

            if s.is_empty() {
                self.$field_name.take();
                return Ok(());
            }

            if !s.is_ascii() {
                return Err(Error::InvalidCharacters);
            }

            $verify(s)?;

            self.$field_name = Some(s.to_owned());
            Ok(())
        }

        pub fn $field_name(&self) -> Option<&str> {
            self.$field_name.as_deref()
        }
    };
}

pub(crate) use gen_get_set;
pub(crate) use gen_get_set_char;
