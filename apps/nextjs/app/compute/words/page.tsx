import { words } from "@src/compute";
import WordsView from "@src/views/Words";

export const dynamic = "force-dynamic";

export default async function Page({ searchParams }: { searchParams: Promise<{ passes?: string }> }) {
  const passes = Math.min(Math.max(Number((await searchParams).passes ?? "20") || 0, 1), 1000);
  return <WordsView result={words(passes)} />;
}
