use std::{env, fs, path::Path};
use xdgen::{App, Context, FluentString};

fn main() {
    // Rebuild if i18n files change
    println!("cargo:rerun-if-changed=i18n");

    let ctx = Context::new("i18n", env::var("CARGO_PKG_NAME").unwrap()).unwrap();
    let app = App::new(FluentString("app-title"))
        .comment(FluentString("app-comment"))
        .keywords(FluentString("app-keywords"));

    let desktop_entry = app
        .expand_desktop("resources/org.cosmic_utils.enroll.desktop", &ctx)
        .unwrap();
    let metainfo = app
        .expand_metainfo("resources/org.cosmic_utils.enroll.metainfo.xml", &ctx)
        .unwrap();

    let output = Path::new("target/xdgen/");
    fs::create_dir_all(output).unwrap();
    fs::write(
        output.join("org.cosmic_utils.enroll.desktop"),
        desktop_entry,
    )
    .unwrap();
    fs::write(
        output.join("org.cosmic_utils.enroll.metainfo.xml"),
        metainfo,
    )
    .unwrap();

    println!("cargo:rerun-if-env-changed=VERGEN_GIT_COMMIT_DATE");
    println!("cargo:rerun-if-env-changed=VERGEN_GIT_SHA");
}
