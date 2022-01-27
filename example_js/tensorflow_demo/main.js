import {Image} from 'image';
import * as std from 'std';
import {TensorflowSession} from 'tensorflow';

let img = new Image('bird.png');
let img_rgb = img.to_rgb().resize(224, 224);
let rgb_pix = img_rgb.pixels_32f();

let session = new TensorflowSession('mobilenet_v2_1.4_224_frozen.pb');
session.add_input_32f('input', rgb_pix, [1, 224, 224, 3]);
session.add_output('MobilenetV2/Predictions/Softmax');
session.run();
let output = session.get_output('MobilenetV2/Predictions/Softmax');
let output_view = new Float32Array(output);
let max = 0;
let max_idx = 0;
for (var i in output_view) {
  let v = output_view[i];
  if (v > max) {
    max = v;
    max_idx = i;
  }
}
let label_file = std.open('imagenet_slim_labels.txt', 'r');
let label = '';
for (var i = 0; i <= max_idx; i++) {
  label = label_file.getline();
}
label_file.close();

print('label:');
print(label);
print('confidence:');
print(max);
