pub async fn create_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    adapter
        .request_device(&Default::default(), None)
        .await
        .unwrap()
}
fn main() {
    env_logger::init();
    let (device, _queue) = futures::executor::block_on(create_device());
    let _module = device.create_shader_module(&wgpu::include_spirv!("module.spv"));
}
