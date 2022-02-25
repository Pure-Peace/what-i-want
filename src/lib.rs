//! Some tools to help with the return value
//!
//! # Examples
//!
//! ```ignore
//! use what_i_want::*;
//!
//! fn login(username: String) -> bool {
//!     require!(username == "admin", false);
//!     ...
//! }
//!
//! fn login2(username: String) {
//!     require!(username == "admin");
//!     ...
//! }
//!
//! ```
//!
//! # Handling Result and Option
//!
//! # Examples
//!
//! ```ignore
//! // Before using `what_i_want`
//! pub async fn get_mutipart_data(mut mutipart_data: Multipart) -> MultipartData {
//!     // Nested hell, and different Enum (Result, Option) handling
//!     // Of course this code is just for demonstration
//!     while let Some(Ok(mut field)) = mutipart_data.next().await {
//!         if let Some(disposition) = field.headers().get(&header::CONTENT_DISPOSITION) {
//!             if let Ok(disposition_str) = disposition.to_str() {
//!                 if let Some(dis) = ContentDisposition::parse(disposition_str) {
//!                     if let Some(key) = dis.name {
//!                         while let Some(Ok(chunk)) = field.next().await {
//!                             ...
//!                         }
//!                     }
//!                 }
//!             }
//!         }
//!     }
//!     MultipartData { ... }
//! }
//!
//! // After using `what_i_want`
//! use what_i_want::*;
//!
//! async fn get_mutipart_data(mut mutipart_data: Multipart) -> MultipartData {
//!     while let Some(Ok(mut field)) = mutipart_data.next().await {
//!         let disposition = unwrap_or_continue!(field.headers().get(&header::CONTENT_DISPOSITION));
//!         let disposition_str = unwrap_or_continue!(disposition.to_str());
//!         let dis = unwrap_or_continue!(ContentDisposition::parse(disposition_str));
//!         let key = unwrap_or_continue!(dis.name);
//!         while let Some(Ok(chunk)) = field.next().await {
//!             ...
//!         }
//!     }
//!     MultipartData { ... }
//! }
//! ```

/// Implement `WhatIwant` and let us know what you want
///
///
/// # Examples
///
/// ```ignore
/// use what_i_want::WhatIwant;
///
///
/// impl<T, E> WhatIwant for Result<T, E> {
///     fn is_i_want(&self) -> bool {
///         self.is_ok()
///     }
/// }
///
///
/// impl<T> WhatIwant for Option<T> {
///     fn is_i_want(&self) -> bool {
///         self.is_some()
///     }
/// }
/// 
/// // Custom enum
/// enum LoginReply {
///     Success,
///     Failed(i32)
/// }
///
/// impl WhatIwant for LoginReply {
///     fn is_i_want(&self) -> bool {
///         match self {
///             LoginReply::Success => true,
///             _ => false
///         }
///     }
/// }
///
/// ```
pub trait WhatIwant {
    fn is_i_want(&self) -> bool;
}

impl<T, E> WhatIwant for Result<T, E> {
    fn is_i_want(&self) -> bool {
        self.is_ok()
    }
}

impl<T> WhatIwant for Option<T> {
    fn is_i_want(&self) -> bool {
        self.is_some()
    }
}

#[macro_export]
/// If it's not what you want, then do what you want
///
/// # Examples
///
/// ```
/// use what_i_want::*;
///
/// let an_err: Result<Option<i32>, ()> = Ok(Some(1));
/// let an_option: Option<Option<i32>> = Some(Some(1));
///
/// unwrap_or_do!(an_err, Some(0));
/// unwrap_or_do!(an_option, Some(0));
///
/// fn a_func() -> bool {
///     let an_option = Some(true);
///     unwrap_or_do!(an_option, return false)
/// }
///
/// ```
macro_rules! unwrap_or_do {
    ($exp: expr, $do: expr) => {
        if $exp.is_i_want() {
            $do
        } else {
            $exp.unwrap()
        }
    };
}

#[macro_export]
/// If it's not what you want, then do `continue`
///
/// # Examples
///
/// ```ignore
/// use what_i_want::*;
///
/// async fn get_mutipart_data(mut mutipart_data: Multipart) -> MultipartData {
///     while let Some(Ok(mut field)) = mutipart_data.next().await {
///         let disposition = unwrap_or_continue!(field.headers().get(&header::CONTENT_DISPOSITION));
///         let disposition_str = unwrap_or_continue!(disposition.to_str());
///         let dis = unwrap_or_continue!(ContentDisposition::parse(disposition_str));
///         let key = unwrap_or_continue!(dis.name);
///         while let Some(Ok(chunk)) = field.next().await {
///             ...
///         }
///     }
///     MultipartData { ... }
/// }
/// ```
macro_rules! unwrap_or_continue {
    ($exp: expr) => {
        unwrap_or_do!($exp, continue)
    };
}

#[macro_export]
/// If it's not what you want, then do `return ()`
///
/// # Examples
///
/// ```ignore
/// use what_i_want::*;
///
/// fn a_func(result: Result<i32, ()>) -> () {
///     let unwrapped = unwrap_or_return!(result);
///     // If ok, do something, else return
///     ...
/// }
/// ```
macro_rules! unwrap_or_return {
    ($exp: expr) => {
        unwrap_or_do!($exp, return)
    };
}

#[macro_export]
/// If it's not what you want, then do `return false`
///
/// # Examples
///
/// ```ignore
/// use what_i_want::*;
///
/// fn a_func(result: Result<i32, ()>) -> bool {
///     let unwrapped = unwrap_or_false!(result);
///     // If ok, do something, else return
///     ...
/// }
/// ```
macro_rules! unwrap_or_false {
    ($exp: expr) => {
        unwrap_or_do!($exp, return false)
    };
}

#[macro_export]
/// If it's not what you want, then do `return true`
///
/// # Examples
///
/// ```ignore
/// use what_i_want::*;
///
/// fn a_func(result: Result<i32, ()>) -> bool {
///     let unwrapped = unwrap_or_true!(result);
///     // If ok, do something, else return
///     ...
/// }
/// ```
macro_rules! unwrap_or_true {
    ($exp: expr) => {
        unwrap_or_do!($exp, return false)
    };
}

#[macro_export]
/// If it's not what you want, then do `return <defined return value>`
///
/// # Examples
///
/// ```ignore
/// use what_i_want::*;
///
/// fn a_func(result: Result<i32, ()>) -> bool {
///     let unwrapped = unwrap_or_val!(result, false);
///     // If ok, do something, else return
///     ...
/// }
/// ```
macro_rules! unwrap_or_val {
    ($exp: expr, $val: expr) => {
        unwrap_or_do!($exp, return $val)
    };
}

#[macro_export]
/// Execute if the condition is true, otherwise return
///
/// # Examples
///
/// ```ignore
/// use what_i_want::*;
///
/// fn login(username: String) -> bool {
///     require!(username == "admin", false);
/// }
///
/// fn login2(username: String) {
///     require!(username == "admin");
/// }
/// ```
macro_rules! require {
    ($condition: expr) => {
        if !$condition {
            return;
        }
    };
    ($condition: expr, $return: expr) => {
        if !$condition {
            return $return;
        }
    };
}
