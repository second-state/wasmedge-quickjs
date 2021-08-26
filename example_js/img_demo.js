import {Image} from 'image'

print('hello img')
let img = new Image("./example_js/bird.png")
print('img',img)
img.save_to_file("./example_js/bird_new.png")