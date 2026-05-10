CREATE TABLE IF NOT EXISTS public.file_records (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id      UUID NOT NULL REFERENCES public.projects(id),
  snapshot_id     UUID NOT NULL REFERENCES public.snapshots(id) ON DELETE CASCADE,
  relative_path   TEXT NOT NULL,
  file_size       BIGINT NOT NULL,
  content_hash    TEXT NOT NULL,
  last_modified   TIMESTAMPTZ,
  is_deleted      BOOLEAN DEFAULT false
);

ALTER TABLE public.file_records ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own file records"
  ON public.file_records FOR SELECT
  USING (
    project_id IN (
      SELECT id FROM public.projects WHERE user_id = auth.uid()
    )
  );

CREATE POLICY "Users can create file records"
  ON public.file_records FOR INSERT
  WITH CHECK (
    project_id IN (
      SELECT id FROM public.projects WHERE user_id = auth.uid()
    )
  );

CREATE INDEX idx_file_records_snapshot
  ON public.file_records(snapshot_id);

CREATE INDEX idx_file_records_hash
  ON public.file_records(content_hash);
