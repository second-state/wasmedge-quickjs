/**
 * 
 * @param {unknown} expr 
 * @param {string} msg 
 */
export function assert(expr, msg = "") {
    if (!expr) {
        throw new Error(msg);
    }
}
