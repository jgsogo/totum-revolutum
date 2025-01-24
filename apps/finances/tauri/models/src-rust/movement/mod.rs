mod movement_amount;
mod movement_direction;
mod movement_type;

pub use movement_amount::{MovementAmount, MovementAmountDividend};
pub use movement_direction::MovementDirection;
pub use movement_type::MovementType;

// use crate::google_type;
// use crate::money_amount::{MoneyAmountNonNumerable, MoneyAmountNumerable};
// use crate::{Error, Result};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Movement(crate::protos::finances_app_models::Movement);

impl Movement {}

/*


message Movement {
    int64 pk = 1;
    google.type.Date date_value = 2;

    int64 transaction_pk = 3;
    MovementType type = 4;
    MovementDirection direction = 5;

    MovementAmount amount = 6;
    Fx fx = 7;
}


*/
