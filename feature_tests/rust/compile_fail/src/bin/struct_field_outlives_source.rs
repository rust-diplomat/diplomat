use diplomat_rust_backend_generated::SliceView;

fn main() {
    let fields = {
        let data = [1u8, 2, 3, 4];
        let view = SliceView::wrap(&data);
        view.fields()
    };
    drop(fields);
}
