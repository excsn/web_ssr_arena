import Link from "next/link";

export default function NotFound() {
  return (
    <section className="hero">
      <h1>No page here</h1>
      <Link href="/">Back to the start</Link>
    </section>
  );
}
