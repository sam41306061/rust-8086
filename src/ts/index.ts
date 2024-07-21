import { promises as fs } from "fs";
const REG_MEM_00_01_10: String[]= [
    "bx + si",
    "bx + di",
    "bp + si",
    "bp + di",
    "si",
    "di",
    "bp",
    "bx",
]
const REG_MEM_11: Array<[string, string]> =   [
    ["al", "ax"],
    ["cl", "cx"],
    ["dl", "dx"],
    ["bl", "bx"],
    ["ah", "sp"],
    ["ch", "bp"],
    ["dh", "si"],
    ["bh", "di"],
];

const MOV_REG_MEM_TO_FRO_MEM: number = (0b00100010, 2);
const MOV_IMM_TO_REG: number = (0b00001011, 4);



async function  main() {
let args = process.argv.slice(2)
if(args.length === 0) {
    throw new Error('No file has been specified');
}
try {
    //@ts-ignore
    const dataBuffer  = await fs.readFile(args[0]);
    //@ts-ignore
    const loadBytes: number[] = Array.from(new Uint8Array(dataBuffer));

    console.log(`Loaded ${loadBytes.length}`);

} catch(err) {
    console.error(`You fucked up${err}`);
}
console.log(REG_MEM_00_01_10);
console.log(REG_MEM_11);
console.log(MOV_REG_MEM_TO_FRO_MEM);
console.log(MOV_IMM_TO_REG);

}
main().catch(console.error);
