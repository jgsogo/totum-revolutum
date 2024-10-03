use itertools::Itertools;
use serde::Serialize;

use super::Account;

#[derive(Serialize, Debug)]
pub struct MenuGroup {
    /// The name of the group
    pub name: String,

    /// All the accounts included in the group
    pub accounts: Vec<Account>,
}

impl MenuGroup {
    pub fn new_grouped_by_holder(accounts: Vec<Account>) -> Vec<Self> {
        accounts
            .into_iter()
            // order first, so all the groups are together
            .sorted_by(|acc_lhs, acc_rhs| {
                Ord::cmp(&acc_lhs.holder.name.to_lowercase(), &acc_rhs.holder.name.to_lowercase())
            })
            // split the vector into the groups
            .chunk_by(|acc| acc.holder.name.clone())
            .into_iter()
            // create the MenuGroup entries from each chunk
            .map(|(key, chunk)| MenuGroup {
                name: key,
                accounts: chunk.collect(),
            })
            .collect()
    }
}
