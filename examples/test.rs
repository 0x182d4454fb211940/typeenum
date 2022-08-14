use typeenum::HasVariant;

#[derive(HasVariant)]
enum Many {
    String(String),
    I32(i32),
    USize { x: usize },
}

fn main() {
    let x: Many = String::new().into();
    let y: Many = 3i32.into();
    let z: Many = 3usize.into();

    assert_eq!(x.get(), Some(&String::new()));
    assert_eq!(x.get(), Option::<&i32>::None);
    assert_eq!(x.get(), Option::<&usize>::None);

    assert_eq!(y.get(), Option::<&String>::None);
    assert_eq!(y.get(), Some(&3i32));
    assert_eq!(y.get(), Option::<&usize>::None);

    assert_eq!(z.get(), Option::<&String>::None);
    assert_eq!(z.get(), Option::<&i32>::None);
    assert_eq!(z.get(), Some(&3usize));
}
