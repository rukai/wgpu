extern crate env_logger;
extern crate wgpu;
extern crate wgpu_native;

fn main() {
    env_logger::init();

    for i in 0..100 {
        println!("iteration: {}", i);
        let instance = wgpu::Instance::new();
        let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
            power_preference: wgpu::PowerPreference::Default,
        });
        let device_ = adapter.create_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
        });
    }
}
