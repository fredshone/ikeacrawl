use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn pathtip(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    
    let mouse_idx = map_idx(map_pos.x, map_pos.y);
    println!("Mouse: {} -{}", map_pos.x.to_string(), map_pos.y.to_string());

    if !player_fov.visible_tiles.contains(&map_pos) {
        return;
    }

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let mut player = <(&Point, &Player)>::query();
    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    println!("Player: {}-{}", player_pos.x, player_pos.y);
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![mouse_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        1024.0
    );

    if let Some(destination) = DijkstraMap::find_lowest_exit(
        &dijkstra_map, mouse_idx, map
    ) {
        let display = "<-Here";
        let destination_point = map.index_to_point2d(destination);
        draw_batch.print(destination_point, &display);
    }

    

    
    // positions
    //     .iter(ecs)
    //     .filter(|(_, pos, _)| 
    //         **pos == map_pos && player_fov.visible_tiles.contains(&pos)
    //     )
    //     .for_each(|(entity, _, name) | {
    //         let screen_pos = *mouse_pos * 2;
    //         let display = if let Ok(health) = ecs.entry_ref(*entity)
    //             .unwrap()
    //             .get_component::<Health>()
    //         {
    //             format!("{} : {} hp", &name.0, health.current)
    //         } else {
    //             name.0.clone()
    //         };
    //         draw_batch.print(screen_pos, &display);
    //     });
    // draw_batch.submit(10100).expect("Batch error");
}