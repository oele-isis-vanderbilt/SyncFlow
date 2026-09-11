const { withContentlayer } = require('next-contentlayer2');

/** @type {import('next').NextConfig} */
const nextConfig = {
  typescript: {
    ignoreBuildErrors: true,
  },
  output: 'standalone',
  transpilePackages: ['@utimalsina/ts-monads'],
  async redirects() {
    return [
      {
        source: '/docs',
        destination: '/docs/introduction/introduction',
        permanent: true,
      },
    ];
  },
};

module.exports = withContentlayer(nextConfig);
