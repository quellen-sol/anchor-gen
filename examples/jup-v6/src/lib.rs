// use typedefs::Swap;

anchor_gen::generate_cpi_crate!("idl.json");

mod output;
pub use output::*;

// declare_id!("Govz1VyoyLD5BL6CSCxUJLVLsQHRwjfFj1prNsdNg5Jw");

fn thing() {
  let t = SharedAccountsRouteIx {
    id: 0,
    route_plan: vec![],
    in_amount: 0,
    quoted_out_amount: 0,
    slippage_bps: 0,
    platform_fee_bps: 0,
  };
}
