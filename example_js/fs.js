import { statSync } from "fs";

print("\nfs.statSync\nExisted File:");

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
        print(key, ": ", typeof(val) === "function" ? val() : val);
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

import { constants } from "fs";

print("\nfs.constants:");

const { F_OK, O_WRONLY } = constants;
print("F_OK: ", F_OK);
print("O_WRONLY: ", O_WRONLY);

import { lstatSync } from "fs";

print("\nfs.lstatSync\nExisted File:");

try {
    let s = lstatSync("README.md");
    print(JSON.stringify(s));
} catch (err) {
    print(JSON.stringify(err));
}

print("\nExisted File with BigInt:");

try {
    let s = lstatSync("README.md", { bigint: true });
    for (const [key, val] of Object.entries(s)) {
        print(key, ": ", typeof(val) === "function" ? val() : val);
    }
} catch (err) {
    print(JSON.stringify(err));
}

print("\nNon-existed File with BigInt:");

try {
    let s = lstatSync("non-exist.file");
    print(JSON.stringify(s));
} catch (err) {
    print(JSON.stringify(err));
}

print("\nNon-existed File with Throw:");

try {
    let s = lstatSync("non-exist.file", { throwIfNoEntry: false });
    print(JSON.stringify(s));
} catch (err) {
    print(err.name);
    print(err.stack);
    print(err.message);
}