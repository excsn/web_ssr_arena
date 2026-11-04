import { useState } from "react";

export default function Counter({ start }: { start: number }) {
  const [count, setCount] = useState(start);
  return (
    <div className="counter">
      <button type="button" onClick={() => setCount(count - 1)}>
        -
      </button>
      <output>{count}</output>
      <button type="button" onClick={() => setCount(count + 1)}>
        +
      </button>
    </div>
  );
}
