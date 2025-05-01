
pub trait Component : Copy + Clone + 'static
{
    fn new() -> Self;
}