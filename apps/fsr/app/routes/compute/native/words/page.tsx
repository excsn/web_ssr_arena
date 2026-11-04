import type { Words } from "@src/compute";
import WordsView from "@src/views/Words";

export default function Page({ result }: { result: Words }) {
  return <WordsView result={result} />;
}
