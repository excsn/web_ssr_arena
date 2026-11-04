import type { Report } from "@src/compute";
import ReportView from "@src/views/Report";

export default function Page({ result }: { result: Report }) {
  return <ReportView result={result} />;
}
