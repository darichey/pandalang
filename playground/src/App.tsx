import { Editor } from "@monaco-editor/react";
import * as wasm from "pandalang-playground";
import { useState } from "react";
import { Panel, PanelGroup } from "react-resizable-panels";
import ResizeHandle from "./ResizeHandle";
import OutputPanel from "./OutputPanel";
import { getExample, getExampleNames } from "./examples";

export default function App() {
  const [source, setSource] = useState(getExample("Hello world"));
  const [ast, types, output] = run(source);

  return (
    <div className="h-full w-full flex flex-col">
      <div className="text-2xl px-2 py-4 border-b-2">PandaLang Playground</div>
      <PanelGroup direction="horizontal">
        <Panel className="flex flex-col">
          <div className="border-b p-2">
            <label>
              <span>Example: </span>
              <select
                onChange={(event) => {
                  setSource(getExample(event.target.value));
                }}
              >
                {getExampleNames().map((key) => (
                  <option key={key} value={key}>
                    {key}
                  </option>
                ))}
              </select>
            </label>
          </div>
          <Editor
            options={{
              minimap: {
                enabled: false,
              },
              scrollBeyondLastLine: false,
            }}
            value={source}
            onChange={(source) => setSource(source ?? "")}
            language="ml"
          />
        </Panel>
        <ResizeHandle />
        <Panel>
          <PanelGroup direction="vertical">
            <OutputPanel name="AST" content={ast} />
            <ResizeHandle />
            <OutputPanel name="Types" content={types} />
            <ResizeHandle />
            <OutputPanel name="Output" content={output} />
          </PanelGroup>
        </Panel>
      </PanelGroup>
    </div>
  );
}

function run(source: string) {
  let ast = "";
  let types = "";
  let output = "";

  try {
    ast = wasm.parse(source);
  } catch (e) {
    ast = e as string;
    return [ast, types, output];
  }

  try {
    types = wasm.typecheck(source);
  } catch (e) {
    types = e as string;
    return [ast, types, output];
  }

  try {
    output = wasm.run(source);
  } catch (e) {
    output = e as string;
    return [ast, types, output];
  }

  return [ast, types, output];
}
