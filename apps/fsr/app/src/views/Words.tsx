import type { Words } from "@src/compute";

export default function WordsView({ result }: { result: Words }) {
  return (
    <section>
      <h1>Word counts</h1>
      <p className="digest">{`words ${result.total} ${result.distinct} ${result.top.map((w) => `${w.word}:${w.count}`).join(",")}`}</p>
      <ol>
        {result.top.map((w) => (
          <li key={w.word}>
            {w.word} {w.count}
          </li>
        ))}
      </ol>
    </section>
  );
}
