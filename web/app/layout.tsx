import './globals.css';

import { Analytics } from '@vercel/analytics/react';
import { Suspense } from 'react';
import Navbar from './navbar';

export const metadata = {
  title: 'RSQuery',
  description:
    'The new big data analytics engine implemented by Rust.'
};

export default async function RootLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <html data-color-mode="light" lang="en" className="h-full bg-gray-50">
      <body className="h-full">
        <Suspense>
          <Navbar />
        </Suspense>
        {children}
        <Analytics />
      </body>
    </html>
  );
}