import AdmZip from "adm-zip"
import fs from 'fs'
import path from 'path'

let outputLocation = process.argv[2];
let input = process.argv.slice(3);
let paths: string[] = [];

let handlePath = (e: string) => {
    let isDir = e.startsWith("@root/") ? false : fs.lstatSync(e).isDirectory(); 

    if (!isDir) {
        paths.push(e);
    } else {
        let content = fs.readdirSync(e, { withFileTypes: true });

        content.forEach((child) => {
            if (child.isDirectory()) {
                handlePath(path.join(e, child.name));
            } else {
                paths.push(path.join(e, child.name));
            }
        });

    }
};

input.forEach((e)=> {
    handlePath(e);
});

let zip = new AdmZip();

paths.forEach((p) => {
    let new_path = p;

    if (p.startsWith("@root/")) {
        new_path = p.replace("@root/", "").split('/').pop()!
    }

    zip.addFile(new_path.replace(`target${path.sep}release${path.sep}`, ""), fs.readFileSync(p.replace("@root/", "")));
});

zip.writeZip(outputLocation);
