# super scuffed codegen script ignore

import json


f = open("opcodes.json")
opcodes = json.load(f)

# Generate Rust match statement
rust_code = "match opcode {\n"
for opcode, data in opcodes["cbprefixed"].items():
    rust_code += f"    {opcode} => {{\n"
    rust_code += f"        // {data['mnemonic']} {' '.join([operand['name'] for operand in data['operands']])}\n"
    if len(data["cycles"]) > 1:
        rust_code += "        // if branch\n"
        rust_code += f"        cycles += {data['cycles'][0]};\n"
        rust_code += "        // if not branch\n"
        rust_code += f"        cycles += {data['cycles'][1]};\n"
    else:
        # rust_code += f"        // {data}\n"
        rust_code += f"        cycles += {data['cycles'][0]};\n"
    rust_code += "    }\n"
rust_code += "    _ => {\n"
rust_code += '        println!("Unknown opcode: {:#04X}", opcode);\n'
rust_code += "    }\n"
rust_code += "}"

print(rust_code)
