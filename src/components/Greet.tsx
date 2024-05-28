"use client";
import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';

export default function Greet() {

 const [greeting, setGreeting] = useState('');

  useEffect(() => {
    invoke<string>('greet', { name: 'Next.js' })
      .then(result => setGreeting(result))
      .catch(console.error)
  }, [])
    return (

<div>
    <h1 className="text-3xl text-sky-500 font-bold" >{greeting}</h1>
</div>
    );
}

