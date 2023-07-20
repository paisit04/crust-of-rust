fn main() {
    // tokio::spwan requires the closure to be static
    // async is not requires the closure to be static
    tokio::spwan(quox(|x| x));
}

// doesn't need to be static
async fn quox<F>(f: F)
where
    F: Fn(&str) -> &str,
{
}
