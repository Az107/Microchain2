const Microchain = require(".");

//make a jest test to index.js

test('dummy test',()=>{
    expect(true).toBeTruthy();
});

test('Test constructor',()=>{
    const microchain = new Microchain("");
    microchain.addData("test");
    expect(microchain).toBeDefined();
});