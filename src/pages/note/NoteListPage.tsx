import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Note = {
  id: number;
  title: string;
  content: string;
  created_at: string;
  updated_at: string;
};

export default function NoteListPage() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchNotes = async () => {
    try {
      setLoading(true);

      const result = await invoke<Note[]>("get_notes");
      setNotes(result);
    } catch (error) {
      console.error(error);
      alert("메모 목록을 불러오지 못했습니다.");
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: number) => {
    const ok = window.confirm("이 메모를 삭제할까요?");
    if (!ok) return;

    try {
      await invoke("delete_note", { id });
      await fetchNotes();
    } catch (error) {
      console.error(error);
      alert("메모 삭제 실패");
    }
  };

  useEffect(() => {
    fetchNotes();
  }, []);

  return (
    <div className="min-vh-100 bg-dark text-white d-flex flex-column">
      <header className="d-flex align-items-center justify-content-between border-bottom border-secondary px-4 py-3">
        <h1 className="h4 fw-bold mb-0">메모 목록</h1>

        <button
          onClick={() => {
            window.location.href = "/create";
          }}
          className="btn btn-primary"
        >
          새 메모
        </button>
      </header>

      <main className="flex-grow-1 overflow-auto p-4">
        {loading ? (
          <p className="text-secondary">불러오는 중...</p>
        ) : notes.length === 0 ? (
          <div className="d-flex h-100 align-items-center justify-content-center text-secondary">
            아직 작성된 메모가 없습니다.
          </div>
        ) : (
          <div className="row g-3">
            {notes.map((note) => (
              <div key={note.id} className="col-12">
                <div className="card bg-black text-white border-secondary">
                  <div className="card-body">
                    <div className="d-flex align-items-start justify-content-between gap-3">
                      <button
                        onClick={() => {
                          window.location.href = `/notes/${note.id}`;
                        }}
                        className="btn text-start text-white flex-grow-1 p-0 border-0"
                      >
                        <h2 className="h5 fw-semibold text-truncate mb-2">
                          {note.title || "제목 없음"}
                        </h2>

                        <p className="text-secondary small mb-2">
                          {note.content || "내용 없음"}
                        </p>

                        <p className="text-secondary small mb-0">
                          수정일: {new Date(note.updated_at).toLocaleString()}
                        </p>
                      </button>

                      <button
                        onClick={() => handleDelete(note.id)}
                        className="btn btn-outline-danger btn-sm"
                      >
                        삭제
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
