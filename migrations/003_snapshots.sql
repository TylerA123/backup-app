CREATE TABLE IF NOT EXISTS public.snapshots (
  id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id          UUID NOT NULL REFERENCES public.projects(id),
  parent_snapshot_id  UUID REFERENCES public.snapshots(id),
  fingerprint         TEXT NOT NULL,
  file_count          INTEGER NOT NULL,
  total_size          BIGINT NOT NULL,
  changes_added       INTEGER DEFAULT 0,
  changes_modified    INTEGER DEFAULT 0,
  changes_deleted     INTEGER DEFAULT 0,
  trigger             TEXT DEFAULT 'auto',
  notes               TEXT,
  created_at          TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE public.snapshots ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own snapshots"
  ON public.snapshots FOR SELECT
  USING (
    project_id IN (
      SELECT id FROM public.projects WHERE user_id = auth.uid()
    )
  );

CREATE POLICY "Users can create snapshots"
  ON public.snapshots FOR INSERT
  WITH CHECK (
    project_id IN (
      SELECT id FROM public.projects WHERE user_id = auth.uid()
    )
  );

CREATE INDEX idx_snapshots_project_time
  ON public.snapshots(project_id, created_at DESC);
