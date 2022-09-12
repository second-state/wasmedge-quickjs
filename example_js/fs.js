import { statSync } from "fs";

print("\nExisted File:");

try {
    let s = statSync("README.md");
    print(JSON.stringify(s));
} catch (err) {
    print(JSON.stringify(err));
}

print("\nExisted File with BigInt:");

try {
    let s = statSync("README.md", { bigint: true });
    for (const [key, val] of Object.entries(s)) {
        print(key, ": ", val);
    }
} catch (err) {
    print(JSON.stringify(err));
}

print("\nNon-existed File with BigInt:");

try {
    let s = statSync("non-exist.file");
    print(JSON.stringify(s));
} catch (err) {
    print(JSON.stringify(err));
}

print("\nNon-existed File with Throw:");

try {
    let s = statSync("non-exist.file", { throwIfNoEntry: false });
    print(JSON.stringify(s));
} catch (err) {
    print(err.name);
    print(err.stack);
    print(err.message);
}