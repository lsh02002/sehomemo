export type FolderType = {
  id: number;
  name: string;
  parent_id: number;
  sort_order: number;
  created_at: Date;
  updated_at: Date;
};

export type NoteType = {
  id: number;
  title: string;
  folder_id: number;
  folder_name: string;
  content: string;
  created_at: string;
  updated_at: string;
};
