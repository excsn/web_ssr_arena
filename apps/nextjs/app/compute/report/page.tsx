import { report } from "@src/compute";
import ReportView from "@src/views/Report";

export const dynamic = "force-dynamic";

export default async function Page({ searchParams }: { searchParams: Promise<{ orders?: string }> }) {
  const orders = Math.min(Math.max(Number((await searchParams).orders ?? "5000") || 0, 1), 200000);
  return <ReportView result={report(orders)} />;
}
