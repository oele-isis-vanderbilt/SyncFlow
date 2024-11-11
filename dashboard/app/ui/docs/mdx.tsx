import { Alert, theme } from 'flowbite-react';
import type { MDXComponents } from 'mdx/types';
import { getMDXComponent } from 'next-contentlayer2/hooks';
import Link from 'next/link';

const components: MDXComponents = {
  Alert,
  a: ({ ref, href = '', ...props }) => {
    const isLocal = href.startsWith('/');

    return (
      <Link
        {...props}
        href={href}
        ref={ref as React.Ref<HTMLAnchorElement>}
        {...(!isLocal && { target: '_blank', rel: 'noreferrer' })}
      />
    );
  },
  // TODO: revisit
  h2: (props) => (
    <h2
      className="group relative z-20 scroll-mt-20 font-bold text-2xl text-gray-900 dark:text-white"
      {...props}
    >
      {props.children}
      <a
        href={`#${props.id}`}
        aria-label={`Link to this section: ${props.children}`}
        className="ml-2 text-gray-900 opacity-0 transition-opacity group-hover:opacity-100 dark:text-white"
      >
        #
      </a>
    </h2>
  ),
  // TODO: revisit
  h3: (props) => (
    <h3
      className="group relative z-10 scroll-mt-20 font-bold text-2xl text-gray-900 dark:text-white"
      {...props}
    >
      {props.children}
      <a
        href={`#${props.id}`}
        aria-label={`Link to this section: ${props.children}`}
        className="ml-2 text-gray-900 opacity-0 transition-opacity group-hover:opacity-100 dark:text-white"
      >
        #
      </a>
    </h3>
  ),
};

export function Mdx({ code }: { code: string }) {
  const Component = getMDXComponent(code);

  return <Component components={components} />;
}
