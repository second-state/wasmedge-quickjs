use std::str::FromStr;

use chat_prompts::{
    chat::{BuildChatPrompt, ChatPrompt},
    PromptTemplateType,
};
use endpoints::chat::{ChatCompletionRequest, ChatCompletionRequestMessage, ChatCompletionRole};
use wasi_nn::BackendError;

use crate::{
    register_class, AsObject, Context, JsClassDef, JsClassTool, JsModuleDef, JsObject, JsValue,
    SelfRefJsValue,
};

struct WasiNNGraph(wasi_nn::Graph);

impl JsClassDef for WasiNNGraph {
    type RefType = WasiNNGraph;

    const CLASS_NAME: &'static str = "Graph";

    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] =
        &[("init_execution_context", 0, Self::js_init_execution_context)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(
        _ctx: &mut crate::Context,
        _argv: &[JsValue],
    ) -> Result<Self::RefType, JsValue> {
        Err(JsValue::UnDefined)
    }
}

impl WasiNNGraph {
    pub fn js_init_execution_context(
        &mut self,
        this: &mut JsObject,
        js_ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        let r = Self::self_ref_opaque_mut(this.clone().into(), |v| v.0.init_execution_context());
        match r {
            None => JsValue::UnDefined,
            Some(Ok(ctx)) => {
                WasiNNGraphExecutionContext::wrap_obj(js_ctx, WasiNNGraphExecutionContext { ctx })
            }
            Some(Err(e)) => {
                let err = ggml_error_to_js_error(js_ctx, e);
                js_ctx.throw_error(err).into()
            }
        }
    }
}

struct WasiNNGraphExecutionContext {
    ctx: SelfRefJsValue<WasiNNGraph, wasi_nn::GraphExecutionContext<'static>>,
}

impl JsClassDef for WasiNNGraphExecutionContext {
    type RefType = Self;

    const CLASS_NAME: &'static str = "GraphExecutionContext";

    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[
        ("set_input", 4, Self::js_set_input),
        ("compute", 0, Self::js_compute),
        ("compute_single", 0, Self::js_compute_single),
        ("fini_single", 0, Self::js_fini_single),
        ("get_output", 2, Self::js_get_output),
        ("get_output_single", 2, Self::js_get_output_single),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        Err(JsValue::UnDefined)
    }
}

lazy_static::lazy_static! {
    static ref MAX_OUTPUT_SIZE: usize ={
        std::env::var("GGML_OUTPUT_BUFF_SIZE")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1024)
    };
}

fn ggml_error_to_js_error(ctx: &mut Context, error: wasi_nn::Error) -> JsValue {
    let (t, msg) = match error {
        wasi_nn::Error::IoError(e) => {
            let mut js_err = ctx.new_error(e.to_string().as_str());
            if let JsValue::Object(js_err) = &mut js_err {
                js_err.set("type", ctx.new_string("IO").into());
            };
            return js_err;
        }
        wasi_nn::Error::BackendError(BackendError::InvalidArgument) => {
            ("BackendError", "InvalidArgument")
        }
        wasi_nn::Error::BackendError(BackendError::InvalidEncoding) => {
            ("BackendError", "InvalidEncoding")
        }
        wasi_nn::Error::BackendError(BackendError::MissingMemory) => {
            ("BackendError", "MissingMemory")
        }
        wasi_nn::Error::BackendError(BackendError::Busy) => ("BackendError", "Busy"),
        wasi_nn::Error::BackendError(BackendError::RuntimeError) => {
            ("BackendError", "RuntimeError")
        }
        wasi_nn::Error::BackendError(BackendError::UnsupportedOperation) => {
            ("BackendError", "UnsupportedOperation")
        }
        wasi_nn::Error::BackendError(BackendError::TooLarge) => ("BackendError", "TooLarge"),
        wasi_nn::Error::BackendError(BackendError::NotFound) => ("BackendError", "NotFound"),
        wasi_nn::Error::BackendError(BackendError::EndOfSequence) => {
            ("BackendError", "EndOfSequence")
        }
        wasi_nn::Error::BackendError(BackendError::ContextFull) => ("BackendError", "ContextFull"),
        wasi_nn::Error::BackendError(BackendError::PromptTooLong) => {
            ("BackendError", "PromptTooLong")
        }
        wasi_nn::Error::BackendError(BackendError::UnknownError(i)) => {
            let mut js_err = ctx.new_error(format!("UnknownError:{i}").as_str());
            if let JsValue::Object(js_err) = &mut js_err {
                js_err.set("type", ctx.new_string("BackendError").into());
            };
            return js_err;
        }
    };
    let mut js_err = ctx.new_error(msg);
    if let JsValue::Object(js_err) = &mut js_err {
        js_err.set("type", ctx.new_string(t).into());
    };
    js_err
}

impl WasiNNGraphExecutionContext {
    fn js_set_input(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let index = if let Some(JsValue::Int(index)) = argv.get(0) {
            *index as usize
        } else {
            return ctx.throw_type_error("'index' must be of type int").into();
        };

        let tensor_buf = match argv.get(1) {
            Some(JsValue::ArrayBuffer(buf)) => buf.as_ref(),
            Some(JsValue::String(s)) => s.as_str().trim().as_bytes(),
            _ => {
                return ctx
                    .throw_type_error("'tensor_buf' must be of type buffer or string")
                    .into();
            }
        };

        let dimensions = if let Some(JsValue::Array(arr)) = argv.get(2) {
            match arr.to_vec() {
                Ok(dimensions) => {
                    let mut dimension_arr = Vec::with_capacity(dimensions.len());

                    for i in dimensions {
                        let v = match i {
                            JsValue::Int(i) => i as usize,
                            JsValue::Float(i) => i as usize,
                            _ => {
                                return ctx
                                    .throw_type_error("'dimensions' must be of type number array")
                                    .into()
                            }
                        };
                        dimension_arr.push(v);
                    }
                    dimension_arr
                }
                Err(e) => return e.into(),
            }
        } else {
            return ctx
                .throw_type_error("'dimensions' must be of type array")
                .into();
        };

        let tensor_type = if let Some(JsValue::Int(input_type)) = argv.get(3) {
            let input_type = *input_type;
            match input_type {
                0 => wasi_nn::TensorType::F16,
                1 => wasi_nn::TensorType::F32,
                2 => wasi_nn::TensorType::F64,
                3 => wasi_nn::TensorType::U8,
                4 => wasi_nn::TensorType::I32,
                5 => wasi_nn::TensorType::I64,

                _ => {
                    return ctx
                        .throw_type_error(&format!("undefined `input_type` {}", input_type))
                        .into();
                }
            }
        } else {
            return ctx.throw_type_error("'index' must be of type int").into();
        };

        if let Err(e) = self
            .ctx
            .set_input(index, tensor_type, &dimensions, tensor_buf)
        {
            let err = ggml_error_to_js_error(ctx, e);
            ctx.throw_error(err).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn js_compute(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        if let Err(e) = self.ctx.compute() {
            let err = ggml_error_to_js_error(ctx, e);
            ctx.throw_error(err).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn js_compute_single(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        if let Err(e) = self.ctx.compute_single() {
            let err = ggml_error_to_js_error(ctx, e);
            ctx.throw_error(err).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn js_fini_single(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        if let Err(e) = self.ctx.fini_single() {
            let err = ggml_error_to_js_error(ctx, e);
            ctx.throw_error(err).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn js_get_output_single(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let index = if let Some(JsValue::Int(index)) = argv.get(0) {
            *index as usize
        } else {
            return ctx.throw_type_error("'index' must be of type int").into();
        };

        let output_type = if let Some(JsValue::Int(type_index)) = argv.get(1) {
            *type_index
        } else {
            return ctx
                .throw_type_error("'output_type' must be of type Int")
                .into();
        };

        let mut output_buffer = vec![0u8; *MAX_OUTPUT_SIZE];

        match self.ctx.get_output_single(index, output_buffer.as_mut()) {
            Ok(n) => match output_type {
                0 => ctx.new_array_buffer(&output_buffer[0..n]).into(),
                _ => ctx
                    .new_string(unsafe { std::str::from_utf8_unchecked(&output_buffer[0..n]) })
                    .into(),
            },
            Err(e) => {
                let err = ggml_error_to_js_error(ctx, e);
                ctx.throw_error(err).into()
            }
        }
    }

    fn js_get_output(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let index = if let Some(JsValue::Int(index)) = argv.get(0) {
            *index as usize
        } else {
            return ctx.throw_type_error("'index' must be of type int").into();
        };

        let mut output = if let Some(JsValue::ArrayBuffer(buf)) = argv.get(1) {
            buf.clone()
        } else {
            return ctx
                .throw_type_error("'output' must be of type buffer")
                .into();
        };

        match self.ctx.get_output(index, output.as_mut()) {
            Ok(n) => JsValue::Int(n as i32),
            Err(e) => {
                let err = ggml_error_to_js_error(ctx, e);
                ctx.throw_error(err).into()
            }
        }
    }
}

fn js_build_graph_from_cache(ctx: &mut Context, _this: JsValue, param: &[JsValue]) -> JsValue {
    if let Some(
        [JsValue::Int(target_index), JsValue::String(metadata), JsValue::String(module_name)],
    ) = param.get(0..3)
    {
        let target = match *target_index {
            0 => wasi_nn::ExecutionTarget::CPU,
            1 => wasi_nn::ExecutionTarget::GPU,
            2 => wasi_nn::ExecutionTarget::TPU,
            _ => wasi_nn::ExecutionTarget::AUTO,
        };
        let config = wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, target)
            .config(metadata.to_string())
            .build_from_cache(module_name.as_str());

        match config {
            Ok(g) => WasiNNGraph::wrap_obj(ctx, WasiNNGraph(g)),
            Err(e) => {
                let err = ggml_error_to_js_error(ctx, e);
                ctx.throw_error(err).into()
            }
        }
    } else {
        JsValue::UnDefined
    }
}

pub fn init_wasi_nn_ggml_module(ctx: &mut Context) {
    ctx.register_fn_module(
        "_wasi_nn_ggml",
        &[
            WasiNNGraph::CLASS_NAME,
            WasiNNGraphExecutionContext::CLASS_NAME,
            "build_graph_from_cache",
        ],
        |ctx, m| {
            let class_ctor = register_class::<WasiNNGraph>(ctx);
            m.add_export(WasiNNGraph::CLASS_NAME, class_ctor);

            let class_ctor = register_class::<WasiNNGraphExecutionContext>(ctx);
            m.add_export(WasiNNGraphExecutionContext::CLASS_NAME, class_ctor);

            let f = ctx.wrap_function("build_graph_from_cache", js_build_graph_from_cache);
            m.add_export("build_graph_from_cache", f.into());
        },
    )
}

struct GGMLChatPromptTemplate {
    prompt: ChatPrompt,
}

fn create_prompt_template(template_ty: PromptTemplateType) -> ChatPrompt {
    match template_ty {
        PromptTemplateType::Llama2Chat => {
            ChatPrompt::Llama2ChatPrompt(chat_prompts::chat::llama::Llama2ChatPrompt::default())
        }
        PromptTemplateType::MistralInstruct => ChatPrompt::MistralInstructPrompt(
            chat_prompts::chat::mistral::MistralInstructPrompt::default(),
        ),
        PromptTemplateType::MistralLite => {
            ChatPrompt::MistralLitePrompt(chat_prompts::chat::mistral::MistralLitePrompt::default())
        }
        PromptTemplateType::OpenChat => {
            ChatPrompt::OpenChatPrompt(chat_prompts::chat::openchat::OpenChatPrompt::default())
        }
        PromptTemplateType::CodeLlama => ChatPrompt::CodeLlamaInstructPrompt(
            chat_prompts::chat::llama::CodeLlamaInstructPrompt::default(),
        ),
        PromptTemplateType::BelleLlama2Chat => ChatPrompt::BelleLlama2ChatPrompt(
            chat_prompts::chat::belle::BelleLlama2ChatPrompt::default(),
        ),
        PromptTemplateType::VicunaChat => {
            ChatPrompt::VicunaChatPrompt(chat_prompts::chat::vicuna::VicunaChatPrompt::default())
        }
        PromptTemplateType::Vicuna11Chat => {
            ChatPrompt::Vicuna11ChatPrompt(chat_prompts::chat::vicuna::Vicuna11ChatPrompt::default())
        }
        PromptTemplateType::ChatML => {
            ChatPrompt::ChatMLPrompt(chat_prompts::chat::chatml::ChatMLPrompt::default())
        }
        PromptTemplateType::Baichuan2 => ChatPrompt::Baichuan2ChatPrompt(
            chat_prompts::chat::baichuan::Baichuan2ChatPrompt::default(),
        ),
        PromptTemplateType::WizardCoder => {
            ChatPrompt::WizardCoderPrompt(chat_prompts::chat::wizard::WizardCoderPrompt::default())
        }
        PromptTemplateType::Zephyr => {
            ChatPrompt::ZephyrChatPrompt(chat_prompts::chat::zephyr::ZephyrChatPrompt::default())
        }
        PromptTemplateType::IntelNeural => {
            ChatPrompt::NeuralChatPrompt(chat_prompts::chat::intel::NeuralChatPrompt::default())
        }
        PromptTemplateType::DeepseekChat => ChatPrompt::DeepseekChatPrompt(
            chat_prompts::chat::deepseek::DeepseekChatPrompt::default(),
        ),
        PromptTemplateType::DeepseekCoder => ChatPrompt::DeepseekCoderPrompt(
            chat_prompts::chat::deepseek::DeepseekCoderPrompt::default(),
        ),
        PromptTemplateType::SolarInstruct => ChatPrompt::SolarInstructPrompt(
            chat_prompts::chat::solar::SolarInstructPrompt::default(),
        ),
    }
}

impl GGMLChatPromptTemplate {
    fn js_build(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some(JsValue::Object(js_obj)) = argv.first() {
            let mut js_obj = js_obj.clone().into();
            if let Some(req) = GGMLChatCompletionRequest::opaque_mut(&mut js_obj) {
                return match self.prompt.build(&mut req.req.messages) {
                    Ok(s) => ctx.new_string(s.as_str()).into(),
                    Err(e) => {
                        let error = ctx.new_error(e.to_string().as_str());
                        ctx.throw_error(error).into()
                    }
                };
            }
        }
        ctx.throw_type_error("'request' must be of type GGMLChatCompletionRequest")
            .into()
    }
}

impl JsClassDef for GGMLChatPromptTemplate {
    type RefType = GGMLChatPromptTemplate;

    const CLASS_NAME: &'static str = "GGMLChatPrompt";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[("build", 1, Self::js_build)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        if let Some(JsValue::String(type_str)) = argv.first() {
            match PromptTemplateType::from_str(type_str.as_str()) {
                Ok(template_ty) => Ok(Self {
                    prompt: create_prompt_template(template_ty),
                }),
                Err(_) => Err(JsValue::UnDefined),
            }
        } else {
            Err(ctx
                .throw_type_error("'tensor_buf' must be of type buffer or string")
                .into())
        }
    }
}

struct GGMLChatCompletionRequest {
    req: ChatCompletionRequest,
}

impl GGMLChatCompletionRequest {
    fn js_push_message(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some([JsValue::String(role), JsValue::String(content)]) = argv.get(0..2) {
            let role =
                match role.as_str() {
                    "system" => ChatCompletionRole::System,
                    "user" => ChatCompletionRole::User,
                    "function" => ChatCompletionRole::Function,
                    "assistant" => ChatCompletionRole::Assistant,
                    _ => return ctx
                        .throw_type_error(
                            "`role` must be either `system`, `user`, `assistant`, or `function`.",
                        )
                        .into(),
                };
            self.req
                .messages
                .push(ChatCompletionRequestMessage::new(role, content.as_str()));
            JsValue::UnDefined
        } else {
            JsValue::UnDefined
        }
    }
}

impl JsClassDef for GGMLChatCompletionRequest {
    type RefType = GGMLChatCompletionRequest;

    const CLASS_NAME: &'static str = "GGMLChatCompletionRequest";

    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] =
        &[("push_message", 2, Self::js_push_message)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        Ok(Self {
            req: ChatCompletionRequest::default(),
        })
    }
}

pub fn init_ggml_template_module(ctx: &mut Context) {
    ctx.register_fn_module(
        "_wasi_nn_ggml_template",
        &[
            GGMLChatCompletionRequest::CLASS_NAME,
            GGMLChatPromptTemplate::CLASS_NAME,
        ],
        |ctx, m| {
            let class_ctor = register_class::<GGMLChatCompletionRequest>(ctx);
            m.add_export(GGMLChatCompletionRequest::CLASS_NAME, class_ctor);

            let class_ctor = register_class::<GGMLChatPromptTemplate>(ctx);
            m.add_export(GGMLChatPromptTemplate::CLASS_NAME, class_ctor);
        },
    )
}
