macro_rules! gen_get {
    ($field:ident as $ty:ty) => {
        #[inline]
        pub fn $field(&self) -> $ty {
            self.$field
        }
    };

    ($field:ident as_ref $ty:ty) => {
        #[inline]
        pub fn $field(&self) -> $ty {
            self.$field.as_ref()
        }
    };

    ($field:ident as_deref $ty:ty) => {
        #[inline]
        pub fn $field(&self) -> $ty {
            self.$field.as_deref()
        }
    };

    (cond $field:ident as $ty:ty) => {
        #[inline]
        pub fn $field(&self) -> $ty {
            self.conditional_data.as_ref().and_then(|c| c.$field)
        }
    };

    (cond $field:ident as_deref $ty:ty) => {
        #[inline]
        pub fn $field(&self) -> $ty {
            self.conditional_data
                .as_ref()
                .and_then(|c| c.$field.as_deref())
        }
    };
}

macro_rules! gen_set {
    ($field:ident as Option<$ty:ty>) => {
        gen_set!($field(core::convert::identity) as Option<$ty>);
    };
    ($field:ident as Option<$ty:ty> using $bcbp_field:path) => {
        gen_set!($field(core::convert::identity) as Option<$ty> using $bcbp_field);
    };
    (cond $field:ident as Option<$ty:ty> using $bcbp_field:path) => {
        gen_set!(cond $field(core::convert::identity) as Option<$ty> using $bcbp_field);
    };
    ($field:ident as $ty:ty) => {
        gen_set!($field(core::convert::identity) as $ty);
    };

    ($field:ident($preprocess:expr) as Option<$ty:ty>) => {
        paste::paste! {
            #[inline]
            pub fn [<set_ $field>](&mut self, value: Option<$ty>) -> BcbpResult<()> {

                let value = value.map($preprocess);

                self.$field = value;

                Ok(())
            }
        }
    };

    ($field:ident($preprocess:expr) as Option<$ty:ty> using $bcbp_field:path) => {
        paste::paste! {
            #[inline]
            pub fn [<set_ $field>](&mut self, value: Option<$ty>) -> BcbpResult<()> {

                let value = value.map($preprocess).unwrap_or_default();
                let field = $bcbp_field;
                let max = field.len();

                if max > 0 && value.len() > max {
                    return Err(Error::FieldLengthExceeded { field, max });
                }

                if value.is_empty() {
                    self.$field = None;
                } else {
                    self.$field = Some(value.to_owned());
                }

                Ok(())
            }
        }
    };

    ($field:ident($preprocess:expr) as $ty:ty) => {
        paste::paste! {
            #[inline]
            pub fn [<set_ $field>](&mut self, value: $ty) -> BcbpResult<()> {

                let value = $preprocess(value);

                self.$field = value;

                Ok(())
            }
        }
    };

    (cond $field:ident as $ty:ty) => {
        paste::paste! {
            #[inline]
            pub fn [<set_ $field>](&mut self, value: $ty) -> BcbpResult<()> {
                self.conditional_mut().$field = value;

                Ok(())
            }
        }
    };

    (cond $field:ident($preprocess:expr) as Option<$ty:ty> using $bcbp_field:path) => {
        paste::paste! {
            #[inline]
            pub fn [<set_ $field>](&mut self, value: Option<$ty>) -> BcbpResult<()> {

                let value = value.map($preprocess).unwrap_or_default();
                let field = $bcbp_field;
                let max = field.len();

                if max > 0 && value.len() > max {
                    return Err(Error::FieldLengthExceeded { field, max });
                }

                if value.is_empty() {
                    self.conditional_mut().$field = None;
                } else {
                    self.conditional_mut().$field = Some(value.to_owned());
                }

                Ok(())
            }
        }
    };
}

// pub fn set_checkin_src(&mut self, checkin_src: Option<char>) -> BcbpResult<()> {
//     self.conditional_mut().checkin_src = checkin_src;

//     Ok(())
// }

#[allow(unused_macros)]
macro_rules! gen_set_str {
    ($method_name:ident for $field_name:ident with len $to:literal) => {
        gen_set_str!($method_name for $field_name with len 1..=$to);
    };
    ($method_name:ident for $field_name:ident with len $from:literal..=$to:literal) => {
        gen_set_str!($method_name(str::trim) for $field_name with len $from..=$to);
    };
    ($method_name:ident($preprocess:path) for $field_name:ident with len $to:literal) => {
        gen_set_str!($method_name($preprocess) for $field_name with len 1..=$to);
    };
    ($method_name:ident($preprocess:path) for $field_name:ident with len $from:literal..=$to:literal) => {
        gen_set_str!(
            $method_name($preprocess) for $field_name
            with |s: &str| {
                if !($from..=$to).contains(&s.len()) {
                    Err(Error::FieldTooLong)
                } else {
                    Ok(())
                }
            }
        );
    };
    ($method_name:ident($preprocess:path) for $field_name:ident with $verify:expr) => {
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
    };
}

pub(crate) use gen_get;
pub(crate) use gen_set;
#[allow(unused_imports)]
pub(crate) use gen_set_str;
