use image::ImageReader;
use std::env;
use std::path::PathBuf;

pub struct RtwImage {
    width: u32,             // Loaded image width
    height: u32,            // Loaded image height
    bdata: Option<Vec<u8>>, // Linear 8-bit pixel data
}

impl RtwImage {
    pub fn new(image_filename: &str) -> Self {
        // Loads image data from the specified file. If the RTW_IMAGES environment variable is
        // defined, looks only in that directory for the image file. If the image was not found,
        // searches for the specified image file first from the current directory, then in the
        // images/ subdirectory, then the _parent's_ images/ subdirectory, and then _that_
        // parent, on so on, for six levels up. If the image was not loaded successfully,
        // width() and height() will return 0.

        let filename = PathBuf::from(image_filename);
        let imagedir = env::var("RTW_IMAGES").ok();

        // Hunt for the image file in some likely locations.
        let search_paths = vec![
            imagedir.map(|dir| PathBuf::from(dir).join(&filename)),
            Some(filename.clone()),
            Some(PathBuf::from("images").join(&filename)),
            Some(PathBuf::from("../images").join(&filename)),
            Some(PathBuf::from("../../images").join(&filename)),
            Some(PathBuf::from("../../../images").join(&filename)),
            Some(PathBuf::from("../../../../images").join(&filename)),
            Some(PathBuf::from("../../../../../images").join(&filename)),
            Some(PathBuf::from("../../../../../../images").join(&filename)),
        ];

        for path in search_paths.into_iter().flatten() {
            if let Some(image) = Self::load(&path) {
                return image;
            }
        }

        panic!("ERROR: Could not load image file '{image_filename}'.");
    }

    fn load(path: &PathBuf) -> Option<Self> {
        // Loads the linear (gamma=1) image data from the given file name. Returns true if the
        // load succeeded. The resulting data buffer contains the three [0.0, 1.0]
        // floating-point values for the first pixel (red, then green, then blue). Pixels are
        // contiguous, going left to right for the width of the image, followed by the next row
        // below, for the full height of the image.

        if !path.exists() {
            return None;
        }

        let img = ImageReader::open(path).ok()?.decode().ok()?;
        let rgb_image = img.to_rgb8();
        let (width, height) = rgb_image.dimensions();
        let bdata = rgb_image.into_raw();

        Some(Self {
            width,
            height,
            bdata: Some(bdata),
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn no_data(&self) -> bool {
        self.bdata.is_none()
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> &[u8] {
        // Return the slice of the three RGB bytes of the pixel at x,y. If there is no image
        // data, return magenta.

        static MAGENTA: &[u8] = &[255, 0, 255];
        if let Some(bdata) = &self.bdata {
            if x >= self.width || y >= self.height {
                return MAGENTA;
            }
            let index = ((y * self.width + x) * 3) as usize;
            &bdata[index..index + 3]
        } else {
            MAGENTA
        }
    }
}

// Usage:
// let image = RtwImage::new("example.png").expect("Image failed to load");
