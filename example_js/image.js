import {Image} from 'image';
let img = new Image('bird.png');
let img_luma = img.to_luma();
img_luma.save_to_file('bird_luma.png')