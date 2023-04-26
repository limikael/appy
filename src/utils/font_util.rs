use std::collections::HashMap;
use rusttype::{Point, Rect};

#[derive(Debug)]
pub enum Error {
    /// A glyph for this character is not present in font.
    NoGlyph(char),
}

// structure containing informations about a character of a font
#[derive(Copy, Clone, Debug)]
pub struct CharacterInfos {
    // coordinates of the character top-left hand corner on the font's texture
    pub tex_coords: (f32, f32),

    // width and height of character in texture units
    pub tex_size: (f32, f32),

    // size of the character in EMs
    pub size: (f32, f32),

    // number of EMs between the bottom of the character and the base line of text
    pub height_over_line: f32,

    // number of EMs at the left of the character
    pub left_padding: f32,

    // number of EMs at the right of the character
    pub right_padding: f32,
}

impl CharacterInfos {
    pub fn scale(&self, font_size: f32) -> CharacterInfos {
        CharacterInfos {
            tex_coords: self.tex_coords,
            tex_size: self.tex_size,
            size: (self.size.0 * font_size, self.size.1 * font_size),
            height_over_line: self.height_over_line * font_size,
            left_padding: self.left_padding * font_size,
            right_padding: self.right_padding * font_size,
        }
    }
}

pub fn build_font_image<I>(
    font: &rusttype::Font,
    characters_list: I,
    font_size: u32,
)->Result<(Vec<u8>,u32,u32,HashMap<char,CharacterInfos>), Error>
where
    I: Iterator<Item = char>,
{
    use std::iter;

    // a margin around each character to prevent artifacts
    const MARGIN: u32 = 2;

    // glyph size for characters not presented in font.
    let invalid_character_width = font_size / 4;

    let size_estimation = characters_list.size_hint().1.unwrap_or(0);

    // this variable will store the texture data
    // we set an arbitrary capacity that we think will match what we will need
    let mut texture_data: Vec<u8> =
        Vec::with_capacity(size_estimation * font_size as usize * font_size as usize);

    // the width is chosen more or less arbitrarily, because we can store
    // everything as long as the texture is at least as wide as the widest
    // character we just try to estimate a width so that width ~= height
    let texture_width = get_nearest_po2(std::cmp::max(
        font_size * 2 as u32,
        ((((size_estimation as u32) * font_size * font_size) as f32).sqrt()) as u32,
    ));

    // we store the position of the "cursor" in the destination texture
    // this cursor points to the top-left pixel of the next character to write on the texture
    let mut cursor_offset = (0u32, 0u32);

    // number of rows to skip at next carriage return
    let mut rows_to_skip = 0u32;

    // now looping through the list of characters, filling the texture and returning the informations
    let em_pixels = font_size as f32;
    let characters_infos = characters_list
        .map(|character| {
            struct Bitmap {
                rows: i32,
                width: i32,
                buffer: Vec<u8>,
            }
            // loading wanted glyph in the font face
            // hope scale will set the right pixel size
            let scaled_glyph = font.glyph(character).scaled(::rusttype::Scale {
                x: font_size as f32,
                y: font_size as f32,
            });
            let h_metrics = scaled_glyph.h_metrics();
            let glyph = scaled_glyph.positioned(::rusttype::Point { x: 0.0, y: 0.0 });

            let bb = glyph.pixel_bounding_box();
            // if no bounding box - we suppose that its invalid character but want it to be draw as empty quad
            let bb = if let Some(bb) = bb {
                bb
            } else {
                Rect {
                    min: Point { x: 0, y: 0 },
                    max: Point {
                        x: invalid_character_width as i32,
                        y: 0,
                    },
                }
            };

            let mut buffer = vec![0; (bb.height() * bb.width()) as usize];

            glyph.draw(|x, y, v| {
                let x = x;
                let y = y;
                buffer[(y * bb.width() as u32 + x) as usize] = (v * 255.0) as u8;
            });
            let bitmap: Bitmap = Bitmap {
                rows: bb.height(),
                width: bb.width(),
                buffer,
            };

            // adding a left margin before our character to prevent artifacts
            cursor_offset.0 += MARGIN;

            // carriage return our cursor if we don't have enough room to write the next caracter
            // we add a margin to prevent artifacts
            if cursor_offset.0 + (bitmap.width as u32) + MARGIN >= texture_width {
                assert!(bitmap.width as u32 <= texture_width); // if this fails, we should increase texture_width
                cursor_offset.0 = 0;
                cursor_offset.1 += rows_to_skip;
                rows_to_skip = 0;
            }

            // if the texture data buffer has not enough lines, adding some
            if rows_to_skip < MARGIN + bitmap.rows as u32 {
                let diff = MARGIN + (bitmap.rows as u32) - rows_to_skip;
                rows_to_skip = MARGIN + bitmap.rows as u32;
                texture_data.extend(iter::repeat(0).take((diff * texture_width * 4) as usize));
            }

            // copying the data to the texture
            let offset_x_before_copy = cursor_offset.0;
            if bitmap.rows >= 1 {
                let destination = &mut texture_data
                    [(cursor_offset.0 * 4 + cursor_offset.1 * texture_width * 4) as usize..];
                let source = &bitmap.buffer;
                //ylet source = std::slice::from_raw_parts(source, destination.len());

                for y in 0..bitmap.rows as u32 {
                    let source = &source[(y * bitmap.width as u32) as usize..];
                    let destination = &mut destination[(y * texture_width * 4) as usize..];

                    for x in 0..bitmap.width {
                        for channel in 0..4 {
                            let val: u8 = source[x as usize];
                            let dest = &mut destination[x as usize * 4 + channel];

                            *dest = val;
                        }
                    }
                }

                cursor_offset.0 += bitmap.width as u32;
                debug_assert!(cursor_offset.0 <= texture_width);
            }

            // filling infos about that character
            // tex_size and tex_coords are in pixels for the moment ; they will be divided
            // by the texture dimensions later
            Ok((
                character,
                CharacterInfos {
                    tex_size: (bitmap.width as f32, bitmap.rows as f32),
                    tex_coords: (offset_x_before_copy as f32, cursor_offset.1 as f32),
                    size: (bitmap.width as f32, bitmap.rows as f32),
                    left_padding: h_metrics.left_side_bearing as f32,
                    right_padding: (h_metrics.advance_width
                        - bitmap.width as f32
                        - h_metrics.left_side_bearing as f32)
                        as f32,
                    height_over_line: -bb.min.y as f32,
                },
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // adding blank lines at the end until the height of the texture is a power of two
    {
        let current_height = texture_data.len() as u32 / texture_width;
        let requested_height = get_nearest_po2(current_height);
        texture_data.extend(
            iter::repeat(0)
                .take((texture_width * 4 * (requested_height - current_height)) as usize),
        );
    }

    // now our texture is finished
    // we know its final dimensions, so we can divide all the pixels values into (0,1) range
    assert!((texture_data.len() as u32 % (texture_width * 4)) == 0);
    let texture_height = (texture_data.len() as u32 / texture_width / 4) as f32;
    let float_texture_width = texture_width as f32;
    let mut characters_infos = characters_infos
        .into_iter()
        .map(|mut chr| {
            chr.1.tex_size.0 /= float_texture_width;
            chr.1.tex_size.1 /= texture_height;
            chr.1.tex_coords.0 /= float_texture_width;
            chr.1.tex_coords.1 /= texture_height;
            chr.1.size.0 /= em_pixels;
            chr.1.size.1 /= em_pixels;
            chr.1.left_padding /= em_pixels;
            chr.1.right_padding /= em_pixels;
            chr.1.height_over_line /= em_pixels;
            chr
        })
        .collect::<HashMap<_, _>>();

    // this HashMap will not be used mutably any more and it makes sense to
    // compact it
    characters_infos.shrink_to_fit();

    // Width multiplied by 4 because the original code wants to pack into RGBA
    // (I think)... Height divided by 4 for unknown reason...
    //
    Ok((
        texture_data,
        texture_width*4,
        (texture_height as u32),
        characters_infos
    ))
}

/// Function that will calculate the nearest power of two.
fn get_nearest_po2(mut x: u32) -> u32 {
    assert!(x > 0);
    x -= 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    x + 1
}
