#[test]
fn wrap_reverse_row_single_line_different_size() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::FlexStart,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(300f32), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(30f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30f32),
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 300f32);
    assert_eq!(layout.size.height, 50f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 30f32);
    assert_eq!(layout.children[0usize].size.height, 10f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 40f32);
    assert_eq!(layout.children[1usize].size.width, 30f32);
    assert_eq!(layout.children[1usize].size.height, 20f32);
    assert_eq!(layout.children[1usize].location.x, 30f32);
    assert_eq!(layout.children[1usize].location.y, 30f32);
    assert_eq!(layout.children[2usize].size.width, 30f32);
    assert_eq!(layout.children[2usize].size.height, 30f32);
    assert_eq!(layout.children[2usize].location.x, 60f32);
    assert_eq!(layout.children[2usize].location.y, 20f32);
    assert_eq!(layout.children[3usize].size.width, 30f32);
    assert_eq!(layout.children[3usize].size.height, 40f32);
    assert_eq!(layout.children[3usize].location.x, 90f32);
    assert_eq!(layout.children[3usize].location.y, 10f32);
    assert_eq!(layout.children[4usize].size.width, 30f32);
    assert_eq!(layout.children[4usize].size.height, 50f32);
    assert_eq!(layout.children[4usize].location.x, 120f32);
    assert_eq!(layout.children[4usize].location.y, 0f32);
}