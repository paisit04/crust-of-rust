#![allow(dead_code, unused_variables)]

#[tokio::main]
async fn main() {
    // https://docs.rs/tokio/latest/tokio/fs/index.html
    // https://docs.rs/tokio/latest/tokio/io/index.html

    let files: Vec<_> = (0..3)
        .map(|i| tokio::fs::read_to_string(format!("file{}", i)))
        .collect();

    // compare
    // reading from file1, file2, to file3. (sequential)
    let file1 = files[0].await;
    let file2 = files[1].await;
    let file3 = files[2].await;

    // to this
    // running all of this concurrently when all completes, then give me all the outputs
    let (file1, file2, file3) = tokio::join!(files[0], files[1], files[2]);

    // please look at https://docs.rs/futures-util/latest/futures_util/future/fn.try_join_all.html
    // make sure the results is the same order as the inputs
    // let file_bytes = try_join_all(files);
}
