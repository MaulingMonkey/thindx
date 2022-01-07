#[cfg(test)] mod tests {
    use dev::d3d9::*;

    #[test] fn evict_managed_resources() {
        let device = device_test();
        for _ in 0..1000 { device.evict_managed_resources().unwrap(); }
        // TODO: Create some Pool::Default and Pool::Managed resources
        // as I understand it, this will only evict cached copies of the latter which is only a perf thing
    }

    #[test] fn get_available_texture_mem() {
        let device = device_test();
        let available = device.get_available_texture_mem();
        assert!(available >= 1024 * 1024 * 1024); // probably a bug if our modern computer doesn't have at least 1 GiB of video mem available
    }

    #[test] fn get_clip_plane() {
        let device  = device_test();
        let _plane0 = device.get_clip_plane(0).unwrap();
        let _planen = device.get_clip_plane(!0).unwrap(); // never fails?
    }

    #[test] fn get_clip_status() {
        let device = device_test();
        let _status = device.get_clip_status().unwrap(); // never fails?
    }

    #[test] fn get_creation_parameters() {
        let device = device_test();
        let _dcp = device.get_creation_parameters().unwrap(); // never fails?
    }

    #[test] fn get_current_texture_palette() {
        let device = device_test();
        let pal = device.get_current_texture_palette().unwrap(); // never fails?
        assert!(pal == 0 || pal == 0xFFFF); // 0xFFFF on my machine - some specific "no palette" constant?
    }
}
