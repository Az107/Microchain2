const Microchain = require('./index').default;
//import fs
const fs = require("fs");

//make a jest test to index.js

test('Test constructor',()=>{
    const microchain = new Microchain('test');
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

test("Test get data",()=>{
    const microchain = new Microchain("");
    microchain.addData("test");
    expect(microchain.getData()[0]).toBe("test");
});

test("Test get data 2",()=>{
    const microchain = new Microchain("");
    microchain.addData("test");
    microchain.addData("test");
    expect(typeof microchain.getData()).not.toBe("string");
    expect(microchain.getData()).not.toBe("testtest");
});

afterAll(()=>{
    fs.unlinkSync("test.json");
});



