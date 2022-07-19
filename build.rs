use windres::Build;

fn main() {
  Build::new().compile("icon.rc").unwrap();
}
