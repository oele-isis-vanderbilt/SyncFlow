/** @type {import('next').NextConfig} */
const nextConfig = {
  typescript: {
    ignoreBuildErrors: true,
  },
  output: 'standalone',
  transpilePackages: ['@utimalsina/ts-monads'],
};

export default nextConfig;
