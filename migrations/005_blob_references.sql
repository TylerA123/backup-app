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
  USING (true);  -- Blobs are content-addressed, no ownership needed

CREATE POLICY "Users can insert blob references"
  ON public.blob_references FOR INSERT
  WITH CHECK (true);

CREATE POLICY "Users can update blob references"
  ON public.blob_references FOR UPDATE
  USING (true);

CREATE INDEX idx_blob_references_hash
  ON public.blob_references(content_hash);
