use crate::prelude::*;
use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR : usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map : Map::new(),
            rooms : Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };
        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);
        self.drunkard(&center, rng, &mut mb.map);

        while mb.openness() < DESIRED_FLOOR {
            self.drunkard(
                &Point::new(
                    rng.range(0, SCREEN_WIDTH), 
                    rng.range(0, SCREEN_HEIGHT)
                ),
                rng,
                &mut mb.map
            );
            
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0
            );

            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {

    fn drunkard (
        &mut self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map,
    ) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x -=1,
                1 => drunkard_pos.x +=1,
                2 => drunkard_pos.y -=1,
                _ => drunkard_pos.y += 1
            }
            if !map.in_bounds(drunkard_pos) {
                break;
            }
            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let closest_point = map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx,_)| (idx, DistanceAlg::Pythagoras.distance2d(
                center,
                map.index_to_point2d(idx)
            )))
            .min_by(|(_,distance), (_,distance2)|
            distance.partial_cmp(&distance2).unwrap()
        )
        .map(|(idx, _)| idx)
        .unwrap();
    map.index_to_point2d(closest_point)
    }


}