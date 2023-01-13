import { Image } from 'image';
import * as fs from 'fs';
import { NnGraph, NnContext, TENSOR_TYPE_U8 } from 'wasi_nn';

let img = new Image(__dirname + '/food.jpg');
let img_rgb = img.to_rgb().resize(192, 192);
let rgb_pix = img_rgb.pixels();

let data = fs.readFileSync(__dirname + '/lite-model_aiy_vision_classifier_food_V1_1.tflite')
let graph = new NnGraph([data.buffer], "tensorflowlite", "cpu");
let context = new NnContext(graph);
context.setInput(0, rgb_pix, [1, 192, 192, 3], TENSOR_TYPE_U8);
context.compute();

let output_view = new Uint8Array(2024);
context.getOutput(0, output_view.buffer)

let max = 0;
let max_idx = 0;
for (var i in output_view) {
    let v = output_view[i];
    if (v > max) {
        max = v;
        max_idx = i;
    }
}

let label_file = fs.readFileSync(__dirname + '/aiy_food_V1_labelmap.txt', 'utf-8');
let lables = label_file.split(/\r?\n/);

let label = lables[max_idx]

print('label:');
print(label);
print('confidence:');
print(max / 255);