use std::cell::RefCell;
use std::rc::Rc;
use kaitai::*;

#[derive(Default, Debug, PartialEq)]
struct RootStruct {
    child: RefCell<SharedType<ChildStruct>>,
    data: RefCell<u32>,
}

impl<'r, 's: 'r> KStruct<'r, 's> for RootStruct {
    type Root = Self;
    type Parent = Self;

    fn read<S: KStream>(
        self_rc: &Rc<Self>,
        io: &'s S,
        root: SharedType<Self::Root>,
        parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc.data.borrow_mut() = 1;
        let x = ChildStruct::read_into(io, Some(root.clone()), Some(parent.clone()))?;
        *self_rc.child.borrow_mut() = SharedType::new(x);
        Ok(())
    }
}

#[test]
fn same_parent() {
    let b = [];
    let reader = BytesReader::new(&b[..]);
    let root_struct: Rc<RootStruct> = RootStruct::read_into(&reader, None, None).unwrap();
    let child = root_struct.as_ref().child.borrow().get().unwrap();
    let parent = child.parent.borrow().get().unwrap();
    assert_eq!(*root_struct.as_ref(), *parent.as_ref());
}

#[derive(Default, Debug, PartialEq)]
struct ChildStruct {
    parent: RefCell<SharedType<RootStruct>>,
    child2: RefCell<SharedType<ChildStruct2>>,
    data: RefCell<u32>,
}

impl<'r, 's: 'r> KStruct<'r, 's> for ChildStruct {
    type Root = RootStruct;
    type Parent = RootStruct;

    fn read<S: KStream>(
        self_rc: &Rc<Self>,
        _io: &'s S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc.parent.borrow_mut() = _parent;
        *self_rc.data.borrow_mut() = 2;
        //self.read(_io, SharedType::<Self::Root>::new(_root.get()), _parent);
        let parent = SharedType::<Self>::new(self_rc.clone());
        let x = ChildStruct2::read_into(_io, Some(_root), Some(parent.clone()))?;
        *self_rc.child2.borrow_mut() = SharedType::new(x);

        Ok(())
    }
}

//////////////////////////////////////////////////
#[derive(Default, Debug, PartialEq)]
struct ChildStruct2 {
    parent: RefCell<SharedType<ChildStruct>>,
    data: RefCell<u32>,
}

impl<'r, 's: 'r> KStruct<'r, 's> for ChildStruct2 {
    type Root = RootStruct;
    type Parent = ChildStruct;

    fn read<S: KStream>(
        self_rc: &Rc<Self>,
        _io: &'s S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc.data.borrow_mut() = 3;
        //self.get().read(_io, _root, _parent)
        Ok(())
    }
}

#[test]
fn root_is_parent() {
    let b = [];
    let reader = BytesReader::new(&b[..]);
    let root_struct: Rc<RootStruct> = RootStruct::read_into(&reader, None, None).unwrap();
    println!("{}", *root_struct.data.borrow());
    println!("{}", *root_struct.child.borrow().get().unwrap().data.borrow());
    println!("{}", *root_struct.child.borrow().get().unwrap().child2.borrow().get().unwrap().data.borrow());

    // let ors = Some(root_struct.clone());
    // let child_struct: ChildStruct = ChildStruct::read_into(&reader, ors, Some(&*root_struct.clone())).unwrap();

    // dbg!(&child_struct);
    // assert_eq!(*child_struct.parent.borrow(), *root_struct);
    // assert_eq!(**child_struct.parent.borrow().child.borrow().as_ref().unwrap(), child_struct);
}
/*
    #[derive(Default, Debug, Clone)]
    struct GrandChildStruct {
        parent: RefCell<ChildStruct>,
    }
    impl<'r, 's: 'r> KStruct<'r, 's> for GrandChildStruct {
        type Root = RootStruct;
        type Parent = ChildStruct;
        fn read<S: KStream>(
                &self,
                _io: &'s S,
                _root: Option<SharedType<Self::Root>>,
                _parent: Option<SharedType<Self::Parent>>,
            ) -> KResult<()> {
                if let Some(parent) = _parent {
                    *self.parent.borrow_mut() = parent.clone();
                }
                Ok(())
        }
    }
    #[test]
    fn child_is_parent() {
        let b = [];
        let reader = BytesReader::new(&b[..]);
        let root_struct = Rc::<RootStruct>::new(RootStruct::read_into(&reader, None, None).unwrap());
        let child_struct: ChildStruct = ChildStruct::read_into(&reader,Some(root_struct.clone ()), Some(&*root_struct.clone())).unwrap();
        let grand_child_struct: GrandChildStruct = GrandChildStruct::read_into(&reader, Some(root_struct.clone()), Some(&child_struct.clone())).unwrap();
        assert_eq!(*child_struct.parent.borrow(), *root_struct);
    }
*/
