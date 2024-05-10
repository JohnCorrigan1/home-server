export async function GET() {

    const fs = require('fs');
    const path = require('path');
    const filePath = "/home/chad/stuff";
    //await fs.writeFileSync(path.join(filePath, 'hello.txt'), 'Hello World!');
    await fs.appendFileSync(path.join(filePath, 'hello.txt'), 'Hello World!');
    console.log('File written to: ' + path.join(filePath, 'hello.txt'));




    return Response.json({ message: 'Hello World!' });
}


