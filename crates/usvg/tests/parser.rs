// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use tiny_skia_path::Rect;
use usvg::Color;

#[test]
fn gradient_stop_offset_overflowing_f32() {
    // `4e38` overflows f32 to infinity; parsing must not panic.
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'>\
        <defs><linearGradient id='g'><stop offset='4e38'/></linearGradient></defs>\
        <rect width='1' height='1' fill='url(#g)'/>\
    </svg>";
    assert!(usvg::Tree::from_str(svg, &usvg::Options::default()).is_ok());
}

#[test]
fn clippath_with_invalid_child() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'>
        <clipPath id='clip1'>
            <rect/>
        </clipPath>
        <rect clip-path='url(#clip1)' width='10' height='10'/>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    // clipPath is invalid and should be removed together with rect.
    assert!(!tree.root().has_children());
}

#[test]
fn stylesheet_injection() {
    let svg = "<svg id='svg1' viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'>
    <style>
        #rect4 {
            fill: green
        }
    </style>
    <rect id='rect1' x='20' y='20' width='60' height='60'/>
    <rect id='rect2' x='120' y='20' width='60' height='60' fill='green'/>
    <rect id='rect3' x='20' y='120' width='60' height='60' style='fill: green'/>
    <rect id='rect4' x='120' y='120' width='60' height='60'/>
    <rect id='rect5' x='70' y='70' width='60' height='60' style='fill: green !important'/>
</svg>
";

    let stylesheet = "rect { fill: red }".to_string();

    let options = usvg::Options {
        style_sheet: Some(stylesheet),
        ..usvg::Options::default()
    };

    let tree = usvg::Tree::from_str(&svg, &options).unwrap();

    let usvg::Node::Path(first) = &tree.root().children()[0] else {
        unreachable!()
    };

    // Only the rects with no CSS attributes should be overridden.
    assert_eq!(
        first.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );

    let usvg::Node::Path(second) = &tree.root().children()[1] else {
        unreachable!()
    };
    assert_eq!(
        second.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );

    let usvg::Node::Path(third) = &tree.root().children()[2] else {
        unreachable!()
    };
    assert_eq!(
        third.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(0, 128, 0))
    );

    let usvg::Node::Path(fourth) = &tree.root().children()[3] else {
        unreachable!()
    };
    assert_eq!(
        fourth.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(0, 128, 0))
    );

    let usvg::Node::Path(fifth) = &tree.root().children()[4] else {
        unreachable!()
    };
    assert_eq!(
        fifth.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(0, 128, 0))
    );
}

#[test]
fn stylesheet_injection_with_important() {
    let svg = "<svg id='svg1' viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'>
    <style>
        #rect4 {
            fill: green
        }
    </style>
    <rect id='rect1' x='20' y='20' width='60' height='60'/>
    <rect id='rect2' x='120' y='20' width='60' height='60' fill='green'/>
    <rect id='rect3' x='20' y='120' width='60' height='60' style='fill: green'/>
    <rect id='rect4' x='120' y='120' width='60' height='60'/>
    <rect id='rect5' x='70' y='70' width='60' height='60' style='fill: green !important'/>
</svg>
";

    let stylesheet = "rect { fill: red !important }".to_string();

    let options = usvg::Options {
        style_sheet: Some(stylesheet),
        ..usvg::Options::default()
    };

    let tree = usvg::Tree::from_str(&svg, &options).unwrap();

    let usvg::Node::Path(first) = &tree.root().children()[0] else {
        unreachable!()
    };

    // All rects should be overridden, since we use `important`.
    assert_eq!(
        first.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );

    let usvg::Node::Path(second) = &tree.root().children()[1] else {
        unreachable!()
    };
    assert_eq!(
        second.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );

    let usvg::Node::Path(third) = &tree.root().children()[2] else {
        unreachable!()
    };
    assert_eq!(
        third.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );

    let usvg::Node::Path(third) = &tree.root().children()[3] else {
        unreachable!()
    };
    assert_eq!(
        third.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );

    let usvg::Node::Path(third) = &tree.root().children()[4] else {
        unreachable!()
    };
    assert_eq!(
        third.fill().unwrap().paint(),
        &usvg::Paint::Color(Color::new_rgb(255, 0, 0))
    );
}

#[test]
fn simplify_paths() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'>
        <path d='M 10 20 L 10 30 Z Z Z'/>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    let path = &tree.root().children()[0];
    match path {
        usvg::Node::Path(path) => {
            // Make sure we have MLZ and not MLZZZ
            assert_eq!(path.data().verbs().len(), 3);
        }
        _ => unreachable!(),
    };
}

#[test]
fn size_detection_1() {
    let svg = "<svg viewBox='0 0 10 20' xmlns='http://www.w3.org/2000/svg'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(10.0, 20.0).unwrap());
}

#[test]
fn size_detection_2() {
    let svg =
        "<svg width='30' height='40' viewBox='0 0 10 20' xmlns='http://www.w3.org/2000/svg'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(30.0, 40.0).unwrap());
}

#[test]
fn size_detection_3() {
    let svg =
        "<svg width='50%' height='100%' viewBox='0 0 10 20' xmlns='http://www.w3.org/2000/svg'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(5.0, 20.0).unwrap());
}

#[test]
fn size_detection_4() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg'>
        <circle cx='18' cy='18' r='18'/>
    </svg>
    ";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(36.0, 36.0).unwrap());
    assert_eq!(tree.size(), usvg::Size::from_wh(36.0, 36.0).unwrap());
}

#[test]
fn size_detection_5() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(100.0, 100.0).unwrap());
}

#[test]
fn size_detection_6() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 10' width='30'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(30.0, 30.0).unwrap());
}

#[test]
fn size_detection_7() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 20' width='30'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(30.0, 60.0).unwrap());
}

#[test]
fn size_detection_8() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 10' height='30'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(30.0, 30.0).unwrap());
}

#[test]
fn size_detection_9() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 20' height='30'/>";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(15.0, 30.0).unwrap());
}

#[test]
fn invalid_size_1() {
    let svg = "<svg width='0' height='0' viewBox='0 0 10 20' xmlns='http://www.w3.org/2000/svg'/>";
    let result = usvg::Tree::from_str(&svg, &usvg::Options::default());
    assert!(result.is_err());
}

#[test]
fn tree_is_send_and_sync() {
    fn ensure_send_and_sync<T: Send + Sync>() {}
    ensure_send_and_sync::<usvg::Tree>();
}

#[test]
fn path_transform() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
        <path transform='translate(10)' d='M 0 0 L 10 10'/>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.root().children().len(), 1);

    let group_node = &tree.root().children()[0];
    assert!(matches!(group_node, usvg::Node::Group(_)));
    assert_eq!(
        group_node.abs_transform(),
        usvg::Transform::from_translate(10.0, 0.0)
    );

    let group = match group_node {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let path = &group.children()[0];
    assert!(matches!(path, usvg::Node::Path(_)));
    assert_eq!(
        path.abs_transform(),
        usvg::Transform::from_translate(10.0, 0.0)
    );
}

#[test]
fn path_transform_nested() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
        <g transform='translate(20)'>
            <path transform='translate(10)' d='M 0 0 L 10 10'/>
        </g>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.root().children().len(), 1);

    let group_node1 = &tree.root().children()[0];
    assert!(matches!(group_node1, usvg::Node::Group(_)));
    assert_eq!(
        group_node1.abs_transform(),
        usvg::Transform::from_translate(20.0, 0.0)
    );

    let group1 = match group_node1 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let group_node2 = &group1.children()[0];
    assert!(matches!(group_node2, usvg::Node::Group(_)));
    assert_eq!(
        group_node2.abs_transform(),
        usvg::Transform::from_translate(30.0, 0.0)
    );

    let group2 = match group_node2 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let path = &group2.children()[0];
    assert!(matches!(path, usvg::Node::Path(_)));
    assert_eq!(
        path.abs_transform(),
        usvg::Transform::from_translate(30.0, 0.0)
    );
}

#[test]
fn path_transform_in_symbol_no_clip() {
    let svg = "
    <svg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'>
        <defs>
            <symbol id='symbol1' overflow='visible'>
                <rect id='rect1' x='0' y='0' width='10' height='10'/>
            </symbol>
        </defs>
        <use id='use1' xlink:href='#symbol1' x='20'/>
    </svg>
    ";

    // Will be parsed as:
    // <svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
    //     <g id="use1">
    //         <g transform="matrix(1 0 0 1 20 0)">
    //             <path fill="#000000" stroke="none" d="M 0 0 L 10 0 L 10 10 L 0 10 Z"/>
    //         </g>
    //     </g>
    // </svg>

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

    let group_node1 = &tree.root().children()[0];
    assert!(matches!(group_node1, usvg::Node::Group(_)));
    assert_eq!(group_node1.id(), "use1");
    assert_eq!(group_node1.abs_transform(), usvg::Transform::default());

    let group1 = match group_node1 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let group_node2 = &group1.children()[0];
    assert!(matches!(group_node2, usvg::Node::Group(_)));
    assert_eq!(
        group_node2.abs_transform(),
        usvg::Transform::from_translate(20.0, 0.0)
    );

    let group2 = match group_node2 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let path = &group2.children()[0];
    assert!(matches!(path, usvg::Node::Path(_)));
    assert_eq!(
        path.abs_transform(),
        usvg::Transform::from_translate(20.0, 0.0)
    );
}

#[test]
fn path_transform_in_symbol_with_clip() {
    let svg = "
    <svg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'>
        <defs>
            <symbol id='symbol1' overflow='hidden'>
                <rect id='rect1' x='0' y='0' width='10' height='10'/>
            </symbol>
        </defs>
        <use id='use1' xlink:href='#symbol1' x='20'/>
    </svg>
    ";

    // Will be parsed as:
    // <svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
    //     <defs>
    //         <clipPath id="clipPath1">
    //             <path fill="#000000" stroke="none" d="M 20 0 L 120 0 L 120 100 L 20 100 Z"/>
    //         </clipPath>
    //     </defs>
    //     <g id="use1" clip-path="url(#clipPath1)">
    //         <g>
    //             <g transform="matrix(1 0 0 1 20 0)">
    //                 <path fill="#000000" stroke="none" d="M 0 0 L 10 0 L 10 10 L 0 10 Z"/>
    //             </g>
    //         </g>
    //     </g>
    // </svg>

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

    let group_node1 = &tree.root().children()[0];
    assert!(matches!(group_node1, usvg::Node::Group(_)));
    assert_eq!(group_node1.id(), "use1");
    assert_eq!(group_node1.abs_transform(), usvg::Transform::default());

    let group1 = match group_node1 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let group_node2 = &group1.children()[0];
    assert!(matches!(group_node2, usvg::Node::Group(_)));
    assert_eq!(group_node2.abs_transform(), usvg::Transform::default());

    let group2 = match group_node2 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let group_node3 = &group2.children()[0];
    assert!(matches!(group_node3, usvg::Node::Group(_)));
    assert_eq!(
        group_node3.abs_transform(),
        usvg::Transform::from_translate(20.0, 0.0)
    );

    let group3 = match group_node3 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let path = &group3.children()[0];
    assert!(matches!(path, usvg::Node::Path(_)));
    assert_eq!(
        path.abs_transform(),
        usvg::Transform::from_translate(20.0, 0.0)
    );
}

#[test]
fn path_transform_in_svg() {
    let svg = "
    <svg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'>
        <g id='g1' transform='translate(100 150)'>
            <svg id='svg1' width='100' height='50'>
                <rect id='rect1' width='10' height='10'/>
            </svg>
        </g>
    </svg>
    ";

    // Will be parsed as:
    // <svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
    //     <defs>
    //         <clipPath id="clipPath1">
    //             <path fill="#000000" stroke="none" d="M 0 0 L 100 0 L 100 50 L 0 50 Z"/>
    //         </clipPath>
    //     </defs>
    //     <g id="g1" transform="matrix(1 0 0 1 100 150)">
    //         <g id="svg1" clip-path="url(#clipPath1)">
    //             <path id="rect1" fill="#000000" stroke="none" d="M 0 0 L 10 0 L 10 10 L 0 10 Z"/>
    //         </g>
    //     </g>
    // </svg>

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

    let group_node1 = &tree.root().children()[0];
    assert!(matches!(group_node1, usvg::Node::Group(_)));
    assert_eq!(group_node1.id(), "g1");
    assert_eq!(
        group_node1.abs_transform(),
        usvg::Transform::from_translate(100.0, 150.0)
    );

    let group1 = match group_node1 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let group_node2 = &group1.children()[0];
    assert!(matches!(group_node2, usvg::Node::Group(_)));
    assert_eq!(group_node2.id(), "svg1");
    assert_eq!(
        group_node2.abs_transform(),
        usvg::Transform::from_translate(100.0, 150.0)
    );

    let group2 = match group_node2 {
        usvg::Node::Group(g) => g,
        _ => unreachable!(),
    };

    let path = &group2.children()[0];
    assert!(matches!(path, usvg::Node::Path(_)));
    assert_eq!(
        path.abs_transform(),
        usvg::Transform::from_translate(100.0, 150.0)
    );
}

#[test]
fn svg_without_xmlns() {
    let svg = "
    <svg viewBox='0 0 100 100'>
        <rect x='0' y='0' width='10' height='10'/>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(tree.size(), usvg::Size::from_wh(100.0, 100.0).unwrap());
}

#[test]
fn image_bbox_with_parent_transform() {
    let svg = "
    <svg viewBox='0 0 200 200' 
         xmlns='http://www.w3.org/2000/svg'
         xmlns:xlink='http://www.w3.org/1999/xlink'>
        <g transform='translate(25 25)'>
            <image id='image1' x='10' y='10' width='50' height='50' xlink:href='data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAABkCAYAAABw4pVUAAABb0lEQVR4Xu3VUQ0AIAzEUOZfA87wAgkq+vGmoGlz2exz73IZAyNIpsUHEaTVQ5BYD0EEqRmI8fghgsQMxHAsRJCYgRiOhQgSMxDDsRBBYgZiOBYiSMxADMdCBIkZiOFYiCAxAzEcCxEkZiCGYyGCxAzEcCxEkJiBGI6FCBIzEMOxEEFiBmI4FiJIzEAMx0IEiRmI4ViIIDEDMRwLESRmIIZjIYLEDMRwLESQmIEYjoUIEjMQw7EQQWIGYjgWIkjMQAzHQgSJGYjhWIggMQMxHAsRJGYghmMhgsQMxHAsRJCYgRiOhQgSMxDDsRBBYgZiOBYiSMxADMdCBIkZiOFYiCAxAzEcCxEkZiCGYyGCxAzEcCxEkJiBGI6FCBIzEMOxEEFiBmI4FiJIzEAMx0IEiRmI4ViIIDEDMRwLESRmIIZjIYLEDMRwLESQmIEYjoUIEjMQw7EQQWIGYjgWIkjMQAzHQgSJGYjhWIggMQMxnAdKSlrwlejIDgAAAABJRU5ErkJggg=='/>
        </g>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

    let usvg::Node::Group(group_node1) = &tree.root().children()[0] else {
        unreachable!()
    };
    let usvg::Node::Group(group_node2) = &group_node1.children()[0] else {
        unreachable!()
    };
    let usvg::Node::Image(image_node) = &group_node2.children()[0] else {
        unreachable!()
    };

    assert_eq!(
        image_node.abs_bounding_box(),
        Rect::from_xywh(35.0, 35.0, 50.0, 50.0).unwrap()
    );
}

#[test]
fn no_text_nodes() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
        <g transform='translate(20)'>
            <path transform='translate(10)' d='M 0 0 L 10 10'/>
        </g>
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert!(!tree.has_text_nodes());
}

#[test]
fn flattened_text_should_inherit_absolute_transform() {
    let svg = "
    <svg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'>
        <g transform='translate(20 20)'>
            <g transform='translate(20 20)'>
                <text x='32' y='100'>Text</text>
            </g>
        </g>
    </svg>
    ";

    let mut opts = usvg::Options::default();
    opts.fontdb_mut()
        .load_fonts_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/../resvg/tests/fonts");
    opts.font_family = "Noto Sans".to_string();

    let tree = usvg::Tree::from_str(&svg, &opts).unwrap();

    let usvg::Node::Group(group0) = &tree.root().children()[0] else {
        unreachable!()
    };
    let usvg::Node::Group(group1) = &group0.children()[0] else {
        unreachable!()
    };
    let usvg::Node::Text(text) = &group1.children()[0] else {
        unreachable!()
    };
    let usvg::Node::Path(path) = &text.flattened().children()[0] else {
        unreachable!()
    };

    let t = path.abs_transform();

    assert_eq!(t.tx, 40.0);
    assert_eq!(t.ty, 40.0);

    assert_ne!(path.bounding_box(), path.abs_bounding_box());
    assert_eq!(
        path.bounding_box().transform(t).unwrap(),
        path.abs_bounding_box()
    );
}

#[test]
fn use_node_abs_transform() {
    let svg = "
    <svg viewBox='0 0 200 200'
         xmlns='http://www.w3.org/2000/svg'
         xmlns:xlink='http://www.w3.org/1999/xlink'>
        <defs>
            <rect id='rect1' x='0' y='0' width='100' height='100'/>
        </defs>
        <use xlink:href='#rect1' transform='matrix(0.5, 0, 0, 0.5, 20, 30)' />
    </svg>
    ";

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

    let usvg::Node::Group(group_node) = &tree.root().children()[0] else {
        unreachable!()
    };
    assert_eq!(group_node.abs_transform().get_scale(), (0.5, 0.5));

    let usvg::Node::Path(path_node) = &group_node.children()[0] else {
        unreachable!()
    };
    assert_eq!(path_node.abs_transform().get_scale(), (0.5, 0.5));
    assert_eq!(
        path_node.abs_bounding_box(),
        Rect::from_xywh(20.0, 30.0, 50.0, 50.0).unwrap()
    );
}

#[test]
fn nested_svg_abs_bounding_box() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg' width='100' height='100'>
        <svg transform='translate(10, 20)' width='50' height='50'>
            <path id='path' d='M 0 0 L 10 10'/>
        </svg>
    </svg>
    ";
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    assert_eq!(
        tree.node_by_id("path").unwrap().abs_bounding_box(),
        Rect::from_xywh(10.0, 20.0, 10.0, 10.0).unwrap()
    );
}

#[test]
fn axis_aligned_stroke_has_layer_bounds() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>\
        <path id='horizontal' d='M 10 50 H 90' fill='none' stroke='black' stroke-width='4'/>\
        <path id='vertical' d='M 50 10 V 90' fill='none' stroke='black' stroke-width='4'/>\
    </svg>";

    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).unwrap();
    for id in ["horizontal", "vertical"] {
        assert!(
            tree.node_by_id(id)
                .unwrap()
                .abs_layer_bounding_box()
                .is_some()
        );
    }
}

fn find_text(group: &usvg::Group) -> Option<&usvg::Text> {
    for node in group.children() {
        match node {
            usvg::Node::Group(child) => {
                if let Some(text) = find_text(child) {
                    return Some(text);
                }
            }
            usvg::Node::Text(text) => {
                return Some(text);
            }
            usvg::Node::Path(_) | usvg::Node::Image(_) => {}
        }
    }
    None
}

fn first_glyph_font_size(tree: &usvg::Tree) -> f32 {
    find_text(tree.root())
        .unwrap()
        .layouted()
        .first()
        .unwrap()
        .positioned_glyphs
        .first()
        .unwrap()
        .font_size()
}

fn node_path_width(tree: &usvg::Tree, id: &str) -> f32 {
    let usvg::Node::Path(path) = tree.node_by_id(id).unwrap() else {
        unreachable!()
    };
    path.bounding_box().width()
}

fn glyph_font_families(tree: &usvg::Tree) -> Vec<(String, String)> {
    let text_node = find_text(tree.root()).unwrap();
    text_node
        .layouted()
        .iter()
        .flat_map(|span| &span.positioned_glyphs)
        .map(|glyph| {
            let family = tree
                .fontdb()
                .face(glyph.font)
                .unwrap()
                .families
                .first()
                .unwrap()
                .0
                .clone();
            (glyph.text.clone(), family)
        })
        .collect()
}

fn first_glyph_font_family(tree: &usvg::Tree, text: &str) -> String {
    glyph_font_families(tree)
        .into_iter()
        .find(|(glyph_text, _)| glyph_text == text)
        .unwrap()
        .1
}

fn bundled_noto_options() -> usvg::Options<'static> {
    let mut options = usvg::Options {
        font_family: "Noto Sans".to_string(),
        font_size: 16.0,
        ..usvg::Options::default()
    };
    options
        .fontdb_mut()
        .load_font_data(include_bytes!("../../resvg/tests/fonts/NotoSans-Regular.ttf").to_vec());
    options
}

fn parse_text_with_font_size(font_size: &str) -> usvg::Tree {
    let svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg'><g font-family='Noto Sans' font-size='16px'><text font-size='{font_size}'>x</text><rect id='ex-box' width='4ex' height='1'/></g></svg>"
    );
    usvg::Tree::from_str(&svg, &bundled_noto_options()).unwrap()
}

fn expected_noto_ex_size(multiplier: f32) -> f32 {
    let font_data = include_bytes!("../../resvg/tests/fonts/NotoSans-Regular.ttf");
    let face = ttf_parser::Face::parse(font_data, 0).unwrap();
    multiplier * 16.0 * f32::from(face.x_height().unwrap()) / f32::from(face.units_per_em())
}

#[test]
fn ex_font_size_uses_bundled_font_x_height() {
    let ex_tree = parse_text_with_font_size("4ex");
    let px_tree = parse_text_with_font_size("16px");

    let expected_size = expected_noto_ex_size(4.0);

    assert!((first_glyph_font_size(&ex_tree) - expected_size).abs() < 0.01);
    assert!((node_path_width(&ex_tree, "ex-box") - expected_size).abs() < 0.01);
    assert_eq!(first_glyph_font_size(&px_tree), 16.0);
    assert_eq!(
        first_glyph_font_size(&parse_text_with_font_size("1em")),
        16.0
    );
    assert_eq!(
        first_glyph_font_size(&parse_text_with_font_size("100%")),
        16.0
    );
}

#[test]
fn root_ex_font_size_uses_options_font_metrics() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' width='200' height='100' font-family='Noto Sans' font-size='5ex'><text x='10' y='80'>x</text></svg>";
    let tree = usvg::Tree::from_str(svg, &bundled_noto_options()).unwrap();

    assert!((first_glyph_font_size(&tree) - expected_noto_ex_size(5.0)).abs() < 0.01);
}

#[test]
fn fallback_keeps_primary_font_for_latin_glyphs() {
    let primary_data = include_bytes!("../../resvg/tests/fonts/NotoSans-Regular.ttf");
    let fallback_data = include_bytes!("../../resvg/tests/fonts/MPLUS1p-Regular.ttf");
    let primary_face = ttf_parser::Face::parse(primary_data, 0).unwrap();
    let fallback_face = ttf_parser::Face::parse(fallback_data, 0).unwrap();
    assert!(primary_face.glyph_index('A').is_some());
    assert!(primary_face.glyph_index('中').is_none());
    assert!(fallback_face.glyph_index('中').is_some());

    let svg = "<svg xmlns='http://www.w3.org/2000/svg' width='200' height='100'><text x='10' y='40' font-family='Noto Sans, Mplus 1p'>A中</text></svg>";
    let mut options = bundled_noto_options();
    options.fontdb_mut().load_font_data(fallback_data.to_vec());
    let tree = usvg::Tree::from_str(svg, &options).unwrap();

    assert_eq!(first_glyph_font_family(&tree, "A"), "Noto Sans");
    assert_eq!(first_glyph_font_family(&tree, "中"), "Mplus 1p");
}

#[test]
fn fallback_keeps_combining_cluster_in_one_font() {
    let primary_data = include_bytes!("../../resvg/tests/fonts/NotoSans-Regular.ttf");
    let fallback_data = include_bytes!("../../resvg/tests/fonts/Amiri-Regular.ttf");
    let primary_face = ttf_parser::Face::parse(primary_data, 0).unwrap();
    let fallback_face = ttf_parser::Face::parse(fallback_data, 0).unwrap();
    assert!(primary_face.glyph_index('A').is_some());
    assert!(primary_face.glyph_index('\u{0651}').is_none());
    assert!(fallback_face.glyph_index('A').is_some());
    assert!(fallback_face.glyph_index('\u{0651}').is_some());

    let svg = "<svg xmlns='http://www.w3.org/2000/svg' width='200' height='100'><text x='10' y='50' font-family='Noto Sans, Amiri'>A\u{0651}</text></svg>";
    let mut options = bundled_noto_options();
    options.fontdb_mut().load_font_data(fallback_data.to_vec());
    let tree = usvg::Tree::from_str(svg, &options).unwrap();
    let glyphs = glyph_font_families(&tree);
    assert!(!glyphs.is_empty());
    assert!(glyphs.iter().all(|(_, family)| family == "Amiri"));
}

fn contains_arabic(text: &str) -> bool {
    text.chars()
        .any(|character| ('\u{0600}'..='\u{06ff}').contains(&character))
}

#[test]
fn rtl_fixture_keeps_latin_primary_and_uses_arabic_fallback() {
    let noto_data = include_bytes!("../../resvg/tests/fonts/NotoSans-Regular.ttf");
    let amiri_data = include_bytes!("../../resvg/tests/fonts/Amiri-Regular.ttf");
    let noto_face = ttf_parser::Face::parse(noto_data, 0).unwrap();
    let amiri_face = ttf_parser::Face::parse(amiri_data, 0).unwrap();
    for character in ['S', 'V', 'G'] {
        assert!(noto_face.glyph_index(character).is_some());
    }
    assert!(noto_face.glyph_index('ق').is_none());
    assert!(amiri_face.glyph_index('S').is_some());
    assert!(amiri_face.glyph_index('ق').is_some());

    let mut options = bundled_noto_options();
    options.fontdb_mut().load_font_data(amiri_data.to_vec());
    let tree = usvg::Tree::from_str(
        include_str!("../../resvg/tests/tests/text/direction/rtl.svg"),
        &options,
    )
    .unwrap();
    let glyphs = glyph_font_families(&tree);

    for character in ["S", "V", "G"] {
        assert!(
            glyphs
                .iter()
                .any(|(text, family)| text.contains(character) && family == "Noto Sans")
        );
    }
    let arabic = glyphs
        .iter()
        .filter(|(text, _)| contains_arabic(text))
        .collect::<Vec<_>>();
    assert!(!arabic.is_empty());
    assert!(arabic.iter().all(|(_, family)| family == "Amiri"));
}
