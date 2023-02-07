use super::Board;

#[rustfmt::skip]
const DEFAULT_15X15: &'static str = 
"tw s  s  dl s  s  s  tw s  s  s  dl s  s tw 
s  dw s  s  s  s  s  s  s  s  s  s  s  dw s
s  s  dw s  s  s  s  s  s  s  s  s  dw s  s
dl s  s  dw s  s  s  s  s  s  s  dw s  s  dl
s  s  s  s  dw s  s  s  s  s  dw s  s  s  s
s  tl s  s  s  tl s  s  s  tl s  s  s  tl s 
s  s  dl s  s  s  dl s  dl s  s  s  dl s  s
tw s  s  dl s  s  s  dw s  s  s  dl s  s  dw
s  s  dl s  s  s  dl s  dl s  s  s  dl s  s
s  tl s  s  s  tl s  s  s  tl s  s  s  tl s 
s  s  s  s  dw s  s  s  s  s  dw s  s  s  s
dl s  s  dw s  s  s  s  s  s  s  dw s  s  dl
s  s  dw s  s  s  s  s  s  s  s  s  dw s  s
s  dw s  s  s  s  s  s  s  s  s  s  s  dw s
tw s  s  dl s  s  s  tw s  s  s  dl s  s  tw";

impl Default for Board<15> {
    fn default() -> Self {
        Self::try_from(DEFAULT_15X15.to_owned()).unwrap()
    }
}

impl Default for Board<1> {
    fn default() -> Self {
        Self::try_from("s".to_owned()).unwrap()
    }
}
