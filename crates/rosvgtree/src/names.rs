// This file is autogenerated. Do not edit it!
// See ./codegen for details.

/// An element ID.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum ElementId {
    A,
    Circle,
    ClipPath,
    Defs,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    G,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Tref,
    Tspan,
    Use
}

static ELEMENTS: Map<ElementId> = Map {
    key: 732231254413039614,
    disps: &[
        (0, 12),
        (1, 11),
        (10, 26),
        (2, 42),
        (1, 19),
        (0, 5),
        (1, 13),
        (8, 50),
        (0, 0),
        (1, 0),
        (7, 45),
    ],
    entries: &[
        ("feFlood", ElementId::FeFlood),
        ("radialGradient", ElementId::RadialGradient),
        ("feImage", ElementId::FeImage),
        ("stop", ElementId::Stop),
        ("fePointLight", ElementId::FePointLight),
        ("feConvolveMatrix", ElementId::FeConvolveMatrix),
        ("feComposite", ElementId::FeComposite),
        ("clipPath", ElementId::ClipPath),
        ("feMerge", ElementId::FeMerge),
        ("defs", ElementId::Defs),
        ("mask", ElementId::Mask),
        ("svg", ElementId::Svg),
        ("symbol", ElementId::Symbol),
        ("linearGradient", ElementId::LinearGradient),
        ("feSpecularLighting", ElementId::FeSpecularLighting),
        ("feFuncB", ElementId::FeFuncB),
        ("filter", ElementId::Filter),
        ("feFuncG", ElementId::FeFuncG),
        ("circle", ElementId::Circle),
        ("g", ElementId::G),
        ("tref", ElementId::Tref),
        ("feFuncA", ElementId::FeFuncA),
        ("image", ElementId::Image),
        ("text", ElementId::Text),
        ("line", ElementId::Line),
        ("pattern", ElementId::Pattern),
        ("use", ElementId::Use),
        ("feDropShadow", ElementId::FeDropShadow),
        ("feSpotLight", ElementId::FeSpotLight),
        ("marker", ElementId::Marker),
        ("style", ElementId::Style),
        ("switch", ElementId::Switch),
        ("tspan", ElementId::Tspan),
        ("feColorMatrix", ElementId::FeColorMatrix),
        ("feOffset", ElementId::FeOffset),
        ("path", ElementId::Path),
        ("feGaussianBlur", ElementId::FeGaussianBlur),
        ("feTile", ElementId::FeTile),
        ("feTurbulence", ElementId::FeTurbulence),
        ("feMergeNode", ElementId::FeMergeNode),
        ("feMorphology", ElementId::FeMorphology),
        ("a", ElementId::A),
        ("textPath", ElementId::TextPath),
        ("ellipse", ElementId::Ellipse),
        ("feComponentTransfer", ElementId::FeComponentTransfer),
        ("feDistantLight", ElementId::FeDistantLight),
        ("polyline", ElementId::Polyline),
        ("polygon", ElementId::Polygon),
        ("feBlend", ElementId::FeBlend),
        ("feDisplacementMap", ElementId::FeDisplacementMap),
        ("feDiffuseLighting", ElementId::FeDiffuseLighting),
        ("rect", ElementId::Rect),
        ("feFuncR", ElementId::FeFuncR),
    ],
};

impl ElementId {
    pub(crate) fn from_str(text: &str) -> Option<ElementId> {
        ELEMENTS.get(text).cloned()
    }

    /// Returns the original string.
    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ELEMENTS.key(&self)
    }
}

impl std::fmt::Debug for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::fmt::Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An attribute ID.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum AttributeId {
    AlignmentBaseline,
    Amplitude,
    Azimuth,
    BaseFrequency,
    BaselineShift,
    Bias,
    Class,
    Clip,
    ClipPath,
    ClipRule,
    ClipPathUnits,
    Color,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorProfile,
    ColorRendering,
    Cx,
    Cy,
    D,
    DiffuseConstant,
    Direction,
    Display,
    Divisor,
    DominantBaseline,
    Dx,
    Dy,
    EdgeMode,
    Elevation,
    EnableBackground,
    Exponent,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FilterUnits,
    FloodColor,
    FloodOpacity,
    Font,
    FontFamily,
    FontFeatureSettings,
    FontKerning,
    FontSize,
    FontSizeAdjust,
    FontStretch,
    FontStyle,
    FontSynthesis,
    FontVariant,
    FontVariantCaps,
    FontVariantEastAsian,
    FontVariantLigatures,
    FontVariantNumeric,
    FontVariantPosition,
    FontWeight,
    Fr,
    Fx,
    Fy,
    GlyphOrientationHorizontal,
    GlyphOrientationVertical,
    GradientTransform,
    GradientUnits,
    Height,
    Href,
    Id,
    ImageRendering,
    In,
    In2,
    InlineSize,
    Intercept,
    Isolation,
    K1,
    K2,
    K3,
    K4,
    KernelMatrix,
    KernelUnitLength,
    Kerning,
    LengthAdjust,
    LetterSpacing,
    LightingColor,
    LimitingConeAngle,
    LineHeight,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    MarkerHeight,
    MarkerUnits,
    MarkerWidth,
    Mask,
    MaskBorder,
    MaskBorderMode,
    MaskBorderOutset,
    MaskBorderRepeat,
    MaskBorderSlice,
    MaskBorderSource,
    MaskBorderWidth,
    MaskClip,
    MaskComposite,
    MaskImage,
    MaskMode,
    MaskOrigin,
    MaskPosition,
    MaskSize,
    MaskType,
    MaskContentUnits,
    MaskUnits,
    MixBlendMode,
    Mode,
    NumOctaves,
    Offset,
    Opacity,
    Operator,
    Order,
    Orient,
    Overflow,
    PaintOrder,
    Path,
    PathLength,
    PatternContentUnits,
    PatternTransform,
    PatternUnits,
    Points,
    PointsAtX,
    PointsAtY,
    PointsAtZ,
    PreserveAlpha,
    PreserveAspectRatio,
    PrimitiveUnits,
    R,
    Radius,
    RefX,
    RefY,
    RequiredExtensions,
    RequiredFeatures,
    Result,
    Rotate,
    Rx,
    Ry,
    Scale,
    Seed,
    ShapeImageThreshold,
    ShapeInside,
    ShapeMargin,
    ShapePadding,
    ShapeRendering,
    ShapeSubtract,
    Side,
    Slope,
    Space,
    SpecularConstant,
    SpecularExponent,
    SpreadMethod,
    StartOffset,
    StdDeviation,
    StitchTiles,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    Style,
    SurfaceScale,
    SystemLanguage,
    TableValues,
    TargetX,
    TargetY,
    TextAlign,
    TextAlignLast,
    TextAnchor,
    TextDecoration,
    TextDecorationColor,
    TextDecorationFill,
    TextDecorationLine,
    TextDecorationStroke,
    TextDecorationStyle,
    TextIndent,
    TextOrientation,
    TextOverflow,
    TextRendering,
    TextUnderlinePosition,
    TextLength,
    Transform,
    TransformBox,
    TransformOrigin,
    Type,
    UnicodeBidi,
    UnicodeRange,
    Values,
    VectorEffect,
    ViewBox,
    Visibility,
    WhiteSpace,
    Width,
    WordSpacing,
    WritingMode,
    X,
    X1,
    X2,
    XChannelSelector,
    Y,
    Y1,
    Y2,
    YChannelSelector,
    Z
}

static ATTRIBUTES: Map<AttributeId> = Map {
    key: 732231254413039614,
    disps: &[
        (63, 17),
        (0, 1),
        (0, 113),
        (0, 21),
        (0, 8),
        (0, 23),
        (0, 15),
        (0, 3),
        (40, 203),
        (0, 3),
        (0, 6),
        (1, 31),
        (0, 68),
        (11, 161),
        (0, 41),
        (2, 58),
        (7, 10),
        (0, 0),
        (0, 39),
        (7, 73),
        (0, 5),
        (65, 30),
        (0, 71),
        (0, 28),
        (0, 7),
        (2, 99),
        (0, 168),
        (1, 79),
        (0, 0),
        (0, 3),
        (0, 21),
        (1, 9),
        (0, 0),
        (5, 171),
        (0, 45),
        (0, 18),
        (99, 110),
        (31, 206),
        (5, 125),
        (0, 14),
        (0, 4),
        (0, 83),
    ],
    entries: &[
        ("mask-type", AttributeId::MaskType),
        ("markerWidth", AttributeId::MarkerWidth),
        ("textLength", AttributeId::TextLength),
        ("alignment-baseline", AttributeId::AlignmentBaseline),
        ("unicode-bidi", AttributeId::UnicodeBidi),
        ("dominant-baseline", AttributeId::DominantBaseline),
        ("orient", AttributeId::Orient),
        ("x", AttributeId::X),
        ("font-style", AttributeId::FontStyle),
        ("color-interpolation", AttributeId::ColorInterpolation),
        ("style", AttributeId::Style),
        ("clip-rule", AttributeId::ClipRule),
        ("bias", AttributeId::Bias),
        ("font-weight", AttributeId::FontWeight),
        ("fill", AttributeId::Fill),
        ("mask-border-outset", AttributeId::MaskBorderOutset),
        ("x1", AttributeId::X1),
        ("mask", AttributeId::Mask),
        ("height", AttributeId::Height),
        ("stroke", AttributeId::Stroke),
        ("transform", AttributeId::Transform),
        ("isolation", AttributeId::Isolation),
        ("fy", AttributeId::Fy),
        ("text-orientation", AttributeId::TextOrientation),
        ("stop-opacity", AttributeId::StopOpacity),
        ("fx", AttributeId::Fx),
        ("font-kerning", AttributeId::FontKerning),
        ("cx", AttributeId::Cx),
        ("maskContentUnits", AttributeId::MaskContentUnits),
        ("opacity", AttributeId::Opacity),
        ("filterUnits", AttributeId::FilterUnits),
        ("lengthAdjust", AttributeId::LengthAdjust),
        ("text-anchor", AttributeId::TextAnchor),
        ("stroke-miterlimit", AttributeId::StrokeMiterlimit),
        ("viewBox", AttributeId::ViewBox),
        ("visibility", AttributeId::Visibility),
        ("ry", AttributeId::Ry),
        ("glyph-orientation-horizontal", AttributeId::GlyphOrientationHorizontal),
        ("gradientTransform", AttributeId::GradientTransform),
        ("markerUnits", AttributeId::MarkerUnits),
        ("shape-inside", AttributeId::ShapeInside),
        ("font-size", AttributeId::FontSize),
        ("yChannelSelector", AttributeId::YChannelSelector),
        ("clip-path", AttributeId::ClipPath),
        ("flood-color", AttributeId::FloodColor),
        ("marker-mid", AttributeId::MarkerMid),
        ("surfaceScale", AttributeId::SurfaceScale),
        ("color-profile", AttributeId::ColorProfile),
        ("requiredFeatures", AttributeId::RequiredFeatures),
        ("class", AttributeId::Class),
        ("startOffset", AttributeId::StartOffset),
        ("mask-origin", AttributeId::MaskOrigin),
        ("stdDeviation", AttributeId::StdDeviation),
        ("mode", AttributeId::Mode),
        ("overflow", AttributeId::Overflow),
        ("text-decoration-fill", AttributeId::TextDecorationFill),
        ("mask-border-mode", AttributeId::MaskBorderMode),
        ("divisor", AttributeId::Divisor),
        ("marker-start", AttributeId::MarkerStart),
        ("text-decoration-line", AttributeId::TextDecorationLine),
        ("preserveAlpha", AttributeId::PreserveAlpha),
        ("primitiveUnits", AttributeId::PrimitiveUnits),
        ("in", AttributeId::In),
        ("points", AttributeId::Points),
        ("pointsAtY", AttributeId::PointsAtY),
        ("specularExponent", AttributeId::SpecularExponent),
        ("shape-rendering", AttributeId::ShapeRendering),
        ("refY", AttributeId::RefY),
        ("clip", AttributeId::Clip),
        ("white-space", AttributeId::WhiteSpace),
        ("vector-effect", AttributeId::VectorEffect),
        ("image-rendering", AttributeId::ImageRendering),
        ("stitchTiles", AttributeId::StitchTiles),
        ("stroke-opacity", AttributeId::StrokeOpacity),
        ("shape-margin", AttributeId::ShapeMargin),
        ("y2", AttributeId::Y2),
        ("operator", AttributeId::Operator),
        ("pathLength", AttributeId::PathLength),
        ("order", AttributeId::Order),
        ("text-rendering", AttributeId::TextRendering),
        ("mask-border", AttributeId::MaskBorder),
        ("exponent", AttributeId::Exponent),
        ("color-interpolation-filters", AttributeId::ColorInterpolationFilters),
        ("diffuseConstant", AttributeId::DiffuseConstant),
        ("space", AttributeId::Space),
        ("font-synthesis", AttributeId::FontSynthesis),
        ("direction", AttributeId::Direction),
        ("font-size-adjust", AttributeId::FontSizeAdjust),
        ("kerning", AttributeId::Kerning),
        ("flood-opacity", AttributeId::FloodOpacity),
        ("mask-border-source", AttributeId::MaskBorderSource),
        ("line-height", AttributeId::LineHeight),
        ("mix-blend-mode", AttributeId::MixBlendMode),
        ("scale", AttributeId::Scale),
        ("mask-image", AttributeId::MaskImage),
        ("dy", AttributeId::Dy),
        ("xChannelSelector", AttributeId::XChannelSelector),
        ("unicode-range", AttributeId::UnicodeRange),
        ("y", AttributeId::Y),
        ("fill-opacity", AttributeId::FillOpacity),
        ("mask-composite", AttributeId::MaskComposite),
        ("targetY", AttributeId::TargetY),
        ("writing-mode", AttributeId::WritingMode),
        ("transform-origin", AttributeId::TransformOrigin),
        ("font-variant-caps", AttributeId::FontVariantCaps),
        ("k2", AttributeId::K2),
        ("transform-box", AttributeId::TransformBox),
        ("text-decoration", AttributeId::TextDecoration),
        ("filter", AttributeId::Filter),
        ("enable-background", AttributeId::EnableBackground),
        ("word-spacing", AttributeId::WordSpacing),
        ("gradientUnits", AttributeId::GradientUnits),
        ("font-feature-settings", AttributeId::FontFeatureSettings),
        ("stroke-linecap", AttributeId::StrokeLinecap),
        ("rx", AttributeId::Rx),
        ("kernelUnitLength", AttributeId::KernelUnitLength),
        ("shape-image-threshold", AttributeId::ShapeImageThreshold),
        ("inline-size", AttributeId::InlineSize),
        ("edgeMode", AttributeId::EdgeMode),
        ("text-decoration-style", AttributeId::TextDecorationStyle),
        ("pointsAtZ", AttributeId::PointsAtZ),
        ("elevation", AttributeId::Elevation),
        ("tableValues", AttributeId::TableValues),
        ("kernelMatrix", AttributeId::KernelMatrix),
        ("patternTransform", AttributeId::PatternTransform),
        ("fill-rule", AttributeId::FillRule),
        ("color", AttributeId::Color),
        ("lighting-color", AttributeId::LightingColor),
        ("cy", AttributeId::Cy),
        ("fr", AttributeId::Fr),
        ("systemLanguage", AttributeId::SystemLanguage),
        ("text-indent", AttributeId::TextIndent),
        ("slope", AttributeId::Slope),
        ("font", AttributeId::Font),
        ("requiredExtensions", AttributeId::RequiredExtensions),
        ("href", AttributeId::Href),
        ("baseFrequency", AttributeId::BaseFrequency),
        ("stroke-dashoffset", AttributeId::StrokeDashoffset),
        ("text-decoration-stroke", AttributeId::TextDecorationStroke),
        ("display", AttributeId::Display),
        ("amplitude", AttributeId::Amplitude),
        ("mask-size", AttributeId::MaskSize),
        ("font-variant-ligatures", AttributeId::FontVariantLigatures),
        ("in2", AttributeId::In2),
        ("maskUnits", AttributeId::MaskUnits),
        ("k4", AttributeId::K4),
        ("shape-padding", AttributeId::ShapePadding),
        ("r", AttributeId::R),
        ("text-underline-position", AttributeId::TextUnderlinePosition),
        ("font-variant-east-asian", AttributeId::FontVariantEastAsian),
        ("z", AttributeId::Z),
        ("text-decoration-color", AttributeId::TextDecorationColor),
        ("intercept", AttributeId::Intercept),
        ("side", AttributeId::Side),
        ("x2", AttributeId::X2),
        ("mask-position", AttributeId::MaskPosition),
        ("pointsAtX", AttributeId::PointsAtX),
        ("patternUnits", AttributeId::PatternUnits),
        ("result", AttributeId::Result),
        ("baseline-shift", AttributeId::BaselineShift),
        ("stop-color", AttributeId::StopColor),
        ("k3", AttributeId::K3),
        ("stroke-dasharray", AttributeId::StrokeDasharray),
        ("y1", AttributeId::Y1),
        ("targetX", AttributeId::TargetX),
        ("stroke-width", AttributeId::StrokeWidth),
        ("limitingConeAngle", AttributeId::LimitingConeAngle),
        ("id", AttributeId::Id),
        ("mask-border-width", AttributeId::MaskBorderWidth),
        ("shape-subtract", AttributeId::ShapeSubtract),
        ("font-variant", AttributeId::FontVariant),
        ("glyph-orientation-vertical", AttributeId::GlyphOrientationVertical),
        ("offset", AttributeId::Offset),
        ("type", AttributeId::Type),
        ("numOctaves", AttributeId::NumOctaves),
        ("font-stretch", AttributeId::FontStretch),
        ("mask-clip", AttributeId::MaskClip),
        ("clipPathUnits", AttributeId::ClipPathUnits),
        ("path", AttributeId::Path),
        ("seed", AttributeId::Seed),
        ("font-variant-position", AttributeId::FontVariantPosition),
        ("rotate", AttributeId::Rotate),
        ("text-overflow", AttributeId::TextOverflow),
        ("mask-border-repeat", AttributeId::MaskBorderRepeat),
        ("stroke-linejoin", AttributeId::StrokeLinejoin),
        ("mask-mode", AttributeId::MaskMode),
        ("specularConstant", AttributeId::SpecularConstant),
        ("spreadMethod", AttributeId::SpreadMethod),
        ("letter-spacing", AttributeId::LetterSpacing),
        ("font-family", AttributeId::FontFamily),
        ("azimuth", AttributeId::Azimuth),
        ("refX", AttributeId::RefX),
        ("color-rendering", AttributeId::ColorRendering),
        ("values", AttributeId::Values),
        ("marker-end", AttributeId::MarkerEnd),
        ("font-variant-numeric", AttributeId::FontVariantNumeric),
        ("markerHeight", AttributeId::MarkerHeight),
        ("radius", AttributeId::Radius),
        ("width", AttributeId::Width),
        ("text-align-last", AttributeId::TextAlignLast),
        ("preserveAspectRatio", AttributeId::PreserveAspectRatio),
        ("patternContentUnits", AttributeId::PatternContentUnits),
        ("k1", AttributeId::K1),
        ("paint-order", AttributeId::PaintOrder),
        ("d", AttributeId::D),
        ("mask-border-slice", AttributeId::MaskBorderSlice),
        ("dx", AttributeId::Dx),
        ("text-align", AttributeId::TextAlign),
    ],
};

impl AttributeId {
    pub(crate) fn from_str(text: &str) -> Option<AttributeId> {
        ATTRIBUTES.get(text).cloned()
    }

    /// Returns the original string.
    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ATTRIBUTES.key(&self)
    }
}

impl std::fmt::Debug for AttributeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::fmt::Display for AttributeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static[(&'static str, V)],
}

impl<V: PartialEq> Map<V> {
    fn get(&self, key: &str) -> Option<&V> {
        use std::borrow::Borrow;

        let hash = hash(key, self.key);
        let index = get_index(hash, self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0.borrow();
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }

    fn key(&self, value: &V) -> &'static str {
        self.entries.iter().find(|kv| kv.1 == *value).unwrap().0
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    use std::hash::Hasher;

    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    ((hash & MASK) as u32,
     ((hash >> BITS) & MASK) as u32,
     ((hash >> (2 * BITS)) & MASK) as u32)
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}