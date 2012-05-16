import check::{fn_ctxt, methods};

// Requires that the two types unify, and prints an error message if they
// don't.
fn suptype(fcx: @fn_ctxt, sp: span,
           expected: ty::t, actual: ty::t) {

    // n.b.: order of actual, expected is reversed
    alt infer::mk_subty(fcx.infcx, actual, expected) {
      result::ok(()) { /* ok */ }
      result::err(err) {
        fcx.report_mismatched_types(sp, expected, actual, err);
      }
    }
}

fn eqtype(fcx: @fn_ctxt, sp: span,
          expected: ty::t, actual: ty::t) {

    alt infer::mk_eqty(fcx.infcx, actual, expected) {
      result::ok(()) { /* ok */ }
      result::err(err) {
        fcx.report_mismatched_types(sp, expected, actual, err);
      }
    }
}

// Checks that the type `actual` can be assigned to `expected`.
fn assign(fcx: @fn_ctxt, sp: span, borrow_scope: ast::node_id,
          expected: ty::t, expr: @ast::expr) {
    let expr_ty = fcx.expr_ty(expr);
    let anmnt = {expr_id: expr.id, borrow_scope: borrow_scope};
    alt infer::mk_assignty(fcx.infcx, anmnt, expr_ty, expected) {
      result::ok(()) { /* ok */ }
      result::err(err) {
        fcx.report_mismatched_types(sp, expected, expr_ty, err);
      }
    }
}


