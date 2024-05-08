
export async function GET() {

    const fs = require('fs');
    const path = require('path');
//    const filePath = path.join(__dirname, 'data.json');
    const filePath = "/home/chad/stuff";
    await fs.writeFileSync(path.join(filePath, 'hello.txt'), 'Hello World!');
    console.log('File written to: ' + path.join(filePath, 'hello.txt'));

    //return jsonData;

    //return { message: 'Hello World!' };
    return Response.json({ message: 'Hello World!' });
}
