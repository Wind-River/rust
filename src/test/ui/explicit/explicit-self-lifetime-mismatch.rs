struct Foo<'a,'b> {
    x: &'a isize,
    y: &'b isize,
}

impl<'a,'b> Foo<'a,'b> {
    fn bar(self:
           Foo<'b,'a>
    //~^ ERROR mismatched `self` parameter type
    //~| expected type `Foo<'a, 'b>`
    //~| found type `Foo<'b, 'a>`
    //~| lifetime mismatch
    //~| ERROR mismatched `self` parameter type
    //~| expected type `Foo<'a, 'b>`
    //~| found type `Foo<'b, 'a>`
    //~| lifetime mismatch
           ) {}
}

fn main() {}
