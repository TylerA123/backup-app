-- Run this entire file in your Supabase SQL Editor
-- Supabase: https://supabase.com/dashboard/project/oewsafpfhkteofylqkab/sql/new

-- 001: Profiles
CREATE TABLE IF NOT EXISTS public.profiles (
  id            UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
  email         TEXT,
  display_name  TEXT,
  created_at    TIMESTAMPTZ DEFAULT now(),
  updated_at    TIMESTAMPTZ DEFAULT now()
);
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can view own profile"
  ON public.profiles FOR SELECT
  USING (auth.uid() = id);
CREATE POLICY "Users can update own profile"
  ON public.profiles FOR UPDATE
  USING (auth.uid() = id);
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO public.profiles (id, email, display_name)
  VALUES (new.id, new.email, split_part(new.email, '@', 1));
  RETURN new;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
CREATE OR REPLACE TRIGGER on_auth_user_created
  AFTER INSERT ON auth.users
  FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- 002: Projects
CREATE TABLE IF NOT EXISTS public.projects (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id       UUID NOT NULL REFERENCES public.profiles(id),
  name          TEXT NOT NULL,
  local_path    TEXT,
  total_size    BIGINT DEFAULT 0,
  file_count    INTEGER DEFAULT 0,
  is_deleted    BOOLEAN DEFAULT false,
  created_at    TIMESTAMPTZ DEFAULT now(),
  updated_at    TIMESTAMPTZ DEFAULT now()
);
ALTER TABLE public.projects ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can view own projects"
  ON public.projects FOR SELECT
  USING (auth.uid() = user_id AND NOT is_deleted);
CREATE POLICY "Users can create projects"
  ON public.projects FOR INSERT
  WITH CHECK (auth.uid() = user_id);
CREATE POLICY "Users can update own projects"
  ON public.projects FOR UPDATE
  USING (auth.uid() = user_id);
CREATE INDEX IF NOT EXISTS idx_projects_user_visible
  ON public.projects(user_id)
  WHERE NOT is_deleted;

-- 003: Snapshots
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
  USING (project_id IN (SELECT id FROM public.projects WHERE user_id = auth.uid()));
CREATE POLICY "Users can create snapshots"
  ON public.snapshots FOR INSERT
  WITH CHECK (project_id IN (SELECT id FROM public.projects WHERE user_id = auth.uid()));
CREATE INDEX IF NOT EXISTS idx_snapshots_project_time
  ON public.snapshots(project_id, created_at DESC);

-- 004: File Records
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
  USING (project_id IN (SELECT id FROM public.projects WHERE user_id = auth.uid()));
CREATE POLICY "Users can create file records"
  ON public.file_records FOR INSERT
  WITH CHECK (project_id IN (SELECT id FROM public.projects WHERE user_id = auth.uid()));
CREATE INDEX IF NOT EXISTS idx_file_records_snapshot
  ON public.file_records(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_file_records_hash
  ON public.file_records(content_hash);

-- 005: Blob References
CREATE TABLE IF NOT EXISTS public.blob_references (
  id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  content_hash      TEXT NOT NULL UNIQUE,
  storage_key       TEXT NOT NULL,
  file_size         BIGINT NOT NULL,
  checksum          TEXT NOT NULL,
  reference_count   INTEGER DEFAULT 1,
  upload_status     TEXT DEFAULT 'pending',
  created_at        TIMESTAMPTZ DEFAULT now(),
  updated_at        TIMESTAMPTZ DEFAULT now()
);
ALTER TABLE public.blob_references ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can view blob references"
  ON public.blob_references FOR SELECT
  USING (true);
CREATE POLICY "Users can insert blob references"
  ON public.blob_references FOR INSERT
  WITH CHECK (true);
CREATE POLICY "Users can update blob references"
  ON public.blob_references FOR UPDATE
  USING (true);
CREATE INDEX IF NOT EXISTS idx_blob_references_hash
  ON public.blob_references(content_hash);
