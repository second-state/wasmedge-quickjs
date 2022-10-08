import uv from "../../internal_binding/uv";

export function internalBinding(mod) {
    if (mod === "uv") {
        return uv;
    }
}