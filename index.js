const _microchain = require('./index.node');

class Microchain {
    constructor(name) {
        this.microchain = _microchain.new(name);
    }

    addData(data) {
        _microchain.addString(this.microchain, data);
    }

    saveBlock() {
        _microchain.saveBlock(this.microchain);
    }

    saveAsFile(fileName) {
        _microchain.saveAsFile(this.microchain, fileName);
    }

    
}

//exports microchain
module.exports = Microchain;