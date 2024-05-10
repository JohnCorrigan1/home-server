import { cpu, mem, drive, os, oscmd, wrapExec, exec } from 'node-os-utils';
import si from 'systeminformation';

export async function GET() {
    const cpuUsage = await cpu.usage();
    const memUsage = await mem.info();
    const driveInfo = await drive.info("sdb1");

    // const osCmd = await oscmd.run('ls');
    //list all the drives






    return Response.json({
        "server info": {
            cpu: cpuUsage,
            mem: memUsage,
            hostname: os.hostname(),
            ip: os.ip(),
            thing: os.uptime(),
            driveInfo,
        }
    });
}
