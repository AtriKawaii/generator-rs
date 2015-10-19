#[macro_use]
extern crate generator;

use generator::*;

#[test]
fn generator_is_done() {
    let mut g = generator!({
        _yield_!();
    });

    g.next();
    assert!(!g.is_done());
    g.next();
    assert!(g.is_done());
}

#[test]
fn test_yield() {
    let mut g = FnGenerator::new(|| {
        _yield_!(10);
        20
    });

    // the para type could be deduced here
    let i = g.send(());
    assert!(i == 10);

    let j = g.next();
    assert!(j.unwrap() == 20);
}

#[test]
fn test_scoped() {
    use std::rc::Rc;
    use std::cell::RefCell;
    let x = Rc::new(RefCell::new(10));

    let x1 = x.clone();
    let mut g = FnGenerator::<(), _>::new(move|| {
         *x1.borrow_mut() = 20;
         _yield_!();
         *x1.borrow_mut() = 5;
    });

    g.next();
    assert!(*x.borrow() == 20);

    g.next();
    assert!(*x.borrow() == 5);

    assert!(g.is_done());
}

#[test]
fn test_scoped_1() {
    let mut x = 10;
    {
        let mut g = FnGenerator::<(), _>::new( || {
           x = 5;
        });
        g.next();
    }

    assert!(x == 5);
}


#[test]
fn test_inner_ref(){
    use std::mem;
    let mut g = FnGenerator::<(), &mut u32>::new(||{
        // setup something
        let mut x:u32 = 10;

        // the x memory remains on heap even returned!
        // the life time of x is assosiated with the generator
        // however modify this interal value is really unsafe
        // but this is useful pattern for setup and teardown
        // which can be put in the same place
        {
            // mut borrow block
            let y: &mut u32 = unsafe { mem::transmute(&mut x) };
            _yield_!(y);
        }
        // this was modified by the invoker
        assert!(x == 5);
        // teardown happened when the generator get dropped
        unsafe { mem::transmute(&mut x) }
    });

    // use the resource setup from generator
    let a = g.next().unwrap();
    assert!(*a == 10);
    *a = 5;
    // a keeps valid until the generator dropped
}

#[test]
fn test_drop() {
    let mut x = 10;
    {
        FnGenerator::<(), _>::new( || {
           x = 1;
           _yield_!();
           x = 5;
        });
    }

    assert!(x == 5);
}

