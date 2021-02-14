mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path. Since add_to_waitlist is defined on the
    // same crate as eat_at_reastaurant, we can use the "crate"
    // keyword.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Explanation:
// In the absolute path, we start with crate, the root of our crate’s
// module tree. Then the front_of_house module is defined in the crate
// root. The front_of_house module isn’t public, but because the
// eat_at_restaurant function is defined in the same module as
// front_of_house (that is, eat_at_restaurant and front_of_house are siblings),
// we can refer to front_of_house from eat_at_restaurant. Next is the hosting
// module marked with pub. We can access the parent module of hosting,
// so we can access hosting. Finally, the add_to_waitlist function is marked
// with pub and we can access its parent module, so this function call works!
