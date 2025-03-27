/// The `bevy_stylebox` is plugin for [bevy](https://bevyengine.org/) engine which
/// allows you to fill UI node with sliced by 9 parts region of image. `Stylebox`
/// doesn't add any additional UI components. It renders just like `UiImage`, but
/// generates more vertices in the rendering system. Only `stretch` mode is
/// supported for now for drawing edges, `repeat` & `round` coming soon.
use bevy::{
    prelude::*,
    render::{Extract, RenderApp, sync_world::MainEntity},
    ui::{ExtractedUiNode, ExtractedUiNodes, ExtractedUiItem, FocusPolicy, RenderUiSystem, UiStack, ResolvedBorderRadius},
};

/// `Stylebox` plugin for `bevy` engine. Dont forget to register it:
/// ```rust
/// use bevy::prelude::*;
/// use bevy_stylebox::*;
///
/// fn main() {
///    let mut app = App::new();
///    app.add_plugins(DefaultPlugins);
///    app.add_plugins(StyleboxPlugin);
/// }
/// ```
pub struct StyleboxPlugin;

const EPSILON: f32 = 0.000005;
const TWO_EPSILONS: f32 = EPSILON + EPSILON;
const ONE_MINUS_EPSILON: f32 = 1.0 - EPSILON;
const ONE_MINUS_TWO_EPSILONS: f32 = ONE_MINUS_EPSILON - EPSILON;

impl Plugin for StyleboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, compute_stylebox_configuration)
            .add_systems(PostUpdate, compute_stylebox_slices)
            .sub_app_mut(RenderApp)
            .add_systems(
                ExtractSchedule,
                extract_stylebox.after(RenderUiSystem::ExtractText),
            );
    }
}
#[derive(Bundle, Clone, Debug, Default)]
/// The bundle with almost the same content a `NodeBundle`,
/// but `Stylebox` component is used instead of `BackgroundColor`.
pub struct StyleboxBundle {
    /// The stylebox of the node
    pub stylebox: Stylebox,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub inherited_visibility: InheritedVisibility,
}

#[derive(Component, Clone, Debug)]
#[require(Node)]
/// Component used to specify how to fill the element with sliced by 9 parts region of image.
pub struct Stylebox {
    /// holds the handle to the image to be used as a stylebox
    pub texture: Handle<Image>,
    /// specifies how to slice the image region specified by texture & region
    /// The image is always sliced into nine sections: four corners, four edges and the middle.
    /// - when `Val::Px` specified, region sliced to the exact amount of pixels
    /// - when `Val::Percent` specified, image region sliced relative to it size
    /// - `Val::Auto` treated as `Val::Percent(50.)`
    pub slice: UiRect,
    /// specifies the width of the edgets of the sliced region:
    /// - edges specified by `Val::Px` values resizes to exact amout of pixels
    /// - edges specified by `Val::Percent` resized relative to width provided by `slice` property
    /// - `Val::Auto` treated as `Val::Percent(100.)`
    ///
    /// Default value for `width` is `Val::Percent(100.)`: use width provided by `slice` property.
    pub width: UiRect,
    /// specifies which region of the image should be sliced.
    /// By default the hole area of image defined by `texture` is used.
    /// - `Val::Px` values defines exact offset from the image edges in pixels
    /// - `Val::Percent` values defines offset from the image edges relative to the image size
    /// - `Val::Auto` treated as `Val::Px(0.)`
    ///
    /// Default value for `region` is `Val::Px(0.)`
    pub region: UiRect,
    /// specifies what color the original image should be multiplied by
    pub modulate: Color,
}

impl Default for Stylebox {
    fn default() -> Self {
        Stylebox {
            texture: Handle::<Image>::default(),
            slice: UiRect::default(),
            width: UiRect::all(Val::Percent(100.)),
            modulate: Color::WHITE,
            region: UiRect::all(Val::Px(0.)),
        }
    }
}

impl std::fmt::Display for Stylebox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stylebox (\n  slice: {:?},\n  width: {:?},\n  region: {:?}\n)",
            self.slice, self.width, self.region
        )
    }
}

#[derive(Default, Debug)]
struct UiRectF32 {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl UiRectF32 {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> UiRectF32 {
        UiRectF32 {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Component, Default)]
/// Component which holds calculated sizes based on `slice`, `width` and `region` provided
/// by `StyleBox`.
pub struct ComputedStylebox {
    // slice rect relative to
    // region size in 0..1 space
    slice: UiRectF32,

    // edges width relative to slices edges
    width: UiRectF32,
    region: Rect,
}
/// Calculates exact values for `Stylebox` when image size is available and stores it into `ComputedStylebox`.
pub fn compute_stylebox_configuration(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut styleboxes: Query<
        (Entity, &Stylebox, Option<&mut ComputedStylebox>),
        Or<(Changed<Stylebox>, Without<ComputedStylebox>)>,
    >,
) {
    for (entity, stylebox, computed) in styleboxes.iter_mut() {
        if stylebox.texture == Handle::<Image>::default() {
            if computed.is_none() {
                commands
                    .entity(entity)
                    .insert(StyleboxSlices::default())
                    .insert(ComputedStylebox::default());
                continue;
            }
        }
        match images.get(&stylebox.texture) {
            None => {
                if computed.is_some() {
                    commands
                        .entity(entity)
                        .remove::<StyleboxSlices>()
                        .remove::<ComputedStylebox>();
                }
            }
            Some(image) => {
                let size = image.size();

                let (size_x, size_y) = (size.x as f32, size.y as f32);

                let region_left = match stylebox.region.left {
                    Val::Percent(percent) => size_x * percent * 0.01,
                    Val::Px(px) => px,
                    _ => 0.,
                };
                let region_right = match stylebox.region.right {
                    Val::Percent(percent) => size_x * percent * 0.01,
                    Val::Px(px) => px,
                    _ => 0.,
                };
                let region_top = match stylebox.region.top {
                    Val::Percent(percent) => size_y * percent * 0.01,
                    Val::Px(px) => px,
                    _ => 0.,
                };
                let region_bottom = match stylebox.region.bottom {
                    Val::Percent(percent) => size_y * percent * 0.01,
                    Val::Px(px) => px,
                    _ => 0.,
                };
                let region = Rect {
                    min: Vec2::new(region_left, region_top),
                    max: Vec2::new(
                        (size_x - region_right).max(region_left),
                        (size_y - region_bottom).max(region_top),
                    ),
                };

                let region_size = region.size();
                let (region_width, region_height) = (region_size.x, region_size.y);

                let slice_left = match stylebox.slice.left {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / region_width,
                    _ => 0.5,
                };
                let slice_right = match stylebox.slice.right {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / region_width,
                    _ => 0.5,
                };
                let slice_top = match stylebox.slice.top {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / region_height,
                    _ => 0.5,
                };
                let slice_bottom = match stylebox.slice.bottom {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / region_height,
                    _ => 0.5,
                };

                let width_left = match stylebox.width.left {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / (region_width * slice_left),
                    _ => 1.0,
                };
                let width_right = match stylebox.width.right {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / (region_width * slice_right),
                    _ => 1.0,
                };
                let width_top = match stylebox.width.top {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / (region_height * slice_top),
                    _ => 1.0,
                };
                let width_bottom = match stylebox.width.bottom {
                    Val::Percent(percent) => percent * 0.01,
                    Val::Px(px) => px / (region_height * slice_bottom),
                    _ => 1.0,
                };

                let computed_stylebox = ComputedStylebox {
                    slice: UiRectF32::new(slice_left, slice_right, slice_top, slice_bottom),
                    width: UiRectF32::new(width_left, width_right, width_top, width_bottom),
                    region,
                };
                if let Some(mut computed) = computed {
                    *computed = computed_stylebox;
                } else {
                    commands
                        .entity(entity)
                        .insert(StyleboxSlices::default())
                        .insert(computed_stylebox);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct StyleboxSlice {
    transform: Mat4,
    region: Rect,
}

#[derive(Component, Default)]
/// Holds transform for each slice
pub struct StyleboxSlices {
    items: Vec<StyleboxSlice>,
}
/// Calculates transforms for each slice based on `ComputedNode.size()` and `ComputedStylebox`
pub fn compute_stylebox_slices(
    mut query: Query<
        (&mut StyleboxSlices, &ComputedNode, &ComputedStylebox),
        Or<(Changed<ComputedNode>, Changed<ComputedStylebox>)>,
    >,
) {
    for (mut transforms, uinode, stylebox) in query.iter_mut() {
        if uinode.size() == Vec2::ZERO {
            continue;
        }
        transforms.items.clear();
        let size = uinode.size();
        let region = stylebox.region;
        let rpos = region.min;
        let rsize = region.size();

        let left = stylebox.slice.left;
        let right = stylebox.slice.right;
        let top = stylebox.slice.top;
        let bot = stylebox.slice.bottom;

        // compute part sizes in uinode space
        let w0 = left * rsize.x * stylebox.width.left;
        let w2 = right * rsize.x * stylebox.width.right;
        let w1 = size.x - w0 - w2;
        let h0 = top * rsize.y * stylebox.width.top;
        let h2 = bot * rsize.y * stylebox.width.bottom;
        let h1 = size.y - h0 - h2;

        let ui_x = &[0., w0, w0 + w1];
        let ui_y = &[0., h0, h0 + h1];
        let ui_width = &[w0, w1, w2];
        let ui_height = &[h0, h1, h2];

        let (tx0, tx1) = normalize_axis(left, right);
        let (ty0, ty1) = normalize_axis(top, bot);
        let tex_x = &[rpos.x, rpos.x + tx0 * rsize.x, rpos.x + tx1 * rsize.x, rpos.x + rsize.x];
        let tex_y = &[rpos.y, rpos.y + ty0 * rsize.y, rpos.y + ty1 * rsize.y, rpos.y + rsize.y];
        let tex_width = &[tx0 * rsize.x, (tx1 - tx0) * rsize.x, (1. - tx1) * rsize.x];
        let tex_height = &[ty0 * rsize.y, (ty1 - ty0) * rsize.y, (1. - ty1) * rsize.y];

        for y in 0..3 {
            for x in 0..3 {
                let scale_x = if tex_width[x] < EPSILON {
                    0.
                } else {
                    ui_width[x] / tex_width[x]
                };
                let scale_y = if tex_height[y] < EPSILON {
                    0.
                } else {
                    ui_height[y] / tex_height[y]
                };

                let transform = Mat4::from_scale_rotation_translation(
                    Vec3::new(scale_x, scale_y, 1.),
                    Quat::IDENTITY,
                    Vec3::new(
                        ui_x[x] - tex_x[x] * scale_x,
                        ui_y[y] - tex_y[y] * scale_y,
                        0.,
                    ),
                );

                let region = Rect {
                    min: Vec2::new(tex_x[x], tex_y[y]),
                    max: Vec2::new(tex_x[x + 1], tex_y[y + 1]),
                };
                transforms.items.push(StyleboxSlice { transform, region });
            }
        }
    }
}

fn normalize_axis(left: f32, right: f32) -> (f32, f32) {
    if left + right > ONE_MINUS_EPSILON {
        // when left + right > 1, we need to scale them down to fit in 0..1 range
        // we want to keep the ratio between left and right
        // and we want to make sure that left + right = 1 - EPSILON
        let sum = left + right;
        let scale = ONE_MINUS_EPSILON / sum;
        let left = left * scale;
        let right = right * scale;
        let t0 = left;
        let t1 = ONE_MINUS_EPSILON - right;
        (t0, t1)
    } else {
        // when left + right <= 1, we can use them as is
        // we want to make sure that left + middle + right = 1
        // and we want to keep the ratio between left and right
        // and we want to make sure that middle is at least EPSILON
        let middle = 1. - left - right;
        let t0 = left;
        let t1 = left + middle;
        (t0, t1)
    }
}

/// Extracts stylebox vertices into render pipeline based on `Stylebox.texture`,
/// `Stylebox.modulate` and `StyleboxSlices`
pub fn extract_stylebox(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    ui_stack: Extract<Res<UiStack>>,
    images: Extract<Res<Assets<Image>>>,
    uinode_query: Extract<
        Query<(
            &ComputedNode,
            &GlobalTransform,
            &Stylebox,
            &StyleboxSlices,
            &InheritedVisibility,
            Option<&CalculatedClip>,
        )>,
    >,
) {
    for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
        let Ok((uinode, transform, stylebox, slices, visibility, clip)) = uinode_query.get(*entity)
        else {
            continue;
        };

        if !visibility.get() {
            continue;
        }
        let image = stylebox.texture.clone_weak();
        // Skip unloaded images
        if !images.contains(&image) {
            continue;
        }

        if uinode.size() == Vec2::ZERO {
            continue;
        }

        let img = images.get(&image).unwrap();
        let tr = transform.compute_matrix();
        let img_size = img.size();
        let img_size = Vec2::new(img_size.x as f32, img_size.y as f32);

        for patch in slices.items.iter() {
            extracted_uinodes.uinodes.insert(
                *entity,
                ExtractedUiNode {
                    stack_index: stack_index as u32,
                    color: stylebox.modulate.to_linear(),
                    rect: patch.region,
                    image: image.id(),
                    clip: clip.map(|clip| clip.clip),
                    camera_entity: Entity::PLACEHOLDER,
                    item: ExtractedUiItem::Node {
                        border_radius: ResolvedBorderRadius::default(),
                        border: BorderRect::default(),
                        node_type: bevy::ui::NodeType::Rect,
                        transform: tr * patch.transform,
                        flip_x: false,
                        flip_y: false,
                        atlas_scaling: Some(img_size),
                    },
                    main_entity: MainEntity::from(*entity),
                },
            );

            // extracted_uinodes.uinodes();
        }
    }
}
