#![feature(try_trait_v2)]

use std::ops;

#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ControlFlow { 
    Continue,
    Break,
}

impl ops::FromResidual for ControlFlow { 
    fn from_residual(r : ControlFlow) -> Self { 
        r
    }
}

impl ops::Try for ControlFlow {
    type Output = ();
    type Residual = ControlFlow;

    #[inline]
    fn from_output((): Self::Output) -> Self {
        ControlFlow::Continue
    }

    #[inline]
    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ControlFlow::Continue => std::ops::ControlFlow::Continue(()),
            ControlFlow::Break => std::ops::ControlFlow::Break(ControlFlow::Break)
        }
    }
}

#[cfg(test)]
mod test { 
    use super::*;
    fn breaks() -> ControlFlow {
        ControlFlow::Break 
    }

    fn continues() -> ControlFlow { 
        ControlFlow::Continue
    }

    fn test1() -> ControlFlow { 
        breaks()?;
        panic!("reached end");
    }

    fn test2() -> ControlFlow { 
        continues()?;
        panic!("reached end");
    }

    #[test]
    fn ensure_breaks() { 
        assert_eq!(test1(), ControlFlow::Break);
    }

    #[test]
    #[should_panic]
    fn ensure_continues() { 
        test2();
    }

}
