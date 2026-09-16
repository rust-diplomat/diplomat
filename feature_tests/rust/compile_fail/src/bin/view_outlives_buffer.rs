use diplomat_rust_backend_generated::SliceView;

fn main() {
    let view = {
        let data = [1u8, 2, 3, 4];
        SliceView::wrap(&data)
    };
    drop(view);
}
