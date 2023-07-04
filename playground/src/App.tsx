import { Editor } from "@monaco-editor/react";
import * as wasm from "pandalang-playground";
import { useState } from "react";
import { Panel, PanelGroup } from "react-resizable-panels";
import ResizeHandle from "./ResizeHandle";
import OutputPanel from "./OutputPanel";

export default function App() {
  const [ast, setAst] = useState("");
  const [types, setTypes] = useState("");
  const [output, setOutput] = useState("");

  function onSourceChange(source?: string) {
    setAst("");
    setTypes("");
    setOutput("");

    source = source ?? "";

    try {
      setAst(wasm.parse(source));
    } catch (e) {
      setAst(e as string);
      return;
    }

    try {
      setTypes(wasm.typecheck(source));
    } catch (e) {
      setTypes(e as string);
      return;
    }

    try {
      setOutput(wasm.run(source));
    } catch (e) {
      setOutput(e as string);
      return;
    }
  }

  return (
    <>
      <div className="text-2xl px-2 py-4 border-b-2">PandaLang Playground</div>
      <PanelGroup direction="horizontal">
        <Panel>
          <Editor
            options={{
              minimap: {
                enabled: false,
              },
            }}
            onChange={onSourceChange}
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
    </>
  );
}
