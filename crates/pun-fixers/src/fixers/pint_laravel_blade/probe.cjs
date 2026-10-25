const path = require("path");
const fs = require("fs");

const probe = (pkg) => {
    let entry;
    try {
        entry = require.resolve(pkg, { paths: [process.cwd()] });
    } catch (e) {
        return null;
    }
    let dir = path.dirname(entry);
    while (dir !== path.dirname(dir)) {
        const manifest = path.join(dir, "package.json");
        if (fs.existsSync(manifest)) {
            try {
                const json = JSON.parse(fs.readFileSync(manifest, "utf8"));
                if (json.name === pkg && json.version) {
                    return String(json.version).trim();
                }
            } catch (e) {}
        }
        dir = path.dirname(dir);
    }
    return "";
};

fs.writeSync(1, JSON.stringify(process.env.PUN_PROBE.split(",").map(probe)) + "\n");
delete process.env.PUN_PROBE;
