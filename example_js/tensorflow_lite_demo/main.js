import {TensorflowLiteSession} from 'tensorflow_lite'
import {Image} from 'image'

let img = new Image('./example_js/tensorflow_lite_demo/food.jpg')
let img_rgb = img.to_rgb().resize(192,192)
let rgb_pix = img_rgb.pixels()

let session = new TensorflowLiteSession('./example_js/tensorflow_lite_demo/lite-model_aiy_vision_classifier_food_V1_1.tflite')
session.add_input('input',rgb_pix)
session.run()
let output = session.get_output('MobilenetV1/Predictions/Softmax');
let output_view = new Uint8Array(output)
let max = 0;
let max_idx = 0;
for (var i in output_view){
    let v = output_view[i]
    if(v>max){
        max = v;
        max_idx = i;
    }
}
print(max,max_idx)