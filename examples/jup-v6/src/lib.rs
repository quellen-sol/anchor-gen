anchor_gen::generate_cpi_crate!("idl.json");

// declare_id!("Govz1VyoyLD5BL6CSCxUJLVLsQHRwjfFj1prNsdNg5Jw");

fn handle_union(union: InstructionUnion) {
    match union {
        InstructionUnion::SharedAccountsExactOutRoute(ix) => {
            println!("{}", ix.platform_fee_bps);
        }
        _ => {
            panic!("AAAAAAAAA");
        }
    }
}

fn thing() {
    let _s: FeeEvent;

    let _t = SharedAccountsRouteIx {
        id: 0,
        route_plan: vec![],
        in_amount: 0,
        quoted_out_amount: 0,
        slippage_bps: 0,
        platform_fee_bps: 0,
    };
}
