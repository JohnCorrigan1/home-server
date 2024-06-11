"use client";

import { CircularProgress } from "@nextui-org/react";
import { useEffect, useState } from "react";
import baseUrl from "./BaseUrl";

export default function Status({
  value,
  label,
  label2 = "",
  label3 = "",
}: StatusProps) {
  return (
    <div className="flex flex-col gap-5 items-center">
      <CircularProgress
        aria-label={label}
        value={value}
        size="lg"
        classNames={{
          svg: "w-24 h-24",
          value: "text-lg font-semibold",
        }}
        showValueLabel={true}
        label={label}
        color={
          value < 50
            ? "success"
            : value < 80
            ? "warning"
            : "danger"
        }
      />
      {label2 != "" ? <p className="">{label2}</p> : null}
      {label3 != "" ? <p className="">{label3}</p> : null}
    </div>
  );
}

export type StatusProps = {
  value: number;
  label: string;
  label2?: string;
  label3?: string;
};

export function CPUStatus() {
  const [value, setValue] = useState<number>(0);
  const [refresh, setRefresh] = useState<number>(0);

  useEffect(() => {
    const callCpu = async () => {
      const cpu = await fetch(`${baseUrl}/api/cpu`).then(
        (res) => res.json()
      );
      setValue(cpu.cpu_usage);
    };
    callCpu();
  }, [refresh]);

  return (
    <div
      className=" cursor-pointer"
      onClick={() => setRefresh(refresh + 1)}>
      <Status value={value} label={"CPU"} />
    </div>
  );
}

type Memory = {
  total_memory: number;
  used_memory: number;
  memory_usage: number;
};

export function MemoryStatus() {
  const [value, setValue] = useState<Memory>();
  const [refresh, setRefresh] = useState<number>(0);

  useEffect(() => {
    const callMemory = async () => {
      const memory = await fetch(
        `${baseUrl}/api/memory`
      ).then((res) => res.json());
      setValue(memory);
    };
    callMemory();
  }, [refresh]);

  return (
    <div
      className=" cursor-pointer"
      onClick={() => setRefresh(refresh + 1)}>
      {value ? (
        <Status
          value={value.memory_usage}
          label={"Memory"}
          label2={`${value.used_memory.toFixed(
            2
          )}GB/${value.total_memory.toFixed(2)}GB`}
        />
      ) : null}
    </div>
  );
}

type Disk = {
  name: string;
  disk_type: string;
  total: number;
  available: number;
  usage: number;
  mount_point: string;
  file_system: string;
};

export function DiskStatus() {
  const [value, setValue] = useState<Disk[]>();
  const [refresh, setRefresh] = useState<number>(0);

  useEffect(() => {
    const callDisk = async () => {
      const disk: Disk[] = await fetch(
        `${baseUrl}/api/storage`
      ).then((res) => res.json());
      setValue(disk);
    };
    callDisk();
  }, [refresh]);

  return (
    <>
      {value?.map((disk, index) => (
        <div
          key={index}
          className=" cursor-pointer"
          onClick={() => setRefresh(refresh + 1)}>
          <Status
            value={disk.usage}
            label={disk.name}
            label2={disk.disk_type}
            label3={`${(
              disk.total - disk.available
            ).toFixed(0)}GB/${disk.total.toFixed(0)}GB`}
          />
        </div>
      ))}
    </>
  );
}
