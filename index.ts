import {createRequire} from "module";
const require = createRequire(import.meta.url);


const _microchain = require('./index.node');

type Block = {
    id: number,
    data: Array<string>,
    previousHash: string
}

export class Microchain  {
        private microchain: any;
        public name;
        constructor(name: string,fileName?: string) {
            if (fileName) {
                this.microchain = _microchain.loadFile(fileName);
            } else {
                this.microchain = _microchain.new(name);
            }
            this.name = name;
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

        getBlock(index: number): Block {
            return _microchain.getBlock(this.microchain, index);
        }
        

        getLength(): number {
            return _microchain.getLength(this.microchain);
        }
    
        saveBlock() {
            _microchain.saveBlock(this.microchain);
        }
    
        saveAsFile(fileName: string) {
            _microchain.saveAsFile(this.microchain, fileName);
        }
    
}

