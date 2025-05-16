use phf::phf_map;
use phf::Map;

//Map of font name -> character map for the font
//Each character map entry for a character contains the tex coords and size for the character.
static FONT_DATA: phf::Map<&'static str, FontData> = phf_map! {
    "default" => FontData::new(
        2, //Texture index
        8, //Whitespace width
        4, //Kerning
        phf_map!
        {
            "A" => FontCharacter::new([2,2], [34,29]),
            "B" => FontCharacter::new([40,2], [34,29]),
            "C" => FontCharacter::new([78,2], [34,29]),
            "D" => FontCharacter::new([116,2], [34,29]),
            "E" => FontCharacter::new([154,2], [34,29]),
            "F" => FontCharacter::new([192,2], [34,29]),
            "G" => FontCharacter::new([230,2], [34,29]),
            "H" => FontCharacter::new([286,2], [34,29]),
            "I" => FontCharacter::new([306,2], [17,29]),
            "J" => FontCharacter::new([327,2], [34,29]),
            "K" => FontCharacter::new([365,2], [34,29]),
            "L" => FontCharacter::new([403,2], [34,29]),
            "M" => FontCharacter::new([441,2], [39,29]),
            "N" => FontCharacter::new([483,2], [34,29]),
            "O" => FontCharacter::new([521,2], [34,29]),
            "P" => FontCharacter::new([559,2], [34,29]),
            "Q" => FontCharacter::new([597,2], [34,29]),
            "R" => FontCharacter::new([635,2], [34,29]),
            "S" => FontCharacter::new([673,2], [34,29]),
            "T" => FontCharacter::new([711,2], [34,29]),
            "U" => FontCharacter::new([749,2], [34,29]),
            "V" => FontCharacter::new([787,2], [34,29]),
            "W" => FontCharacter::new([825,2], [34,29]),
            "X" => FontCharacter::new([867,2], [34,29]),
            "Y" => FontCharacter::new([905,2], [34,29]),
            "Z" => FontCharacter::new([943,2], [34,29]),

            "a" => FontCharacter::new([2,41], [25,21]),
            "b" => FontCharacter::new([31,33], [25,29]),
            "c" => FontCharacter::new([61,41], [25,21]),
            "d" => FontCharacter::new([90,33], [25,29]),
            "e" => FontCharacter::new([120,41], [25,21]),
            "f" => FontCharacter::new([149,33], [25,29]),
            "g" => FontCharacter::new([179,41], [25,29]), //below line
            "h" => FontCharacter::new([208,33], [25,29]),
            "i" => FontCharacter::new([238,33], [8,29]),
            "j" => FontCharacter::new([251,33], [12,29]), //below line
            "k" => FontCharacter::new([267,33], [25,29]),
            "l" => FontCharacter::new([297,33], [8,29]),
            "m" => FontCharacter::new([310,41], [34,21]),
            "n" => FontCharacter::new([348,41], [25,21]),
            "o" => FontCharacter::new([377,41], [25,21]),
            "p" => FontCharacter::new([407,41], [25,29]),
            "q" => FontCharacter::new([436,41], [25,29]),
            "r" => FontCharacter::new([466,41], [25,21]),
            "s" => FontCharacter::new([495,41], [25,21]),
            "t" => FontCharacter::new([525,37], [21,25]),
            "u" => FontCharacter::new([550,41], [25,21]),
            "v" => FontCharacter::new([580,41], [26,21]),
            "w" => FontCharacter::new([609,41], [34,21]),
            "x" => FontCharacter::new([647,41], [25,21]),
            "y" => FontCharacter::new([677,41], [25,29]), //below line
            "z" => FontCharacter::new([706,41], [25,21]),

            "1" => FontCharacter::new([2,72], [25,29]),
            "2" => FontCharacter::new([31,72], [34,29]),
            "3" => FontCharacter::new([69,72], [34,29]),
            "4" => FontCharacter::new([107,72], [34,29]),
            "5" => FontCharacter::new([145,72], [34,29]),
            "6" => FontCharacter::new([183,72], [34,29]),
            "7" => FontCharacter::new([221,72], [34,29]),
            "8" => FontCharacter::new([259,71], [34,29]),
            "9" => FontCharacter::new([297,71], [34,29]),
            "0" => FontCharacter::new([335,71], [34,29]),
        }
    )
};

pub struct FontData
{
    texture_index: u32,
    whitespace: u32,
    kerning: u32,
    font_map: Map<&'static str, FontCharacter> 
}

impl FontData
{
    pub const fn new(texture_index : u32, whitespace: u32, kerning: u32, font_map: Map<&str, FontCharacter>) -> Self
    {
        Self {
            texture_index,
            whitespace,
            kerning,
            font_map
        }
    }

    pub fn get_texture_index(&self) -> u32
    {
        self.texture_index
    }

    pub fn get_whitespace_width(&self) -> u32
    {
        self.whitespace
    }
    
    pub fn get_kerning_width(&self) -> u32
    {
        self.kerning
    }

    pub fn get_character_data(&self, character: &str) -> Option<&FontCharacter>
    {
        let data = match self.font_map.get(character)
        {
            Some(d) => Some(d),
            None => { return None; }
        };

        data
    }
}

pub struct FontCharacter
{
    tex_coords: [i32;2],
    size: [i32;2],
}

impl FontCharacter
{
    pub const fn new(tex_coords: [i32;2], size: [i32;2]) -> Self
    {
        Self {
            tex_coords: tex_coords,
            size: size,
        }
    }

    pub fn get_size(&self) -> &[i32;2]
    {
        &&self.size
    }

    pub fn get_tex_coords(&self) -> &[i32;2]
    {
        &&self.tex_coords
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub enum Font
{
    Default
}

impl Font
{
    fn name(&self) -> &'static str 
    {
        match self {
            Font::Default => "default"
        }
    }

    pub fn get_whitespace_pixel_width(&self) -> Option<u32>
    {
        let font_data = match FONT_DATA.get(self.name())
        {
            Some(f) => f,
            None => {return None; }
        };

        Some(font_data.get_whitespace_width())
    }

    pub fn get_kerning_pixel_width(&self) -> Option<u32>
    {
        let font_data = match FONT_DATA.get(self.name())
        {
            Some(f) => f,
            None => {return None; }
        };

        Some(font_data.get_kerning_width())
    }

    pub fn get_texture_index(&self) -> Option<u32>
    {
        let font_data = match FONT_DATA.get(self.name())
        {
            Some(f) => f,
            None => {return None; }
        };

        Some(font_data.get_texture_index())
    }

    pub fn get_character_data<'a>(&self, character: &char) -> Option<&'a FontCharacter> {
        let font_data = match FONT_DATA.get(self.name())
        {
            Some(f) => f,
            None => {return None; }
        };

        font_data.get_character_data(character.to_string().as_str()) //TODO: make less ugly
    }
}


