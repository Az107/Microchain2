"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const _microchain = require('./index.node');
class Microchain {
    constructor(name) {
        this.microchain = _microchain.new(name);
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
exports.default = Microchain;
