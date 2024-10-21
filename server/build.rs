use std::{fs::File, io::Write};
use std::path::Path;

use common_types::{Message, CurrentUser};
use schemars::schema_for;
use schemars_zod::{merge_schemas, convert};

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = Path::new(&manifest_dir).join("../app/types/server.ts");
    let mut f = File::create(out_path).unwrap();
    let combined = merge_schemas(vec![schema_for!(Message), schema_for!(CurrentUser)].into_iter());
    let converted= convert(combined);
    let types = converted.into_values().collect::<Vec<_>>().join("\n");
    let out = format!(
r#"import * as z from 'zod';
import memoizeOne from 'memoize-one';

{}
"#, types);
    f.write_all(out.as_bytes()).unwrap();
}
