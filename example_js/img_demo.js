import {Image} from 'image'
import * as std from 'std'

// load image from file_path
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

// save image with native function
img.save_to_file("./example_js/bird_new.png")

// new image from memory
let new_img = new Image(buf);

// image resize
let resize_img = img.resize(100,100)

// draw rect
resize_img.draw_hollow_rect(0,0,50,50,0xff0000)
resize_img.draw_filled_rect(50,50,50,50,0x0000ff)
resize_img.draw_hollow_rect(0,50,50,50,0x00ff00)

resize_img.save_to_file("./example_js/bird_new_2.jpg");