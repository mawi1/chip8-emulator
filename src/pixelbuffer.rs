use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::window::Window;

pub struct PixelBufferSize {
    pub width: u32,
    pub height: u32,
    pub pixel_size: u32,
}

impl PixelBufferSize {
    fn logical_width(&self) -> u32 {
        self.width * self.pixel_size
    }

    fn logical_height(&self) -> u32 {
        self.height * self.pixel_size
    }

    pub fn logical_size(&self) -> LogicalSize<u32> {
        LogicalSize::new(self.logical_width(), self.logical_height())
    }
}

pub struct PixelBuffer {
    size: PixelBufferSize,
    on_color: [u8; 4],
    pixels: Pixels,
}

impl PixelBuffer {
    pub fn new(
        window: &Window,
        size: PixelBufferSize,
        on_color: (u8, u8, u8),
    ) -> anyhow::Result<Self> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        let pixels = Pixels::new(size.logical_width(), size.logical_height(), surface_texture)?;
        let on_color = [on_color.0, on_color.1, on_color.2, 0xff];

        Ok(PixelBuffer {
            size,
            on_color,
            pixels,
        })
    }

    pub fn set_pixels<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: Fn(usize, usize) -> bool,
    {
        let bytes_per_row = (self.size.logical_width() * 4 * self.size.pixel_size) as usize;

        for (y, pixel) in self
            .pixels
            .frame_mut()
            .chunks_exact_mut(bytes_per_row)
            .enumerate()
        {
            // set pixels for one line
            let mut line = Vec::with_capacity((self.size.logical_width() * 4) as usize);
            for x in 0..self.size.width as usize {
                let rgba = if f(x, y) {
                    self.on_color
                } else {
                    [0x0, 0x0, 0x0, 0xff]
                };
                // copy pixel pixel_size times
                for _ in 0..self.size.pixel_size {
                    line.extend_from_slice(&rgba);
                }
            }

            // copy that line pixel_size times into frame buffer
            for (px, src) in pixel
                .iter_mut()
                .zip(line.iter().cycle().take(bytes_per_row))
            {
                *px = *src;
            }
        }

        self.pixels.render()?;
        anyhow::Result::Ok(())
    }
}
