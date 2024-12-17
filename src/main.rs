use std::fs;

use anyhow;
use fbs::test_generated::{FbsBar, FbsBarArgs, FbsFoo, FbsFooArgs};
use flatbuffers::{self, FlatBufferBuilder, WIPOffset};

mod fbs;

#[derive(Clone, Debug)]
struct Foo {
    name: String,
    a: i32,
}

#[derive(Clone, Debug)]
struct Bar {
    name: String,
    foo: Vec<Foo>,
}

fn foo(name: &str, a: i32) -> Foo {
    Foo {
        name: name.to_string(),
        a,
    }
}
fn bar(name: &str, foo: &Vec<Foo>) -> Bar {
    Bar {
        name: name.to_string(),
        foo: foo.clone(),
    }
}

fn foo_to_fbs<'a, 'fbb>(fbb: &'a mut FlatBufferBuilder<'fbb>, foo: Foo) -> WIPOffset<FbsFoo<'fbb>> {
    let name = Some(fbb.create_string(&foo.name));
    FbsFoo::create(fbb, &FbsFooArgs { name, a: foo.a })
}

fn bar_to_fbs<'a, 'fbb>(fbb: &'a mut FlatBufferBuilder<'fbb>, bar: Bar) -> WIPOffset<FbsBar<'fbb>> {
    let name = Some(fbb.create_string(&bar.name));

    let items = bar
        .foo
        .into_iter()
        .map(|f| foo_to_fbs(fbb, f))
        .collect::<Vec<_>>();
    let foo = Some(fbb.create_vector(&items));

    FbsBar::create(fbb, &FbsBarArgs { name, foo })
}

fn main() -> anyhow::Result<()> {
    // Serialize
    // let bar1 = bar("bar");

    // let mut fbb = flatbuffers::FlatBufferBuilder::new();
    // let offset = bar_to_fbs(&mut fbb, bar1);
    // fbb.finish(offset, None);
    // fs::write("./out.bin", fbb.finished_data())?;

    // Deserialize
    let bin = include_bytes!("../out.bin");

    let opts = flatbuffers::VerifierOptions {
        max_depth: 10,
        ..Default::default()
    };
    let data = flatbuffers::root_with_opts::<FbsBar>(&opts, bin)?;
    dbg!(data);

    Ok(())
}
