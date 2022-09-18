import { parse } from 'acorn';
import { simple } from 'acorn-walk';

const program = parse('let x = 10;', {
    ecmaVersion: 2018,
    sourceType: 'module'
});

for (const node of program['body']) {
    
}

console.log(program);