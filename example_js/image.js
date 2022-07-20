import {Image} from 'image';
let img = new Image(__dirname + '/bird.png');
let img_luma = img.to_luma();
img_luma.save_to_file(__dirname + '/bird_luma.png')
