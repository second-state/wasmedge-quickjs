import {Image} from 'image'
import * as std from 'std'

// load image from file_path
print('start load image')
let img = new Image("./example_js/bird.png")

// image to ArrayBuffer
let buf = img.save_to_buf('png')
print('buf:',buf)

// save buf to file
let f = std.open('./example_js/bird_new_1.png','w')
let x = f.write(buf,0,buf.byteLength)
print(x)
f.flush()
f.close()

// new image from memory
print('new image from memory')
let new_img = new Image(buf);

// image resize
let resize_img = img.resize(100,100)

// draw rect
resize_img.draw_hollow_rect(0,0,50,50,0xff0000)
resize_img.draw_filled_rect(50,50,50,50,0x0000ff)
resize_img.draw_hollow_rect(0,50,50,50,0x00ff00)

// save image with native function
resize_img.save_to_file("./example_js/bird_new_2.jpg");

// get pixels and to rgb bgr luma
{
    let pix = resize_img.pixels()
    print('rgba pixels len',pix.byteLength)

    let rgb_img = resize_img.to_rgb()
    let rgb_pix = rgb_img.pixels()
    print('rgb pixels len',rgb_pix.byteLength)

    let luma_img = resize_img.to_luma()
    let luma_pix = luma_img.pixels()
    print('luma pixels len',luma_pix.byteLength)

    luma_img.draw_hollow_rect(0,0,50,50,0xff0000)
    luma_img.save_to_file("./example_js/bird_luma.png")
}