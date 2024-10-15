fn test() {
    // --------Test get neighbors-------------------------

    for neighbor in get_neighbors(BASE) {
        println!("{neighbor:?}");
    }

    // --------Test antipode-------------------------

    let debug = antipode(&BASE);
    println!("{debug:?}");

    // ---------Test build map------------------------

    let map = build_map();

    let a = map[&['a', 'b', 'c', 'e', 'd', 'f', 'g', 'h', 'i']];
    let b = map[&antipode(&BASE)];

    println!("{a} and {b}");

    // ---------Test check1------------------------

    let word = vec![
        ['a', 'b', 'c', 'e', 'd', 'f', 'g', 'h', 'i'],
        ['a', 'c', 'b', 'e', 'd', 'f', 'g', 'h', 'i'],
        ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
        ['a', 'b', 'c', 'e', 'd', 'g', 'f', 'h', 'i'],
    ];

    let a_s = antisum(&word);

    println!("Antisum is {a_s}");

    println!("{}", check1(&word, &BASE, a_s));

    // ---------Test check2------------------------

    let word = vec![BASE, BASE, BASE, BASE];

    let a = check2(&word);

    println!("{a:?}");
}
