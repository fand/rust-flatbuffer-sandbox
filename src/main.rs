use std::fs;

use anyhow;
use fbs::test_generated::{FbsBar, FbsBarArgs, FbsFoo, FbsFooArgs};
use flatbuffers::{self, FlatBufferBuilder, WIPOffset};

mod fbs;

struct Foo {
    name: String,
    a: i32,
}

struct Bar {
    name: String,
}

fn foo(name: &str, a: i32) -> Foo {
    Foo {
        name: name.to_string(),
        a,
    }
}
fn bar(name: &str) -> Bar {
    Bar {
        name: name.to_string(),
    }
}

fn foo_to_fbs<'a, 'fbb>(fbb: &'a mut FlatBufferBuilder<'fbb>, foo: Foo) -> WIPOffset<FbsFoo<'fbb>> {
    let name = Some(fbb.create_string(&foo.name));
    FbsFoo::create(fbb, &FbsFooArgs { name, a: foo.a })
}

fn bar_to_fbs<'a, 'fbb>(fbb: &'a mut FlatBufferBuilder<'fbb>, foo: Bar) -> WIPOffset<FbsBar<'fbb>> {
    let name = Some(fbb.create_string(&foo.name));
    FbsBar::create(fbb, &FbsBarArgs { name })
}

fn main() -> anyhow::Result<()> {
    // Serialize
    let bar1 = bar("bar");

    let mut fbb = flatbuffers::FlatBufferBuilder::new();
    let offset = bar_to_fbs(&mut fbb, bar1);
    fbb.finish(offset, None);
    fs::write("./out.bin", fbb.finished_data())?;

    Ok(())
}
