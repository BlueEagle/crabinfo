use crabinfo;
use crabinfo::SystemInformation;

fn main() {
    let system_info = SystemInformation::build();
    let battery_profile = SystemInformation::profile_battery();
    let battery_profile = battery_profile.split("\n").for_each(|item| {
        let trimmed_item = item.trim();
        println!("{:?}", trimmed_item);
    });
    // println!("{:?}\n", system_info);
    // println!("{:?}\n", battery_profile);
    // TODO: Add behavior that can parse key values from colon separated associations and create
    // groupings where there would otherwise be duplicate keys.
    //
    // Create a list of all previous keys.
    // Maybe this will look like each time a key is created, if it already exists in the hashmap
    // then remove all keys before duplicate in previous keys list. Using updated list, move all
    // key:value pairs from previous list to new grouping. Empty the list.
    //
    // OR iterate list of possible keys, and write duplicates into duplist.
    // When a duplist is found...The issue with this pattern is as follows:
    // a:
    //  spec:
    //  color:
    // b:
    //  spec:
    //  color:
    // requirements:
    // c:
    //  spec:
    //  color:
    //
    //  In this case, the spec, then color attributes map directly in pattern but should not be
    //  expected to follow an order. Also, counting instances of a pattern is not reliable, since
    //  instances should also not be expected in order (though in most cases this would
    //  realistically be fine).
}
