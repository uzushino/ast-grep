// use ast_grep_language::{SupportLang, Language};
// use ast_grep_core::language::TSLanguage;
// use ast_grep_dynamic::DynamicLang;
// use serde::{Serialize, Deserialize};

// use std::borrow::Cow;
// use std::path::Path;

// #[derive(Clone, PartialEq, Eq)]
// pub enum SgLang {
//   Builtin(SupportLang),
//   Custom(DynamicLang),
// }

// impl Serialize for SgLang {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//           match self {
//             Builtin(s) => s.serialize(serializer),
//             Custom(c) => c.serialize(serializer),
//           }
//         }
// }

// impl Deserialize for SgLang {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//         }
// }

// impl From<SupportLang> for SgLang {
//   fn from(value: SupportLang) -> Self {
//     Self::Builtin(value)
//   }
// }

// use SgLang::*;
// impl Language for SgLang {
//   fn get_ts_language(&self) -> TSLanguage {
//     match self {
//       Builtin(b) => b.get_ts_language(),
//       Custom(c)  => c.get_ts_language(),
//     }
//   }

//   fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
//     SupportLang::from_path(path.as_ref()).map(SgLang::Builtin).or_else(|| DynamicLang::from_path(path).map(SgLang::Custom))
//   }

//   fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str> {
//     match self {
//       Builtin(b) => b.pre_process_pattern(query),
//       Custom(c)  => c.pre_process_pattern(query),
//     }
//   }

//   #[inline]
//   fn meta_var_char(&self) -> char {
//     match self {
//       Builtin(b) => b.meta_var_char(),
//       Custom(c)  => c.meta_var_char(),
//     }
//   }

//   #[inline]
//   fn expando_char(&self) -> char {
//     match self {
//       Builtin(b) => b.expando_char(),
//       Custom(c)  => c.expando_char(),
//     }
//   }
// }

// #[cfg(test)]
// mod test {
//   use super::*;
//   use std::mem::size_of;

//   #[test]
//   fn test_sg_lang_size() {
//     assert_eq!(size_of::<SgLang>(), size_of::<DynamicLang>());
//   }
// }