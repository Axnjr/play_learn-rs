use std::{thread, time::Duration};

const  DATA: &str = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668"
;

/// for larger data (>= 7rows) async version is faster !!
pub fn map_reduce_sync() {

    let mut children: Vec<u32> = vec![];
    let chunked_data = DATA.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        let result: u32 = data_segment
            // iterate over the characters of our segment..
            .chars()
            // .. convert text-characters to their number value..
            .map(|c| c.to_digit(10).expect("should be a digit"))
            // .. and sum the resulting iterator of numbers
            .sum()
        ;

        children.push(result);
        println!("processed segment {}, result={}", i, result);
    }

    // explicitly given `sum` function type of `u32`
    let final_result = children.iter().sum::<u32>();
    println!("Final sum result: {}", final_result);
}


/// for smaller data (7rows <=) sync version is faster !!
/// ## This is our data to process.
///     - We will calculate the sum of all digits via a threaded map-reduce algorithm.
///     - Each whitespace separated chunk will be handled in a different thread.
pub fn map_reduce_async() {

    // let mut counter = 0;
    // ticker(move || {
    //     println!("Ticker executing the closure of the main function. Counter: {}", counter);
    //     counter += 1;
    // });

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = DATA.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                        // iterate over the characters of our segment..
                        .chars()
                        // .. convert text-characters to their number value..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // .. and sum the resulting iterator of numbers
                        .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result

        }));
    }


    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // combine each thread's intermediate results into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);


}