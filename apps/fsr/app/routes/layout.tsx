import type { Children } from "@snapfire/fsr/template";
import { Link } from "@snapfire/fsr/components";

export default function Layout({ children }: { children: Children }) {
  return (
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
  );
}
