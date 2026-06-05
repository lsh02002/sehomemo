import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import { FolderType, NoteType } from "../../types/type";
import { useZustandStore } from "../../zustand/ZustandStore";

export default function NoteListPage() {
  const navigate = useNavigate();

  const [keyword, setKeyword] = useState("");
  const { selectedFolderId, setSelectedFolderId } = useZustandStore();
  const [folders, setFolders] = useState<FolderType[]>([]);
  const [notes, setNotes] = useState<NoteType[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchFolders = async () => {
    const result = await invoke<FolderType[]>("get_folders");
    setFolders(result);
  };

  const fetchNotes = async () => {
    try {
      setLoading(true);

      const trimmedKeyword = keyword.trim();

      let result =
        trimmedKeyword !== ""
          ? await invoke<NoteType[]>("get_notes_by_keyword", {
              keyword: trimmedKeyword,
            })
          : selectedFolderId !== null
            ? await invoke<NoteType[]>("get_notes_by_folder_id", {
                id: selectedFolderId,
              })
            : await invoke<NoteType[]>("get_notes");

      if (selectedFolderId !== null) {
        result = result.filter((note) => note.folder_id === selectedFolderId);
      }

      setNotes(result);
    } catch (error) {
      console.error(error);
      alert("메모 목록을 불러오지 못했습니다.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFolders();
  }, []);

  useEffect(() => {
    fetchNotes();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [keyword, selectedFolderId]);

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

  const regex = useMemo(() => {
    if (!keyword.trim()) return null;
    const escaped = keyword.trim().replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
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
    <div className="min-vh-100 bg-dark text-white d-flex">
      <aside
        className="border-end border-secondary p-3 bg-black"
        style={{ width: 240, minWidth: 240, maxWidth: 240 }}
      >
        <div className="d-flex align-items-center justify-content-between mb-3">
          <h2 className="mt-5 h6 fw-bold mb-0">폴더</h2>

          <button
            onClick={() => navigate("/folder/create")}
            className="btn btn-outline-primary btn-sm"
          >
            +
          </button>
        </div>

        <div className="d-grid gap-2">
          <button
            onClick={() => setSelectedFolderId(null)}
            className={`btn text-start ${
              selectedFolderId === null
                ? "btn-primary"
                : "btn-outline-secondary"
            }`}
          >
            전체 메모
          </button>

          {folders.map((folder) => (
            <button
              key={folder.id}
              onClick={() => setSelectedFolderId(folder.id)}
              className={`btn text-start ${
                selectedFolderId === folder.id
                  ? "btn-primary"
                  : "btn-outline-secondary"
              }`}
            >
              {folder.name}
            </button>
          ))}
        </div>
      </aside>

      <div className="flex-grow-1 d-flex flex-column">
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
    </div>
  );
}
