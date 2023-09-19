/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    domains: [
      "avatars.githubusercontent.com",
      "avatar.vercel.sh",
      "arrow.apache.org",
    ],
  },
  experimental: {
    serverComponentsExternalPackages: ["@tremor/react"],
  },
};

module.exports = nextConfig;
