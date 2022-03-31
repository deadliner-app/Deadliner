import JSZip from "jszip"
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

let zip = new JSZip();

paths.forEach((p) => {
    let path = p;

    if (p.startsWith("@root/")) {
        path = p.replace("@root/", "").split('/').pop()!
    }

    zip.file(path, fs.readFileSync(p.replace("@root/", "")));
});

zip
.generateNodeStream({ type: 'nodebuffer', streamFiles:true })
.pipe(fs.createWriteStream(outputLocation))
.on('finish', () => {
    // JSZip generates a readable stream with a "end" event,
    // but is piped here in a writable stream which emits a "finish" event.
    console.log(`${outputLocation} written.`);
});