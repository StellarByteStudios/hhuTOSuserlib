// Verknüpfung auf den Syscall um ein Bild zu malen

use crate::graphix::picturepainting::animate::Frame;
use crate::kernel::syscall::user_api::usr_paint_picture_on_pos;

pub fn draw_picture(x: usize, y: usize, picture: &Frame) {
    usr_paint_picture_on_pos(
        x,
        y,
        picture.width as usize,
        picture.height as usize,
        picture.bpp as usize,
        picture.data.as_ptr(),
    )
}
