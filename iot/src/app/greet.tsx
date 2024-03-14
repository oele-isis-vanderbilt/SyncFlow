'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

type GreetProps = {
  msg?: string
}

export default function Greet(props: GreetProps) {
  const [greeting, setGreeting] = useState('');

  useEffect(() => {
    invoke<string>('greet', { name: props.msg || 'Next.js' })
      .then(result => setGreeting(result))
      .catch(console.error)
  }, [props.msg])

  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}