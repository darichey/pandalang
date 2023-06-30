import * as wasm from "pandalang-parser";
import { useState } from "react";

export default function App() {
  const [output, setOutput] = useState("");

  function onInputChange(value: string) {
    try {
      setOutput(wasm.parse(value));
    } catch (e) {
      setOutput(JSON.stringify(e));
    }
  }

  return (
    <>
      <textarea
        className="input"
        onChange={(event) => onInputChange(event.target.value)}
      ></textarea>
      <div className="output">{output}</div>
    </>
  );
}
