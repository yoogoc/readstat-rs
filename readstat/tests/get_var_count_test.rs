use dunce;
use readstat;
use readstat_sys;
use std::env;
use std::path::Path;

#[test]
fn get_var_count() {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let data_dir = project_dir.parent().unwrap().join("data");
    let sas_path = dunce::canonicalize(data_dir.join("cars.sas7bdat")).unwrap();

    let mut md = readstat::ReadStatMetadata::new().set_path(sas_path);
    let error = md.get_metadata().unwrap();

    assert_eq!(error, readstat_sys::readstat_error_e_READSTAT_OK);

    assert_eq!(md.var_count, 13);
}