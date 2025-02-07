use hecs::Entity;

#[derive(Debug)]
pub struct EntityMoved {
    pub entity: Entity,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

#[derive(Debug)]
pub enum Event {
    PlayerHitObstacle,
    EntityMoved(EntityMoved),
    BoxPlacedOnSpot(BoxPlacedOnSpot),
    GameWon,
}