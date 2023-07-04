import { Panel } from "react-resizable-panels";

export default function OutputPanel({
  name,
  content,
}: {
  name: string;
  content: string;
}) {
  return (
    <Panel>
      <div className="panel-header">{name}</div>
      <div className="output">
        <pre className="output-text">{content}</pre>
      </div>
    </Panel>
  );
}
