import { Editor } from "@monaco-editor/react";
import { useEffect, useState } from "react";
import { Panel, PanelGroup } from "react-resizable-panels";
import ResizeHandle from "./ResizeHandle";
import OutputPanel from "./OutputPanel";
import { getExample, getExampleNames } from "./examples";
import * as LZString from "lz-string";

// @ts-ignore (TODO: figure out how to use generated typescript types. https://github.com/wasm-tool/rollup-plugin-rust/issues/9)
import * as wasm from "../../crates/playground/Cargo.toml";

export default function App() {
  const [selectedExample, setSelectedExample] = useState(
    window.location.hash ? "none" : "Hello world"
  );
  const [source, setSource] = useState(
    window.location.hash
      ? LZString.decompressFromEncodedURIComponent(
          window.location.hash.slice(1)
        )
      : getExample(selectedExample)
  );
  const [ast, types, output] = run(source);

  useEffect(() => {
    window.location.hash = LZString.compressToEncodedURIComponent(source);
  }, [source]);

  useEffect(() => {
    if (selectedExample !== "none") {
      setSource(getExample(selectedExample));
    }
  }, [selectedExample]);

  return (
    <div className="h-full w-full flex flex-col">
      <div className="text-2xl px-2 py-4 border-b-2">PandaLang Playground</div>
      <PanelGroup direction="horizontal">
        <Panel className="flex flex-col">
          <div className="border-b p-2">
            <label>
              <span>Example: </span>
              <select
                value={selectedExample}
                onChange={(event) => {
                  setSelectedExample(event.target.value);
                }}
              >
                <option value="none" disabled hidden>
                  Select an example
                </option>
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
            onChange={(source) => {
              setSource(source ?? "");
              setSelectedExample("none");
            }}
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
