import { createRequire } from "module";
const require = createRequire(import.meta.url);
const _microchain = require('./index.node');
export class Microchain {
    constructor(name, fileName) {
        if (fileName) {
            this.microchain = _microchain.loadFile(fileName);
        }
        else {
            this.microchain = _microchain.new(name);
        }
        this.name = name;
    }
    addData(data) {
        //check data is string
        let dataP;
        if (typeof data !== 'string') {
            dataP = JSON.stringify(data);
        }
        else {
            dataP = data;
        }
        _microchain.addString(this.microchain, dataP);
    }
    getData() {
        return _microchain.getString(this.microchain);
    }
    saveBlock() {
        _microchain.saveBlock(this.microchain);
    }
    saveAsFile(fileName) {
        _microchain.saveAsFile(this.microchain, fileName);
    }
}
