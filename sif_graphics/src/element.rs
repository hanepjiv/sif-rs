// -*- mode:rust; coding:utf-8-unix; -*-

//! element.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/12
//  @date 2019/05/27

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
bitflags! {
    #[allow(missing_docs)]
    pub struct Element: i32 {
    #[allow(missing_docs)]
    const REND_                 = -1i32;
    #[allow(missing_docs)]
    const BEGIN_                =  0i32;
    // --------------------------------------------------------------------
    #[allow(missing_docs)]
    const POSITION              =  Self::BEGIN_.bits;
    #[allow(missing_docs)]
    const NORMAL                =  1i32;
    #[allow(missing_docs)]
    const COORD                 =  2i32;
    #[allow(missing_docs)]
    const BONE                  =  3i32;
    #[allow(missing_docs)]
    const WEIGHT                =  4i32;
    #[allow(missing_docs)]
    const TANGENT               =  5i32;
    // --------------------------------------------------------------------
    #[allow(missing_docs)]
    const END_                  =  6i32;
    #[allow(missing_docs)]
    const RBEGIN_               =  Self::END_.bits - 1i32;
    #[allow(missing_docs)]
    const SIZE_                 =  Self::END_.bits - Self::BEGIN_.bits;
    }
}
// ============================================================================
/// const ELEMENT_SIZE
pub const ELEMENT_SIZE: usize = Element::SIZE_.bits as usize;
// ============================================================================
impl Default for Element {
    fn default() -> Self {
        Self::BEGIN_
    }
}
// ============================================================================
impl Element {
    // ========================================================================
    /// fn length
    pub fn length(self) -> Option<usize> {
        match self {
            Self::POSITION => Some(3usize),
            Self::NORMAL => Some(3usize),
            Self::COORD => Some(2usize),
            Self::BONE => Some(4usize),
            Self::WEIGHT => Some(4usize),
            Self::TANGENT => Some(4usize),
            _ => None,
        }
    }
    // ========================================================================
    /// fn location_name
    pub fn location_name(self) -> Option<&'static str> {
        match self {
            Self::POSITION => Some("iv_Position"),
            Self::NORMAL => Some("iv_Normal"),
            Self::COORD => Some("iv_Coord"),
            Self::BONE => Some("iv_BoneIdx"),
            Self::WEIGHT => Some("iv_Weight"),
            Self::TANGENT => Some("iv_Tangent"),
            _ => None,
        }
    }
}
