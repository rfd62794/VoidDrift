use bevy::prelude::*;
use crate::components::*;

pub type BaseShipFilter = (
    Without<Station>,
    Without<ActiveAsteroid>,
    Without<Berth>,
    Without<MainCamera>,
    Without<StarLayer>,
    Without<StationVisualsContainer>,
    Without<DestinationHighlight>,
    Without<ShipCargoBarFill>,
);

pub type BaseStationFilter = (
    Without<Ship>,
    Without<ActiveAsteroid>,
    Without<Berth>,
    Without<MainCamera>,
    Without<StarLayer>,
    Without<StationVisualsContainer>,
    Without<DestinationHighlight>,
    Without<ShipCargoBarFill>,
);

pub type BaseCameraFilter = (
    Without<Ship>,
    Without<AutonomousShip>,
    Without<Station>,
    Without<ActiveAsteroid>,
    Without<Berth>,
    Without<DestinationHighlight>,
    Without<StarLayer>,
);

pub type VisualsCameraFilter = (With<MainCamera>, Without<StarLayer>, Without<Ship>, Without<AutonomousShip>, Without<Station>, Without<ActiveAsteroid>, Without<Berth>);
pub type VisualsStarFilter = (Without<MainCamera>, Without<Ship>, Without<AutonomousShip>, Without<Station>, Without<ActiveAsteroid>, Without<Berth>);
pub type VisualsStationFilter = (Without<Ship>, Without<AutonomousShip>, Without<StationVisualsContainer>, Without<ActiveAsteroid>, Without<Berth>);
pub type VisualsContainerFilter = (With<StationVisualsContainer>, Without<Station>, Without<Ship>, Without<AutonomousShip>, Without<ActiveAsteroid>, Without<Berth>);
pub type VisualsShipFilter = (Without<Station>, Without<StationVisualsContainer>, Without<AutonomousShip>, Without<ActiveAsteroid>, Without<Berth>);
pub type VisualsAutoShipFilter = (Without<Station>, Without<StationVisualsContainer>, Without<Ship>, Without<ActiveAsteroid>, Without<Berth>);
