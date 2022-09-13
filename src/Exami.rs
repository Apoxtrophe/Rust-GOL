fn gameoflife() {

    let mut X_Cell = 0;
    let mut Y_Cell = 0;


    // Create gridded world
    let mut world: Grid<u8> = Grid::new(World_Size, World_Size);

    //let mut world: Grid<u8> = grid![[1,0,1,1]
    //                                [1,1,0,1]
    //                                [1,0,0,1]
    //                                [0,1,0,0]];
    // Fill with random values
    for z in 0..world.rows() {
        for x in 0..world.cols() {
            let mut rng = thread_rng();
            let y1: u8 = rng.gen_range(0..=1);
            *world.get_mut(z, x).unwrap() = y1;
        }
    }
    let mut neighbors = 0;

    //let mut Low_X_Check= (X_Cell - 1);
    let mut Low_X_Check = 0;

    if X_Cell + 1 == 1{
        Low_X_Check = X_Cell;
    }
    else {
        Low_X_Check= (X_Cell - 1);
    }

    for x in Low_X_Check..=X_Cell + 1{
        if Y_Cell + 1 != 1{
            let mut high_check = world.get(Y_Cell - 1,x);
            match high_check {
                Some(1) => {neighbors += 1}
                Some(0) => {}
                Some(value) => {}
                None => {}
            }
        }
        let mut middle_check = world.get(Y_Cell,x);
        match middle_check {
            Some(1) => {neighbors += 1}
            Some(0) => {}
            Some(value) => {}
            None => {}
        }
        let mut bottom_check = world.get(Y_Cell + 1,x);
        match bottom_check {
            Some(1) => {neighbors += 1}
            Some(0) => {}
            Some(value) => {}
            None => {}
        }
    }
    // If the cell is already filled subtract 1 neighbor
    if world.get(Y_Cell,X_Cell) == Some(&1){
        neighbors = neighbors - 1;
    }

    // Print the world to the console as a series of vectors
    let mut console_screen = vec! [];
    let mut screen_push = 0;


    for x in 0..world.rows(){
        for x in 0..world.rows(){
            console_screen.push(world.get(screen_push,x).unwrap());
        }
        println!("{:?}", console_screen);
        console_screen = vec! [];
        screen_push += 1;
    }
    println!("X Position: {}, Y Position: {}", X_Cell, Y_Cell);
    println!("Number of neighbors: {}", neighbors);

}
