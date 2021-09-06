use lazy_static::lazy_static;

pub struct RGB(pub u8, pub u8, pub u8);

lazy_static! {
    // Ref https://svn.fractint.net/trunk/fractint/maps/default.map
    pub static ref VGA_MAP: Vec<RGB> = vec!(
        RGB(0, 0, 0),
        RGB(0, 0, 168),
        RGB(0, 168, 0),
        RGB(0, 168, 168),
        RGB(168, 0, 0),
        RGB(168, 0, 168),
        RGB(168, 84, 0),
        RGB(168, 168, 168),
        RGB(84, 84, 84),
        RGB(84, 84, 252),
        RGB(84, 252, 84),
        RGB(84, 252, 252),
        RGB(252, 84, 84),
        RGB(252, 84, 252),
        RGB(252, 252, 84),
        RGB(252, 252, 252),
    );
}
