import type { Metadata } from "next";
import Link from "next/link";
import "./app.css";

export const metadata: Metadata = { title: "Outfitters" };

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <div className="shell">
          <header className="bar">
            <Link href="/" className="brand">
              Outfitters
            </Link>
            <nav>
              <Link href="/products">Products</Link>
              <Link href="/search?q=pack">Search</Link>
              <Link href="/counter">Counter</Link>
            </nav>
          </header>
          <main className="content">{children}</main>
          <footer className="foot">Outfitters, a benchmark storefront.</footer>
        </div>
      </body>
    </html>
  );
}
