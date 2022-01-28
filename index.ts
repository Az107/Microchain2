const _microchain = require('./index.node');


export default class Microchain  {
        microchain: any;
        constructor(name: string) {
            this.microchain = _microchain.new(name);
        }
    
        addData(data: any) {
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

        getData(): Array<string> {
            return _microchain.getString(this.microchain) as Array<string>;
        }
    
        saveBlock() {
            _microchain.saveBlock(this.microchain);
        }
    
        saveAsFile(fileName: string) {
            _microchain.saveAsFile(this.microchain, fileName);
        }
    
}

