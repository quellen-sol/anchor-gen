use anchor_lang::prelude::*;
#[derive(AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct AddLiquidity {
    pub token_amount_in: u64,
    pub min_lp_amount_out: u64,
    pub token_amount_pre_swap: Option<u64>,
}
#[derive(AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct RemoveLiquidity {
    pub lp_amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}
#[derive(AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}
#[derive(AnchorDeserialize, Clone, Debug, Copy)]
pub enum Side {
    Bid,
    Ask,
}
impl Default for Side {
    fn default() -> Self {
        Self::Bid
    }
}
#[derive(AnchorDeserialize, Clone, Debug, Copy)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema,
    Lifinity,
    Mercurial,
    Cykura,
    Serum,
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin,
    AldrinV2,
    Whirlpool,
    Invariant,
    Meteora,
    GooseFX,
    DeltaFi,
    Balansol,
    MarcoPolo,
    Dradex,
    LifinityV2,
    RaydiumClmm,
    Openbook,
    Phoenix,
    Symmetry,
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake,
    GooseFXV2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
}
impl Default for Swap {
    fn default() -> Self {
        Self::Saber
    }
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct RouteIx {
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct RouteWithTokenLedgerIx {
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SharedAccountsRouteIx {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SharedAccountsRouteWithTokenLedgerIx {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SharedAccountsExactOutRouteIx {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SetTokenLedgerIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct CreateOpenOrdersIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct CreateProgramOpenOrdersIx {
    pub id: u8,
}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct CreateTokenLedgerIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct MercurialSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct CykuraSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SerumSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SaberSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SaberAddDecimalsIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct TokenSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct TokenSwapV2Ix {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SenchaSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct StepSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct CropperSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct RaydiumSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct CremaSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct LifinitySwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct MarinadeDepositIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct MarinadeUnstakeIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct AldrinSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct AldrinV2SwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct WhirlpoolSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct InvariantSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct MeteoraSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct GoosefxSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct DeltafiSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct BalansolSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct MarcoPoloSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct DradexSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct LifinityV2SwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct RaydiumClmmSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct PhoenixSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct SymmetrySwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct HeliumTreasuryManagementRedeemV0Ix {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct GoosefxV2SwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct PerpsSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct PerpsAddLiquidityIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct PerpsRemoveLiquidityIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct MeteoraDlmmSwapIx {}
#[derive(AnchorDeserialize, Clone, Debug)]
pub struct TokenLedger {
    pub token_account: Pubkey,
    pub amount: u64,
}
