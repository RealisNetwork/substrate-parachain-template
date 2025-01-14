#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_uniques.
pub trait WeightInfo {
    fn create() -> Weight;
    fn force_create() -> Weight;
    fn destroy(n: u32, m: u32, a: u32) -> Weight;
    fn mint() -> Weight;
    fn burn() -> Weight;
    fn transfer() -> Weight;
    fn redeposit(i: u32) -> Weight;
    fn freeze() -> Weight;
    fn thaw() -> Weight;
    fn freeze_class() -> Weight;
    fn thaw_class() -> Weight;
    fn transfer_ownership() -> Weight;
    fn set_team() -> Weight;
    fn force_asset_status() -> Weight;
    fn set_attribute() -> Weight;
    fn clear_attribute() -> Weight;
    fn set_metadata() -> Weight;
    fn clear_metadata() -> Weight;
    fn set_class_metadata() -> Weight;
    fn clear_class_metadata() -> Weight;
    fn approve_transfer() -> Weight;
    fn cancel_approval() -> Weight;
    fn set_accept_ownership() -> Weight;
}

/// Weights for pallet_uniques using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:1)
    fn create() -> Weight {
        (24_063_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:1)
    fn force_create() -> Weight {
        (13_017_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Asset (r:1 w:0)
    // Storage: Uniques ClassAccount (r:0 w:1)
    // Storage: Uniques Attribute (r:0 w:1000)
    // Storage: Uniques ClassMetadataOf (r:0 w:1)
    // Storage: Uniques InstanceMetadataOf (r:0 w:1000)
    // Storage: Uniques Account (r:0 w:20)
    fn destroy(n: u32, m: u32, a: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 14_000
            .saturating_add((9_248_000 as Weight).saturating_mul(n as Weight))
            // Standard Error: 14_000
            .saturating_add((854_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 14_000
            .saturating_add((758_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Account (r:0 w:1)
    fn mint() -> Weight {
        (29_865_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Account (r:0 w:1)
    fn burn() -> Weight {
        (31_603_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Account (r:0 w:2)
    fn transfer() -> Weight {
        (23_331_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Asset (r:100 w:100)
    fn redeposit(i: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 12_000
            .saturating_add((11_527_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Class (r:1 w:0)
    fn freeze() -> Weight {
        (18_617_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Class (r:1 w:0)
    fn thaw() -> Weight {
        (18_618_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    fn freeze_class() -> Weight {
        (13_570_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    fn thaw_class() -> Weight {
        (13_937_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:2)
    fn transfer_ownership() -> Weight {
        (31_021_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    fn set_team() -> Weight {
        (14_739_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:1)
    fn force_asset_status() -> Weight {
        (16_826_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:0)
    // Storage: Uniques Attribute (r:1 w:1)
    fn set_attribute() -> Weight {
        (37_010_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:0)
    // Storage: Uniques Attribute (r:1 w:1)
    fn clear_attribute() -> Weight {
        (34_432_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:1)
    fn set_metadata() -> Weight {
        (28_575_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:1)
    fn clear_metadata() -> Weight {
        (28_730_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassMetadataOf (r:1 w:1)
    fn set_class_metadata() -> Weight {
        (28_225_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques ClassMetadataOf (r:1 w:1)
    fn clear_class_metadata() -> Weight {
        (26_455_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    fn approve_transfer() -> Weight {
        (19_587_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    fn cancel_approval() -> Weight {
        (19_417_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    fn set_accept_ownership() -> Weight {
        (19_417_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:1)
    fn create() -> Weight {
        (24_063_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:1)
    fn force_create() -> Weight {
        (13_017_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Asset (r:1 w:0)
    // Storage: Uniques ClassAccount (r:0 w:1)
    // Storage: Uniques Attribute (r:0 w:1000)
    // Storage: Uniques ClassMetadataOf (r:0 w:1)
    // Storage: Uniques InstanceMetadataOf (r:0 w:1000)
    // Storage: Uniques Account (r:0 w:20)
    fn destroy(n: u32, m: u32, a: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 14_000
            .saturating_add((9_248_000 as Weight).saturating_mul(n as Weight))
            // Standard Error: 14_000
            .saturating_add((854_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 14_000
            .saturating_add((758_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Account (r:0 w:1)
    fn mint() -> Weight {
        (29_865_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Account (r:0 w:1)
    fn burn() -> Weight {
        (31_603_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Account (r:0 w:2)
    fn transfer() -> Weight {
        (23_331_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques Asset (r:100 w:100)
    fn redeposit(i: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 12_000
            .saturating_add((11_527_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Class (r:1 w:0)
    fn freeze() -> Weight {
        (18_617_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: Uniques Class (r:1 w:0)
    fn thaw() -> Weight {
        (18_618_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    fn freeze_class() -> Weight {
        (13_570_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    fn thaw_class() -> Weight {
        (13_937_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:2)
    fn transfer_ownership() -> Weight {
        (31_021_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    fn set_team() -> Weight {
        (14_739_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassAccount (r:0 w:1)
    fn force_asset_status() -> Weight {
        (16_826_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:0)
    // Storage: Uniques Attribute (r:1 w:1)
    fn set_attribute() -> Weight {
        (37_010_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:0)
    // Storage: Uniques Attribute (r:1 w:1)
    fn clear_attribute() -> Weight {
        (34_432_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:1)
    fn set_metadata() -> Weight {
        (28_575_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques InstanceMetadataOf (r:1 w:1)
    fn clear_metadata() -> Weight {
        (28_730_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:1)
    // Storage: Uniques ClassMetadataOf (r:1 w:1)
    fn set_class_metadata() -> Weight {
        (28_225_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques ClassMetadataOf (r:1 w:1)
    fn clear_class_metadata() -> Weight {
        (26_455_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    fn approve_transfer() -> Weight {
        (19_587_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    fn cancel_approval() -> Weight {
        (19_417_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    fn set_accept_ownership() -> Weight {
        (19_417_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}