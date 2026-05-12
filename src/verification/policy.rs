use crate::verification::node_location::NodeLocation;
use strum::Display;

#[derive(Debug, Display, Clone)]
pub enum SyntaxPolicy {
    Allowed,
    Blocked(BlockedLocations),
}

#[derive(Debug)]
pub struct BlockedLocations {
    pub locations: Vec<NodeLocation>,
}

impl Clone for BlockedLocations {
    fn clone(&self) -> Self {
        Self {
            locations: self.locations.clone(),
        }
    }
}
