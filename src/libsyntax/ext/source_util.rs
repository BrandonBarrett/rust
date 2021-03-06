// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::*;
use codemap::{span, Loc, FileMap};
use print::pprust;
use build::{mk_base_vec_e,mk_uint,mk_u8,mk_uniq_str};

export expand_line;
export expand_col;
export expand_file;
export expand_stringify;
export expand_mod;
export expand_include;
export expand_include_str;
export expand_include_bin;

/* line!(): expands to the current line number */
fn expand_line(cx: ext_ctxt, sp: span, arg: ast::mac_arg,
               _body: ast::mac_body) -> @ast::expr {
    get_mac_args(cx, sp, arg, 0u, option::Some(0u), ~"line");
    let loc = cx.codemap().lookup_char_pos(sp.lo);
    return mk_uint(cx, sp, loc.line);
}

/* col!(): expands to the current column number */
fn expand_col(cx: ext_ctxt, sp: span, arg: ast::mac_arg,
              _body: ast::mac_body) -> @ast::expr {
    get_mac_args(cx, sp, arg, 0u, option::Some(0u), ~"col");
    let loc = cx.codemap().lookup_char_pos(sp.lo);
    return mk_uint(cx, sp, loc.col.to_uint());
}

/* file!(): expands to the current filename */
/* The filemap (`loc.file`) contains a bunch more information we could spit
 * out if we wanted. */
fn expand_file(cx: ext_ctxt, sp: span, arg: ast::mac_arg,
               _body: ast::mac_body) -> @ast::expr {
    get_mac_args(cx, sp, arg, 0u, option::Some(0u), ~"file");
    let Loc { file: @FileMap { name: filename, _ }, _ } =
        cx.codemap().lookup_char_pos(sp.lo);
    return mk_uniq_str(cx, sp, filename);
}

fn expand_stringify(cx: ext_ctxt, sp: span, arg: ast::mac_arg,
                    _body: ast::mac_body) -> @ast::expr {
    let args = get_mac_args(cx, sp, arg, 1u, option::Some(1u), ~"stringify");
    let s = pprust::expr_to_str(args[0], cx.parse_sess().interner);
    return mk_uniq_str(cx, sp, s);
}

fn expand_mod(cx: ext_ctxt, sp: span, arg: ast::mac_arg, _body: ast::mac_body)
    -> @ast::expr {
    get_mac_args(cx, sp, arg, 0u, option::Some(0u), ~"file");
    return mk_uniq_str(cx, sp,
                       str::connect(cx.mod_path().map(
                           |x| cx.str_of(*x)), ~"::"));
}

fn expand_include(cx: ext_ctxt, sp: span, arg: ast::mac_arg,
                  _body: ast::mac_body) -> @ast::expr {
    let args = get_mac_args(cx, sp, arg, 1u, option::Some(1u), ~"include");
    let file = expr_to_str(cx, args[0], ~"include_str! requires a string");
    let p = parse::new_sub_parser_from_file(
        cx.parse_sess(), cx.cfg(),
        &res_rel_file(cx, sp, &Path(file)), sp);
    return p.parse_expr();
}

fn expand_include_str(cx: ext_ctxt, sp: codemap::span, arg: ast::mac_arg,
                      _body: ast::mac_body) -> @ast::expr {
    let args = get_mac_args(cx,sp,arg,1u,option::Some(1u),~"include_str");

    let file = expr_to_str(cx, args[0], ~"include_str! requires a string");

    let res = io::read_whole_file_str(&res_rel_file(cx, sp, &Path(file)));
    match res {
      result::Ok(_) => { /* Continue. */ }
      result::Err(ref e) => {
        cx.parse_sess().span_diagnostic.handler().fatal((*e));
      }
    }

    return mk_uniq_str(cx, sp, result::unwrap(res));
}

fn expand_include_bin(cx: ext_ctxt, sp: codemap::span, arg: ast::mac_arg,
                      _body: ast::mac_body) -> @ast::expr {
    let args = get_mac_args(cx,sp,arg,1u,option::Some(1u),~"include_bin");

    let file = expr_to_str(cx, args[0], ~"include_bin! requires a string");

    match io::read_whole_file(&res_rel_file(cx, sp, &Path(file))) {
      result::Ok(src) => {
        let u8_exprs = vec::map(src, |char| {
            mk_u8(cx, sp, *char)
        });
        return mk_base_vec_e(cx, sp, u8_exprs);
      }
      result::Err(ref e) => {
        cx.parse_sess().span_diagnostic.handler().fatal((*e))
      }
    }
}

fn res_rel_file(cx: ext_ctxt, sp: codemap::span, arg: &Path) -> Path {
    // NB: relative paths are resolved relative to the compilation unit
    if !arg.is_absolute {
        let cu = Path(cx.codemap().span_to_filename(sp));
        cu.dir_path().push_many(arg.components)
    } else {
        copy *arg
    }
}

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
