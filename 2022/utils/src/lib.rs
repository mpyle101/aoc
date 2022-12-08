
pub fn timeit<T>(s: &str, func: impl Fn() -> T)
    where T: std::fmt::Debug
{
    let t = std::time::Instant::now();
    let result = func();
    println!("{s}: {:?} ({:?})", result, t.elapsed());
}
