const Microchain = require(".");
//import fs
const fs = require("fs");

//make a jest test to index.js

test('Test constructor',()=>{
    const microchain = new Microchain("");
    expect(microchain).toBeDefined();
});

test('Test file creation',()=>{
    const microchain = new Microchain("");
    microchain.addData("test");
    microchain.saveBlock();
    microchain.addData("test");
    microchain.saveBlock();
    microchain.saveAsFile("test.json");
    expect(fs.existsSync("test.json")).toBeTruthy();
});



