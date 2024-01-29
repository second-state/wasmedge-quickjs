import { GGMLChatCompletionRequest, GGMLChatPrompt } from '_wasi_nn_ggml_template'
import { build_graph_from_cache } from '_wasi_nn_ggml'
function main() {
    let opt = {
        "enable-log": true,
        "ctx_size": 512,
        "n-predict": 1024,
        "n-gpu-layers": 100,
        "batch-size": 512,
        "temp": 0.8,
        "repeat-penalty": 1.1
    }

    let graph = build_graph_from_cache(3, JSON.stringify(opt), "default")
    let context = graph.init_execution_context()

    let template = new GGMLChatPrompt('llama-2-chat')

    let req = new GGMLChatCompletionRequest()

    let messages = ['hello', 'who are you?']

    for (var i in messages) {
        print("[YOU]:", messages[i])
        req.push_message("user", messages[i])
        let p = template.build(req)
        context.set_input(0, p, [1], 3)
        var ss = '';

        while (1) {
            try {
                context.compute_single()
                let s = context.get_output_single(0, 1)
                ss += s;
                print('BOT:', s)
            } catch (e) {
                if (e['type'] == "BackendError" && e['message'] == "EndOfSequence") {
                    print('[log] EndOfSequence!')
                    break
                } else if (e['type'] == "BackendError" && e['message'] == "ContextFull") {
                    print('[log] ContextFull!')
                    break
                } else {
                    return
                }
            }
        }
        req.push_message("assistant", ss)
        print("[BOT]:", ss);
    }

    let p = template.build(req);
    print()
    print(p)
}

main()