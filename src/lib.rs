use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryInto;

use pdf::{content::Operation as PdfOperation, primitive::Primitive};
use pdf::encoding::BaseEncoding;
use pdf::font::Font;
use pdf::object::RcRef;
use pdf::primitive::PdfString;

pub struct Name<'src>(&'src str);

pub enum LineCapStyle {
    ButtCap,
    RoundCap,
    ProjectingSquareCap,
}

pub enum LineJoinStyle {
    MiterJoin,
    RoundJoin,
    BevelJoin,
}

pub enum TextRenderingMode {
    FillText,
    StrokeText,
    FillThenStrokeText,
    Invisible,
    FillTextAndAddToPathForClipping,
    StrokeTextAndAddToPathForClipping,
    FillThenStrokeTextAndAddToPathForClipping,
    AddTextToPathForClipping,
}

pub enum TextOrGlyphPositioning<'src> {
    Text(Cow<'src, str>),
    GlyphPositioning(f32),
}

pub enum UntypedColor {
    DeviceGrayCalGrayOrIndexed(f32),
    DeviceRGBCalRGBOrLab(f32, f32, f32),
    DeviceCMYK(f32, f32, f32, f32),
}

pub enum ColorRenderingIntent {
    AbsoluteColorimetric,
    RelativeColorimetric,
    Saturation,
    Perceptual,
}

pub enum Operation<'src> {
    CloseFillAndStrokePathUsingNonZeroWindingNumber,
    FillAndStrokePathUsingNonZeroWindingNumber,
    CloseFillAndStrokePathUsingEvenOddRule,
    FillAndStrokePathUsingEvenOddRule,
    BeginMarkedContentSequenceWithPropertyList,
    BeginInlineImageObject,
    BeginMarkedContentSequence,
    BeginTextObject,
    BeginCompatibilitySection,
    AppendCurvedSegmentToPath {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    },
    ConcatenateMatrixToCurrentTransformationMatrix(f32, f32, f32, f32, f32, f32),
    SetColorSpaceForStrokingOperations(Name<'src>),
    SetColorSpaceForNonStrokingOperations(Name<'src>),
    SetLineDashPattern {
        array: Vec<f32>,
        phase: f32,
    },
    SetGlyphWidthInType3Font {
        wx: f32,
        wy: f32,
    },
    SetGlyphWidthAndBoundingBoxInType3Font {
        wx: f32,
        wy: f32,
        llx: f32,
        lly: f32,
        urx: f32,
        ury: f32,
    },
    InvokeNamedXObject(Name<'src>),
    DefineMarkedContentPointWithPropertyList,
    EndInlineImageObject,
    EndMarkedContentSequence,
    EndTextObject,
    EndCompatibilitySection,
    FillPathUsingNonZeroWindingNumberRule,
    ObsoleteFillPathUsingNonZeroWindingMumberRule,
    FillPathUsingEvenOddRule,
    SetGrayLevelForStrokingOperations(f32),
    SetGrayLevelForNonStrokingOperations(f32),
    SetParametersFromGraphicsStateParameterDictionary(Name<'src>),
    CloseSubpath,
    SetFlatnessTolerance(i32),
    BeginInlineImageData,
    SetLineJoinStyle(LineJoinStyle),
    SetLineCapStyle(LineCapStyle),
    SetCMYKColorForStrokingOperations(f32, f32, f32, f32),
    SetCMYKColorForNonStrokingOperations(f32, f32, f32, f32),
    AppendStraightLineSegmentToPath {
        x: f32,
        y: f32,
    },
    BeginNewSubpath {
        x: f32,
        y: f32,
    },
    SetMiterLimit(f32),
    DefineMarkedContentPoint(Name<'src>),
    EndPathWithoutFillingOrStroking,
    SaveGraphicsState,
    RestoreGraphicsState,
    AppendRectangleToPath {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    SetRGBColorForStrokingOperations(f32, f32, f32),
    SetRGBColorForNonStrokingOperations(f32, f32, f32),
    SetColorRenderingIntent(ColorRenderingIntent),
    CloseAndStrokePath,
    StrokePath,
    SetColorForStrokingOperations(UntypedColor),
    SetColorForNonStrokingOperations(UntypedColor),
    SetColorForStrokingOperationsICCBasedAndSpecialColorSpaces {
        cs: Vec<f32>,
        name: Option<Name<'src>>,
    },
    SetColorForNonStrokingOperationsICCBasedAndSpecialColorSpaces {
        cs: Vec<f32>,
        name: Option<Name<'src>>,
    },
    PaintAreaDefinedByShadingPattern(Name<'src>),
    MoveToStartOfNextTextLine,
    SetCharacterSpacing(f32),
    MoveTextPosition {
        x: f32,
        y: f32,
    },
    MoveTextPositionAndSetLeading {
        x: f32,
        y: f32,
    },
    SetTextFontAndSize {
        font: &'src str,
        size: f32,
    },
    ShowText(Cow<'src, str>),
    ShowTextAllowingIndividualGlyphPositioning(Vec<TextOrGlyphPositioning<'src>>),
    SetTextLeading(f32),
    SetTextMatrixAndTextLineMatrix(f32, f32, f32, f32, f32, f32),
    SetTextRenderingMode(TextRenderingMode),
    SetTextRise(f32),
    SetWordSpacing(f32),
    SetHorizontalTextScaling(f32),
    AppendCurvedSegmentToPathInitialPointReplicated {
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    },
    SetLineWidth(f32),
    SetClippingPathUsingNonZeroWindingNumberRule,
    SetClippingPathUsingEvenOddRule,
    AppendCurvedSegmentToPathFinalPointReplicated {
        x1: f32,
        y1: f32,
        x3: f32,
        y3: f32,
    },
    MoveToNextLineAndShowText(Cow<'src, str>),
    SetWordAndCharacterSpacingMoveToNextLineAndShowText {
        text: Cow<'src, str>,
        word_spacing: f32,
        character_spacing: f32,
    },
    Unknown {
        operator: &'src str,
        operands: &'src [Primitive],
    },
}

trait PrimitiveExt {
    fn try_to_f(&self) -> Option<f32>;
}

impl PrimitiveExt for Primitive {
    fn try_to_f(&self) -> Option<f32> {
        match self {
            Primitive::Integer(i) => Some(*i as f32),
            Primitive::Number(f) => Some(*f as f32),
            _ => None,
        }
    }
}

pub struct FontInfo {
    pub font: RcRef<Font>,
    pub cmap: HashMap<u16, String>
}

fn decode_string<'a>(text: &'a PdfString, current_font: Option<&FontInfo>) -> pdf::error::Result<Cow<'a, str>> {
    match current_font {
        Some(cf) => match cf.font.encoding() {
            Some(encoding) => {
                match encoding.base {
                    BaseEncoding::IdentityH => {
                        let mut out: String = "".to_string();
                        for w in text.as_bytes().windows(2) {
                            let cp = u16::from_be_bytes(w.try_into().unwrap());
                            if let Some(s) = cf.cmap.get(&cp) {
                                out.push_str(s);
                            }
                        }
                        Ok(Cow::from(out))
                    }
                    _ => {
                        let mut out: String = "".to_string();
                        for &b in text.as_bytes() {
                            if let Some(s) = cf.cmap.get(&(b as u16)) {
                                out.push_str(s);
                            } else {
                                out.push(b as char);
                            }
                        }
                        Ok(Cow::from(out))
                    }
                }
            }
            None => text.as_str()
        }
        None => text.as_str()
    }
}

pub fn normalize_operation(operation: &PdfOperation) -> Operation {
    normalize_operation_with_font(operation, None)
}

pub fn normalize_operation_with_font<'a>(operation: &'a PdfOperation, current_font: Option<&FontInfo>) -> Operation<'a> {
    let PdfOperation { operator, operands } = operation;

    match (operator.as_str(), operands.as_slice()) {
        ("b", _) => Operation::CloseFillAndStrokePathUsingNonZeroWindingNumber,
        ("B", _) => Operation::FillAndStrokePathUsingNonZeroWindingNumber,
        ("b*", _) => Operation::CloseFillAndStrokePathUsingEvenOddRule,
        ("B*", _) => Operation::FillAndStrokePathUsingEvenOddRule,
        ("BDC", []) => Operation::BeginMarkedContentSequenceWithPropertyList,
        ("BI", []) => Operation::BeginInlineImageObject,
        ("BMC", []) => Operation::BeginMarkedContentSequence,
        ("BT", _) => Operation::BeginTextObject,
        ("BX", []) => Operation::BeginCompatibilitySection,
        ("c", ns) => {
            let ns = ns.iter().filter_map(|n| n.try_to_f()).collect::<Vec<_>>();

            if let [x1, y1, x2, y2, x3, y3] = ns.as_slice() {
                Operation::AppendCurvedSegmentToPath {
                    x1: *x1,
                    y1: *y1,
                    x2: *x2,
                    y2: *y2,
                    x3: *x3,
                    y3: *y3,
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("cm", ns) => {
            let ns = ns.iter().filter_map(|n| n.try_to_f()).collect::<Vec<_>>();

            if let [a, b, c, d, e, f] = ns.as_slice() {
                Operation::ConcatenateMatrixToCurrentTransformationMatrix(*a, *b, *c, *d, *e, *f)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("CS", [Primitive::Name(name)]) => {
            Operation::SetColorSpaceForStrokingOperations(Name(name))
        }
        ("cs", [Primitive::Name(name)]) => {
            Operation::SetColorSpaceForNonStrokingOperations(Name(name))
        }
        ("d", [Primitive::Array(primitive_array), phase]) => {
            let array = primitive_array
                .iter()
                .filter_map(|n| {
                    if let Primitive::Integer(n) = n {
                        Some(*n as f32)
                    } else if let Primitive::Number(n) = n {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let phase = phase.try_to_f();

            if let Some(phase) = phase {
                if array.len() == primitive_array.len() {
                    Operation::SetLineDashPattern { array, phase }
                } else {
                    Operation::Unknown { operator, operands }
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("d0", [wx, wy]) => {
            if let (Some(wx), Some(wy)) = (wx.try_to_f(), wy.try_to_f()) {
                Operation::SetGlyphWidthInType3Font { wx, wy }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("d1", [wx, wy, llx, lly, urx, ury]) => {
            if let (Some(wx), Some(wy), Some(llx), Some(lly), Some(urx), Some(ury)) = (
                wx.try_to_f(),
                wy.try_to_f(),
                llx.try_to_f(),
                lly.try_to_f(),
                urx.try_to_f(),
                ury.try_to_f(),
            ) {
                Operation::SetGlyphWidthAndBoundingBoxInType3Font {
                    wx,
                    wy,
                    llx,
                    lly,
                    urx,
                    ury,
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("Do", [Primitive::Name(name)]) => Operation::InvokeNamedXObject(Name(name)),
        ("DP", []) => Operation::DefineMarkedContentPointWithPropertyList,
        ("EI", []) => Operation::EndInlineImageObject,
        ("EMC", []) => Operation::EndMarkedContentSequence,
        ("ET", _) => Operation::EndTextObject,
        ("EX", []) => Operation::EndCompatibilitySection,
        ("f", _) => Operation::FillPathUsingNonZeroWindingNumberRule,
        ("F", _) => Operation::ObsoleteFillPathUsingNonZeroWindingMumberRule,
        ("f*", _) => Operation::FillPathUsingEvenOddRule,
        ("G", [Primitive::Number(shade)]) => Operation::SetGrayLevelForStrokingOperations(*shade),
        ("G", [Primitive::Integer(shade)]) => {
            Operation::SetGrayLevelForStrokingOperations(*shade as f32)
        }
        ("g", [Primitive::Number(shade)]) => {
            Operation::SetGrayLevelForNonStrokingOperations(*shade)
        }
        ("g", [Primitive::Integer(shade)]) => {
            Operation::SetGrayLevelForNonStrokingOperations(*shade as f32)
        }
        ("gs", [Primitive::Name(name)]) => {
            Operation::SetParametersFromGraphicsStateParameterDictionary(Name(name))
        }
        ("h", _) => Operation::CloseSubpath,
        ("i", [Primitive::Integer(flatness)]) => Operation::SetFlatnessTolerance(*flatness),
        ("ID", []) => Operation::BeginInlineImageData,
        ("j", [Primitive::Integer(0)]) => Operation::SetLineJoinStyle(LineJoinStyle::MiterJoin),
        ("j", [Primitive::Integer(1)]) => Operation::SetLineJoinStyle(LineJoinStyle::RoundJoin),
        ("j", [Primitive::Integer(2)]) => Operation::SetLineJoinStyle(LineJoinStyle::BevelJoin),
        ("J", [Primitive::Integer(0)]) => Operation::SetLineCapStyle(LineCapStyle::ButtCap),
        ("J", [Primitive::Integer(1)]) => Operation::SetLineCapStyle(LineCapStyle::RoundCap),
        ("J", [Primitive::Integer(2)]) => {
            Operation::SetLineCapStyle(LineCapStyle::ProjectingSquareCap)
        }
        ("K", [c, m, y, k]) => {
            if let (Some(c), Some(m), Some(y), Some(k)) =
                (c.try_to_f(), m.try_to_f(), y.try_to_f(), k.try_to_f())
            {
                Operation::SetCMYKColorForStrokingOperations(c, m, y, k)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("k", [c, m, y, k]) => {
            if let (Some(c), Some(m), Some(y), Some(k)) =
                (c.try_to_f(), m.try_to_f(), y.try_to_f(), k.try_to_f())
            {
                Operation::SetCMYKColorForNonStrokingOperations(c, m, y, k)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("l", [x, y]) => {
            let x = x.try_to_f();
            let y = y.try_to_f();

            if let (Some(x), Some(y)) = (x, y) {
                Operation::AppendStraightLineSegmentToPath { x, y }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("m", [x, y]) => {
            let x = x.try_to_f();
            let y = y.try_to_f();

            if let (Some(x), Some(y)) = (x, y) {
                Operation::BeginNewSubpath { x, y }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("M", [limit]) => {
            if let Some(limit) = limit.try_to_f() {
                Operation::SetMiterLimit(limit)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("MP", [Primitive::Name(tag)]) => Operation::DefineMarkedContentPoint(Name(tag)),
        ("n", _) => Operation::EndPathWithoutFillingOrStroking,
        ("q", _) => Operation::SaveGraphicsState,
        ("Q", _) => Operation::RestoreGraphicsState,
        ("re", [x, y, width, height]) => {
            let x = x.try_to_f();
            let y = y.try_to_f();
            let width = width.try_to_f();
            let height = height.try_to_f();

            if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
                Operation::AppendRectangleToPath {
                    x,
                    y,
                    width,
                    height,
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("RG", [r, g, b]) => {
            let r = r.try_to_f();
            let g = g.try_to_f();
            let b = b.try_to_f();

            if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                Operation::SetRGBColorForStrokingOperations(r, g, b)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("rg", [r, g, b]) => {
            let r = r.try_to_f();
            let g = g.try_to_f();
            let b = b.try_to_f();

            if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                Operation::SetRGBColorForNonStrokingOperations(r, g, b)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("ri", [Primitive::Name(name)]) if name == "AbsoluteColorimetric" => {
            Operation::SetColorRenderingIntent(ColorRenderingIntent::AbsoluteColorimetric)
        }
        ("ri", [Primitive::Name(name)]) if name == "RelativeColorimetric" => {
            Operation::SetColorRenderingIntent(ColorRenderingIntent::RelativeColorimetric)
        }
        ("ri", [Primitive::Name(name)]) if name == "Saturation" => {
            Operation::SetColorRenderingIntent(ColorRenderingIntent::Saturation)
        }
        ("ri", [Primitive::Name(name)]) if name == "Perceptual" => {
            Operation::SetColorRenderingIntent(ColorRenderingIntent::Perceptual)
        }
        ("s", _) => Operation::CloseAndStrokePath,
        ("S", _) => Operation::StrokePath,
        ("SC", [a]) => {
            let a = a.try_to_f();

            if let Some(a) = a {
                Operation::SetColorForStrokingOperations(UntypedColor::DeviceGrayCalGrayOrIndexed(
                    a,
                ))
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("SC", [a, b, c]) => {
            let a = a.try_to_f();
            let b = b.try_to_f();
            let c = c.try_to_f();

            if let (Some(a), Some(b), Some(c)) = (a, b, c) {
                Operation::SetColorForStrokingOperations(UntypedColor::DeviceRGBCalRGBOrLab(
                    a, b, c,
                ))
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("SC", [a, b, c, d]) => {
            let a = a.try_to_f();
            let b = b.try_to_f();
            let c = c.try_to_f();
            let d = d.try_to_f();

            if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
                Operation::SetColorForStrokingOperations(UntypedColor::DeviceCMYK(a, b, c, d))
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("sc", [a]) => {
            let a = a.try_to_f();

            if let Some(a) = a {
                Operation::SetColorForNonStrokingOperations(
                    UntypedColor::DeviceGrayCalGrayOrIndexed(a),
                )
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("sc", [a, b, c]) => {
            let a = a.try_to_f();
            let b = b.try_to_f();
            let c = c.try_to_f();

            if let (Some(a), Some(b), Some(c)) = (a, b, c) {
                Operation::SetColorForNonStrokingOperations(UntypedColor::DeviceRGBCalRGBOrLab(
                    a, b, c,
                ))
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("sc", [a, b, c, d]) => {
            let a = a.try_to_f();
            let b = b.try_to_f();
            let c = c.try_to_f();
            let d = d.try_to_f();

            if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
                Operation::SetColorForNonStrokingOperations(UntypedColor::DeviceCMYK(a, b, c, d))
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("SCN", [primitive_cs @ .., Primitive::Name(name)]) => {
            let cs = primitive_cs
                .iter()
                .filter_map(|p| p.try_to_f())
                .collect::<Vec<_>>();

            if cs.len() == primitive_cs.len() {
                Operation::SetColorForStrokingOperationsICCBasedAndSpecialColorSpaces {
                    cs,
                    name: Some(Name(name)),
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("SCN", primitive_cs) => {
            let cs = primitive_cs
                .iter()
                .filter_map(|p| p.try_to_f())
                .collect::<Vec<_>>();

            if cs.len() == primitive_cs.len() {
                Operation::SetColorForStrokingOperationsICCBasedAndSpecialColorSpaces {
                    cs,
                    name: None,
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("scn", [primitive_cs @ .., Primitive::Name(name)]) => {
            let cs = primitive_cs
                .iter()
                .filter_map(|p| p.try_to_f())
                .collect::<Vec<_>>();

            if cs.len() == primitive_cs.len() {
                Operation::SetColorForNonStrokingOperationsICCBasedAndSpecialColorSpaces {
                    cs,
                    name: Some(Name(name)),
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("scn", primitive_cs) => {
            let cs = primitive_cs
                .iter()
                .filter_map(|p| p.try_to_f())
                .collect::<Vec<_>>();

            if cs.len() == primitive_cs.len() {
                Operation::SetColorForNonStrokingOperationsICCBasedAndSpecialColorSpaces {
                    cs,
                    name: None,
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("sh", [Primitive::Name(name)]) => Operation::PaintAreaDefinedByShadingPattern(Name(name)),
        ("T*", _) => Operation::MoveToStartOfNextTextLine,
        ("Tc", [Primitive::Number(spacing)]) => Operation::SetCharacterSpacing(*spacing),
        ("Tc", [Primitive::Integer(spacing)]) => Operation::SetCharacterSpacing(*spacing as f32),
        ("Td", [x, y]) => {
            let x = x.try_to_f();
            let y = y.try_to_f();

            if let (Some(x), Some(y)) = (x, y) {
                Operation::MoveTextPosition { x, y }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("TD", [x, y]) => {
            if let (Some(x), Some(y)) = (x.try_to_f(), y.try_to_f()) {
                Operation::MoveTextPositionAndSetLeading { x, y }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("Tf", [Primitive::Name(font), Primitive::Number(size)]) => Operation::SetTextFontAndSize {
            font: &font,
            size: *size,
        },
        ("Tf", [Primitive::Name(font), Primitive::Integer(size)]) => {
            Operation::SetTextFontAndSize {
                font: &font,
                size: *size as f32,
            }
        }
        ("Tj", [Primitive::String(text)]) => {
            decode_string(text, current_font)
                .map(Operation::ShowText)
                .unwrap_or_else(|_| Operation::Unknown { operator, operands })
        },
        ("TJ", [Primitive::Array(primitive_array)]) => {
            let array = primitive_array
                .iter()
                .filter_map(|primitive| match primitive {
                    Primitive::String(string) => {
                        decode_string(string, current_font).ok().map(TextOrGlyphPositioning::Text)
                    }
                    Primitive::Number(glyph_positioning) => {
                        Some(TextOrGlyphPositioning::GlyphPositioning(*glyph_positioning))
                    }
                    Primitive::Integer(glyph_positioning) => Some(
                        TextOrGlyphPositioning::GlyphPositioning(*glyph_positioning as f32),
                    ),
                    _ => None,
                })
                .collect::<Vec<_>>();

            if primitive_array.len() == array.len() {
                Operation::ShowTextAllowingIndividualGlyphPositioning(array)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("TL", [Primitive::Number(leading)]) => Operation::SetTextLeading(*leading),
        ("Tm", ns) => {
            let ns = ns.iter().filter_map(|n| n.try_to_f()).collect::<Vec<_>>();

            if let [a, b, c, d, e, f] = ns.as_slice() {
                if *a == 1.0 && *b == 0.0 && *c == 0.0 && *d == 1.0 {
                    Operation::MoveTextPosition { x: *e, y: *f }
                } else {
                    Operation::SetTextMatrixAndTextLineMatrix(*a, *b, *c, *d, *e, *f)
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("Tr", [Primitive::Integer(0)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::FillText)
        }
        ("Tr", [Primitive::Integer(1)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::StrokeText)
        }
        ("Tr", [Primitive::Integer(2)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::FillThenStrokeText)
        }
        ("Tr", [Primitive::Integer(3)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::Invisible)
        }
        ("Tr", [Primitive::Integer(4)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::FillTextAndAddToPathForClipping)
        }
        ("Tr", [Primitive::Integer(5)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::StrokeTextAndAddToPathForClipping)
        }
        ("Tr", [Primitive::Integer(6)]) => Operation::SetTextRenderingMode(
            TextRenderingMode::FillThenStrokeTextAndAddToPathForClipping,
        ),
        ("Tr", [Primitive::Integer(7)]) => {
            Operation::SetTextRenderingMode(TextRenderingMode::AddTextToPathForClipping)
        }
        ("Ts", [rise]) => rise
            .try_to_f()
            .map(Operation::SetTextRise)
            .unwrap_or_else(|| Operation::Unknown { operator, operands }),
        ("Tw", [spacing]) => {
            if let Some(spacing) = spacing.try_to_f() {
                Operation::SetWordSpacing(spacing)
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("Tz", [scale]) => scale
            .try_to_f()
            .map(Operation::SetHorizontalTextScaling)
            .unwrap_or_else(|| Operation::Unknown { operator, operands }),
        ("v", [x2, y2, x3, y3]) => {
            if let (Some(x2), Some(y2), Some(x3), Some(y3)) =
                (x2.try_to_f(), y2.try_to_f(), x3.try_to_f(), y3.try_to_f())
            {
                Operation::AppendCurvedSegmentToPathInitialPointReplicated { x2, y2, x3, y3 }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("w", [Primitive::Number(width)]) => Operation::SetLineWidth(*width),
        ("w", [Primitive::Integer(width)]) => Operation::SetLineWidth(*width as f32),
        ("W", _) => Operation::SetClippingPathUsingNonZeroWindingNumberRule,
        ("W*", _) => Operation::SetClippingPathUsingEvenOddRule,
        ("y", [x1, y1, x3, y3]) => {
            if let (Some(x1), Some(y1), Some(x3), Some(y3)) =
                (x1.try_to_f(), y1.try_to_f(), x3.try_to_f(), y3.try_to_f())
            {
                Operation::AppendCurvedSegmentToPathFinalPointReplicated { x1, y1, x3, y3 }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        ("'", [Primitive::String(text)]) => decode_string(text, current_font)
            .map(Operation::MoveToNextLineAndShowText)
            .unwrap_or_else(|_| Operation::Unknown { operator, operands }),
        ("\"", [word_spacing, character_spacing, Primitive::String(text)]) => {
            if let (Some(word_spacing), Some(character_spacing), Ok(text)) = (
                word_spacing.try_to_f(),
                character_spacing.try_to_f(),
                decode_string(text, current_font),
            ) {
                Operation::SetWordAndCharacterSpacingMoveToNextLineAndShowText {
                    word_spacing,
                    character_spacing,
                    text,
                }
            } else {
                Operation::Unknown { operator, operands }
            }
        }
        _ => Operation::Unknown { operator, operands },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
