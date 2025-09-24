pub mod dashboard;

mod task_tables;
pub(super) use task_tables::AutoSubmitTaskTables;
pub(super) use task_tables::ConciseTaskTables;
pub(super) use task_tables::ProverTaskTables;

mod task_summary;
pub(super) use task_summary::TaskSummary;

mod stats_summary;
pub(super) use stats_summary::StatsSummary;
