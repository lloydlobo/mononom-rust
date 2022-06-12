// https://github.com/bevyengine/bevy/blob/latest/examples/2d/text2d.rs

// #[derive(Component)]
// struct AnimateTranslation;
// #[derive(Component)]
// struct AnimateRotation;
// #[derive(Component)]
// struct AnimateScale;

pub fn remove() {
    let node = vec![1, 2];
    let val = 2;
    loop {
        if !(node && node == val) {
            break;
        }
        println!("{}", node);
    }
}
