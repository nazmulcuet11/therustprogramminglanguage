mod front_of_house;

use crate::front_of_house::hosting;
use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}
