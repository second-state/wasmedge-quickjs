mod generated;
use generated as wasi_nn;

use wasi_nn::NnErrno;

use crate::{register_class, Context, JsClassDef, JsClassTool, JsObject, JsValue};

#[derive(Debug, Clone, Copy)]
struct NnGraph(wasi_nn::Graph);
impl NnGraph {
    pub fn load(
        weights: &[&[u8]],
        encoding: wasi_nn::GraphEncoding,
        execution_target: wasi_nn::ExecutionTarget,
    ) -> Result<Self, NnErrno> {
        unsafe {
            let graph = wasi_nn::load(weights, encoding, execution_target)?;
            Ok(Self(graph))
        }
    }
}

struct NnContext {
    ctx: wasi_nn::GraphExecutionContext,
}

impl NnContext {
    pub fn init(graph: &NnGraph) -> Result<Self, NnErrno> {
        unsafe {
            Ok(NnContext {
                ctx: wasi_nn::init_execution_context(graph.0)?,
            })
        }
    }

    pub fn set_input(&mut self, index: u32, tensor: wasi_nn::Tensor) -> Result<(), NnErrno> {
        unsafe { wasi_nn::set_input(self.ctx, index, tensor) }
    }

    pub fn compute(&mut self) -> Result<(), NnErrno> {
        unsafe { wasi_nn::compute(self.ctx) }
    }

    pub fn get_output(&mut self, index: u32, output: &mut [u8]) -> Result<u32, NnErrno> {
        unsafe {
            let out_buffer = output.as_mut_ptr();
            let out_buffer_max_size = output.len();
            wasi_nn::get_output(self.ctx, index, out_buffer, out_buffer_max_size as u32)
        }
    }
}

impl JsClassDef for NnGraph {
    type RefType = NnGraph;

    const CLASS_NAME: &'static str = "NnGraph";

    const CONSTRUCTOR_ARGC: u8 = 3;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(
        ctx: &mut crate::Context,
        argv: &[crate::JsValue],
    ) -> Result<Self::RefType, crate::JsValue> {
        if let Some(
            [JsValue::Array(weights), JsValue::String(encoding), JsValue::String(execution_target)],
        ) = argv.get(0..3)
        {
            let weights = weights.to_vec().map_err(|e| JsValue::Exception(e))?;
            let mut weights_vec = Vec::with_capacity(weights.len());

            for weight in &weights {
                if let JsValue::ArrayBuffer(buffer) = weight {
                    weights_vec.push(buffer.as_ref());
                }
            }

            let encoding = match encoding.as_str() {
                "openvino" => wasi_nn::GRAPH_ENCODING_OPENVINO,
                "onnx" => wasi_nn::GRAPH_ENCODING_ONNX,
                "pytorch" => wasi_nn::GRAPH_ENCODING_PYTORCH,
                "tensorflow" => wasi_nn::GRAPH_ENCODING_TENSORFLOW,
                "tensorflowlite" | "tensorflow_lite" | "tensorflow-lite" => {
                    wasi_nn::GRAPH_ENCODING_TENSORFLOWLITE
                }
                _ => return Err(JsValue::UnDefined),
            };
            let execution_target = match execution_target.as_str() {
                "cpu" => wasi_nn::EXECUTION_TARGET_CPU,
                "gpu" => wasi_nn::EXECUTION_TARGET_GPU,
                "tpu" => wasi_nn::EXECUTION_TARGET_TPU,
                _ => return Err(JsValue::UnDefined),
            };
            NnGraph::load(&weights_vec, encoding, execution_target)
                .map_err(|e| ctx.throw_internal_type_error(e.message()).into())
        } else {
            Err(JsValue::UnDefined)
        }
    }
}

impl NnContext {
    fn js_set_input(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let index = if let Some(JsValue::Int(index)) = argv.get(0) {
            *index
        } else {
            return ctx.throw_type_error("'index' must be of type int").into();
        };

        let tensor_buf = if let Some(JsValue::ArrayBuffer(buf)) = argv.get(1) {
            buf.as_ref()
        } else {
            return ctx
                .throw_type_error("'tensor_buf' must be of type buffer")
                .into();
        };

        let dimensions = if let Some(JsValue::Array(arr)) = argv.get(2) {
            match arr.to_vec() {
                Ok(dimensions) => {
                    let mut dimension_arr = Vec::with_capacity(dimensions.len());

                    for i in dimensions {
                        let v = match i {
                            JsValue::Int(i) => i as u32,
                            JsValue::Float(i) => i as u32,
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

        let input_type = if let Some(JsValue::Int(input_type)) = argv.get(3) {
            let input_type = *input_type;
            match input_type {
                0 => wasi_nn::TENSOR_TYPE_F16,
                1 => wasi_nn::TENSOR_TYPE_F32,
                2 => wasi_nn::TENSOR_TYPE_U8,
                3 => wasi_nn::TENSOR_TYPE_I32,
                _ => {
                    return ctx
                        .throw_type_error(&format!("undefined `input_type` {}", input_type))
                        .into();
                }
            }
        } else {
            return ctx.throw_type_error("'index' must be of type int").into();
        };

        let tensor = wasi_nn::Tensor {
            dimensions: &dimensions,
            type_: input_type,
            data: tensor_buf,
        };

        if let Err(e) = self.set_input(index as u32, tensor) {
            return ctx.throw_internal_type_error(e.message()).into();
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
        if let Err(e) = self.compute() {
            ctx.throw_internal_type_error(e.message()).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn js_get_output(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let index = if let Some(JsValue::Int(index)) = argv.get(0) {
            *index
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

        match self.get_output(index as u32, output.as_mut()) {
            Ok(n) => JsValue::Int(n as i32),
            Err(e) => ctx.throw_internal_type_error(e.message()).into(),
        }
    }
}

impl JsClassDef for NnContext {
    type RefType = Self;

    const CLASS_NAME: &'static str = "NnContext";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[
        ("setInput", 4, Self::js_set_input),
        ("compute", 0, Self::js_compute),
        ("getOutput", 2, Self::js_get_output),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(
        ctx: &mut crate::Context,
        argv: &[JsValue],
    ) -> Result<Self::RefType, JsValue> {
        if let Some(graph) = argv.get(0) {
            if let Some(graph) = NnGraph::opaque(graph) {
                Self::init(graph).map_err(|e| ctx.throw_internal_type_error(e.message()).into())
            } else {
                return Err(ctx
                    .throw_type_error("'graph' must be of type 'NnGraph'")
                    .into());
            }
        } else {
            Err(JsValue::UnDefined)
        }
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_fn_module(
        "wasi_nn\0",
        &[
            NnGraph::CLASS_NAME,
            NnContext::CLASS_NAME,
            "TENSOR_TYPE_F16",
            "TENSOR_TYPE_F32",
            "TENSOR_TYPE_U8",
            "TENSOR_TYPE_I32",
        ],
        |ctx, m| {
            let class_ctor = register_class::<NnGraph>(ctx);
            m.add_export(NnGraph::CLASS_NAME, class_ctor);

            let class_ctor = register_class::<NnContext>(ctx);
            m.add_export(NnContext::CLASS_NAME, class_ctor);

            m.add_export(
                "TENSOR_TYPE_F16",
                JsValue::Int(wasi_nn::TENSOR_TYPE_F16.raw() as i32),
            );
            m.add_export(
                "TENSOR_TYPE_F32",
                JsValue::Int(wasi_nn::TENSOR_TYPE_F32.raw() as i32),
            );
            m.add_export(
                "TENSOR_TYPE_U8",
                JsValue::Int(wasi_nn::TENSOR_TYPE_U8.raw() as i32),
            );
            m.add_export(
                "TENSOR_TYPE_I32",
                JsValue::Int(wasi_nn::TENSOR_TYPE_I32.raw() as i32),
            );
        },
    )
}
