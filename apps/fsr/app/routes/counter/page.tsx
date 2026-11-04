import Counter from "@src/ui/Counter";

export default function CounterPage() {
  return (
    <section>
      <h1>Counter</h1>
      <p>One component that runs in the browser.</p>
      <Counter start={3} />
    </section>
  );
}
