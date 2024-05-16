import { cpu, mem, drive, os, oscmd, wrapExec, exec } from 'node-os-utils';
import si from 'systeminformation';

export async function GET() {
    const cpuUsage = await cpu.usage();
    const memUsage = await mem.info();
    //    const driveInfo = await drive.info("sdb1");
    si.diskLayout().then(data => {
        console.log("diskLayout\n\n");
        console.log(data);
    }).catch(error => console.error(error));

    si.inetLatency().then(data => {
        console.log("inetLatency\n\n");
        console.log(data);
    }).catch(error => console.error(error));


    //list all drives
    si.networkStats().then(data => {
        console.log("networkStats\n\n");
        console.log(data);
    }).catch(error => console.error(error));

    // const osCmd = await oscmd.run('ls');
    //list all the drives

    /*    si.networkConnections().then(data => {
            console.log("networkConnections\n\n");
            console.log(data);
        }).catch(error => console.error(error));
    */

    si.disksIO().then(data => {
        console.log("diskIO\n\n");
        console.log(data);
    }).catch(error => console.error(error));

    return Response.json({
        "server info": {
            cpu: cpuUsage,
            mem: memUsage,
            hostname: os.hostname(),
            ip: os.ip(),
            thing: os.uptime(),
            //driveInfo,
        }
    });
}
