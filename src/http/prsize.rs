// This file is going to be used to classify PR sizes
// though it's not implemented yet
#[warn(unused)]
enum PrSize {
    XS, 
    S, 
    M,
    L,
    XL,  
   
}

#[warn(unused)]
pub fn classify_pr_size(add: u32, sub: u32) -> PrSize{
    let size: u32 = add + sub;

   match size {
        0..=10 => return PrSize::XS,
        11..=50 => return PrSize::S,
        51..=200 => return PrSize::M,
        201..=500 => return PrSize::L,
        _ => return  PrSize::XL,
    }
}
