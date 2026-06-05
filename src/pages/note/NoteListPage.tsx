import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import { NoteType } from "../../types/type";

export default function NoteListPage() {
  const navigate = useNavigate();

  const [keyword, setKeyword] = useState("");
  const [notes, setNotes] = useState<NoteType[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchNotes = async () => {
    try {
      setLoading(true);

      const trimmedKeyword = keyword.trim();

      const result =
        trimmedKeyword === ""
          ? await invoke<NoteType[]>("get_notes")
          : await invoke<NoteType[]>("get_notes_by_keyword", {
              keyword: trimmedKeyword,
            });

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
      await invoke("delete_note_softly", { id });
      await fetchNotes();
    } catch (error) {
      console.error(error);
      alert("메모 삭제 실패");
    }
  };

  useEffect(() => {
    fetchNotes();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [keyword]);

  const regex = useMemo(() => {
    if (!keyword) return null;

    const escaped = keyword.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

    return new RegExp(`(${escaped})`, "gi");
  }, [keyword]);

  const highlightText = (text: string) => {
    if (!regex) return text;

    return text
      .split(regex)
      .map((part, index) =>
        regex.test(part) ? <mark key={index}>{part}</mark> : part,
      );
  };

  return (
    <div className="min-vh-100 bg-dark text-white d-flex flex-column">
      <header className="d-flex align-items-center justify-content-between border-bottom border-secondary px-4 py-3">
        <h1 className="h4 fw-bold mb-0">메모 목록</h1>

        <div className="d-flex gap-2">
          <button
            onClick={() => navigate("/create")}
            className="btn btn-primary"
          >
            새 메모
          </button>

          <button
            onClick={() => navigate("/folder/create")}
            className="btn btn-primary"
          >
            새 폴더
          </button>

          <button
            onClick={() => navigate("/trash")}
            className="btn btn-primary"
          >
            휴지통
          </button>
        </div>
      </header>

      <main className="flex-grow-1 overflow-auto p-4">
        <div className="mb-4">
          <input
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            className="form-control bg-black text-white border-secondary"
            placeholder="제목, 내용, 폴더명으로 검색"
          />
        </div>

        {loading ? (
          <p className="text-secondary">불러오는 중...</p>
        ) : notes.length === 0 ? (
          <div className="d-flex h-100 align-items-center justify-content-center text-secondary">
            {keyword.trim() === ""
              ? "아직 작성된 메모가 없습니다."
              : "검색 결과가 없습니다."}
          </div>
        ) : (
          <div className="row g-3">
            {notes.map((note) => (
              <div key={note.id} className="col-12">
                <div className="card bg-black text-white border-secondary">
                  <div className="card-body">
                    <div className="d-flex align-items-start justify-content-between gap-3">
                      <button
                        onClick={() => navigate(`/update/${note.id}`)}
                        className="btn text-start text-white flex-grow-1 p-0 border-0"
                      >
                        <h2 className="h5 fw-semibold text-truncate mb-2">
                          {highlightText(note.title || "제목 없음")}
                        </h2>

                        <p className="text-secondary small mb-2">
                          {highlightText(note.folder_name || "폴더 없음")}
                        </p>

                        <p className="text-secondary small mb-2">
                          {highlightText(note.content || "내용 없음")}
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
