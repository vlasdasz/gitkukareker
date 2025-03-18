use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: labirintas::test_engine::AndroidApp) {
    labirintas::test_engine::test_engine_start_app(app);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_labirintas_MainActivity_setFilesDir<'local>(
    mut env: JNIEnv<'local>,
    _: JClass,
    input: JString<'local>,
) {
    use labirintas::test_engine::store::Paths;
    let input: String = env.get_string(&input).expect("Couldn't get java string!").into();
    Paths::set_storage_path(input);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_labirintas_MainActivity_setAssetManager<'local>(
    _env: JNIEnv<'local>,
    _: JClass,
    _input: JClass,
) {
    dbg!("Java_com_example_labirintas_MainActivity_setAssetManager");
}
