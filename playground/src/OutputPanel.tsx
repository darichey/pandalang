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
      <div className="border-b p-1">{name}</div>
      <div className="w-full h-full overflow-auto">
        <pre className="p-1">{content}</pre>
      </div>
    </Panel>
  );
}
