#[link(name="Crack")]
extern "C"
{
    fn _CRACK();
}

fn main()
{
    unsafe
    {
        _CRACK();
    }
}
