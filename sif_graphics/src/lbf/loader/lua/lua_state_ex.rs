// -*- mode:rust; coding:utf-8-unix; -*-

//! lua_state_ex.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use lua::State as LuaState;
// ----------------------------------------------------------------------------
use super::{lua_type::LuaType, Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) trait LuaStateEx {
    // ========================================================================
    /// idxtbl
    fn idxtbl<T: LuaType>(&mut self, idx: i32, key: &str) -> Result<T>;
}
// ----------------------------------------------------------------------------
impl LuaStateEx for LuaState {
    // ========================================================================
    /// idxtbl
    fn idxtbl<T: LuaType>(&mut self, idx: i32, key: &str) -> Result<T> {
        debug!("::lbf::LuaStateEx::idxtbl: {}", key);
        self.push_string(key);
        let t = self.get_table(idx - 1);
        let result = if !T::lua_type(t) {
            Err(Error::Type(format!("::lbf::LBF::idxtbl(idx, '{}')", key)))
        } else {
            let result = T::from_lua(self, -1);
            if let Err(ref e) = result {
                error!("::lbf::LuaStateEx::idxtbl: {}", e);
            }
            result
        };
        self.pop(1); // get_table
        result
    }
}
