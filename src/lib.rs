use anchor_lang::declare_id;
pub use anchor_gen::generate_cpi_crate;

generate_cpi_crate!("drift_2.92.0.json");
declare_id!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

pub mod public_api {
    pub use drift::cpi::accounts::{InitializeUser, InitializeUserStats, Withdraw as DriftWithdraw, Deposit as DriftDeposit};
    pub use drift::cpi::{withdraw as driftwithdraw, deposit as driftdeposit, initialize_user_stats, initialize_user};
    pub use drift::SpotBalanceType::{Deposit as DepositType, Borrow as BorrowType};
}
